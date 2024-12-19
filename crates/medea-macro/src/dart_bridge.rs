//! `#[dart_bridge]` macro implementation.

#[cfg(feature = "dart-codegen")]
use std::{env, fs::File, io::Write as _, path::PathBuf};

use inflector::Inflector as _;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned as _, token};

#[cfg(feature = "dart-codegen")]
use crate::dart_codegen::{DartCodegen, FnRegistrationBuilder};

// TODO: Refactor to get rid of `static mut` in this macro.
/// Expands `#[dart_bridge]` attribute placed on a Rust module declaration.
pub(crate) fn expand(
    args: TokenStream,
    input: TokenStream,
) -> syn::Result<TokenStream> {
    let expander = ModExpander::try_from(syn::parse2::<syn::ItemMod>(input)?)?;

    #[cfg(feature = "dart-codegen")]
    expander.generate_dart_code(&syn::parse2(args)?)?;
    #[cfg(not(feature = "dart-codegen"))]
    drop(args);

    Ok(expander.expand())
}

/// Expander of the `#[dart_bridge]` attribute.
///
/// Expands to a module registering Dart functions and generating glue code
/// to call them.
#[derive(Debug)]
struct ModExpander {
    /// Visibility of the expanded module.
    vis: syn::Visibility,

    /// Identifier of the expanded module.
    ident: syn::Ident,

    /// Attributes placed on the expanded module by user.
    attrs: Vec<syn::Attribute>,

    /// Uses (`use foo::bar::baz`) of the expanded module.
    uses: Vec<syn::ItemUse>,

    /// [`FnExpander`]s of the all the functions this module should contain.
    fn_expanders: Vec<FnExpander>,

    /// Name of the `register_*` extern function, registering all the module
    /// functions.
    register_fn_name: syn::Ident,
}

impl TryFrom<syn::ItemMod> for ModExpander {
    type Error = syn::Error;

    fn try_from(module: syn::ItemMod) -> syn::Result<Self> {
        use self::mod_parser as parser;

        let mod_span = module.span();

        let mut extern_fns: Vec<FnExpander> = Vec::new();
        let mut use_items = Vec::new();
        let register_prefix = &module.ident;
        for item in parser::try_unwrap_mod_content(module.content)? {
            match item {
                syn::Item::ForeignMod(r#mod) => {
                    for i in r#mod.items {
                        extern_fns.push(FnExpander::parse(
                            parser::get_extern_fn(i)?,
                            register_prefix,
                        )?);
                    }
                }
                syn::Item::Use(r#use) => {
                    use_items.push(r#use);
                }
                syn::Item::Const(_)
                | syn::Item::Enum(_)
                | syn::Item::ExternCrate(_)
                | syn::Item::Fn(_)
                | syn::Item::Impl(_)
                | syn::Item::Macro(_)
                | syn::Item::Mod(_)
                | syn::Item::Static(_)
                | syn::Item::Struct(_)
                | syn::Item::Trait(_)
                | syn::Item::TraitAlias(_)
                | syn::Item::Type(_)
                | syn::Item::Union(_)
                | syn::Item::Verbatim(_) => {
                    return Err(syn::Error::new(
                        item.span(),
                        "Module contains unsupported content",
                    ));
                }
                _ => {
                    return Err(syn::Error::new(
                        item.span(),
                        "Module contains unknown content",
                    ));
                }
            }
        }

        if extern_fns.is_empty() {
            return Err(syn::Error::new(
                mod_span,
                "At least one `extern \"C\"` block is required",
            ));
        }

        Ok(Self {
            register_fn_name: format_ident!("register_{}", module.ident),
            vis: module.vis,
            ident: module.ident,
            attrs: module.attrs,
            uses: use_items,
            fn_expanders: extern_fns,
        })
    }
}

impl ModExpander {
    /// Fully expands a module under the `#[dart_bridge]` attribute.
    fn expand(&self) -> TokenStream {
        let (vis, ident) = (&self.vis, &self.ident);
        let attrs = &self.attrs;
        let uses = &self.uses;

        let type_aliases =
            self.fn_expanders.iter().map(FnExpander::gen_fn_type);

        let fn_storages =
            self.fn_expanders.iter().map(FnExpander::gen_fn_storages);

        let register_fn_name = &self.register_fn_name;
        let register_fn_inputs = self
            .fn_expanders
            .iter()
            .map(FnExpander::gen_register_fn_input);
        let register_fn_assigns = self
            .fn_expanders
            .iter()
            .map(FnExpander::gen_register_fn_expr);

        let caller_fns =
            self.fn_expanders.iter().map(FnExpander::gen_caller_fn);

        let errors_slots =
            self.fn_expanders.iter().map(FnExpander::get_errors_slot);
        let errors_setters =
            self.fn_expanders.iter().map(FnExpander::gen_error_setter);

        quote! {
            #[automatically_derived]
            #( #attrs )*
            #vis mod #ident {
                #( #uses )*

                #( #type_aliases )*

                #( #fn_storages )*

                #( #errors_slots )*

                #[no_mangle]
                pub unsafe extern "C" fn #register_fn_name(
                    #( #register_fn_inputs, )*
                ) {
                    #( #register_fn_assigns; )*
                }

                #( #errors_setters )*

                #( #caller_fns )*
            }
        }
    }

    #[cfg(feature = "dart-codegen")]
    /// Generates all the required Dart code at the provided `relative_path`.
    fn generate_dart_code(
        &self,
        relative_path: &syn::ExprLit,
    ) -> syn::Result<()> {
        let root_path = env::var("CARGO_MANIFEST_DIR").map_err(|e| {
            syn::Error::new(
                relative_path.span(),
                format!("Cannot read `CARGO_MANIFEST_DIR` env var: {e}"),
            )
        })?;
        let path = PathBuf::from(root_path).join(get_path_arg(relative_path)?);

        let mut file = File::create(path).map_err(|e| {
            syn::Error::new(
                relative_path.span(),
                format!("Failed to create file at the provided path: {e}"),
            )
        })?;

        let registerers = self
            .fn_expanders
            .iter()
            .map(|f| FnRegistrationBuilder {
                inputs: f.input_args.iter().cloned().collect(),
                output: f.ret_ok_ty.clone(),
                name: f.ident.clone(),
                error_setter_ident: f.error_setter_ident.clone(),
            })
            .collect::<Vec<_>>();
        let generated_code =
            DartCodegen::new(&self.register_fn_name, registerers)?
                .generate()
                .map_err(|e| {
                    syn::Error::new(
                        relative_path.span(),
                        format!("Failed to generate Dart code: {e}"),
                    )
                })?;

        file.write_all(generated_code.as_bytes()).map_err(|e| {
            let msg = format!(
                "Failed to write generated Dart code to the file: {e}",
            );
            syn::Error::new(relative_path.span(), msg)
        })
    }
}

#[cfg(feature = "dart-codegen")]
/// Returns a [`String`] from the provided [`syn::Lit::Str`].
///
/// # Errors
///
/// If the provided [`syn::ExprLit`] isn't a [`syn::Lit::Str`].
fn get_path_arg(arg: &syn::ExprLit) -> syn::Result<String> {
    use proc_macro2::Span;

    if let syn::Lit::Str(lit) = &arg.lit {
        Ok(lit.value())
    } else {
        let msg = format!(
            "Expected a str literal with a Dart file path, got: {arg:?}",
        );
        Err(syn::Error::new(Span::call_site(), msg))
    }
}

mod mod_parser {
    //! Parser utils for the [`ModExpander`].
    //!
    //! [`ModExpander`]: super::ModExpander

    use proc_macro2::Span;
    use syn::{spanned::Spanned as _, token};

    /// Tries to parse a [`syn::TraitItemMethod`] from the provided
    /// [`syn::ForeignItem`].
    ///
    /// # Errors
    ///
    /// If provided [`syn::ForeignItem`] cannot be parsed as a
    /// [`syn::TraitItemMethod`].
    pub(super) fn get_extern_fn(
        item: syn::ForeignItem,
    ) -> syn::Result<syn::ForeignItemFn> {
        if let syn::ForeignItem::Fn(func) = item {
            Ok(func)
        } else {
            Err(syn::Error::new(item.span(), "Unsupported item"))
        }
    }

    /// Tries to unwrap the provided [`ItemMod::content`].
    ///
    /// # Errors
    ///
    /// If the [`ItemMod::content`] is [`None`].
    pub(super) fn try_unwrap_mod_content(
        item: Option<(token::Brace, Vec<syn::Item>)>,
    ) -> syn::Result<Vec<syn::Item>> {
        if let Some((_, items)) = item {
            Ok(items)
        } else {
            Err(syn::Error::new(Span::call_site(), "Empty module provided"))
        }
    }
}

/// Generator of [`Ident`]s for a [`FnExpander`].
struct IdentGenerator<'a> {
    /// Prefix to use in the generated [`Ident`]s.
    prefix: &'a syn::Ident,

    /// Name to concatenate with an [`IdentGenerator::prefix`].
    name: &'a syn::Ident,
}

impl<'a> IdentGenerator<'a> {
    /// Returns a new [`IdentGenerator`] with the provided `prefix` and `name`.
    const fn new(prefix: &'a syn::Ident, name: &'a syn::Ident) -> Self {
        Self { prefix, name }
    }

    /// Returns a [`syn::Ident`] for the [`FnExpander`]'s type alias.
    ///
    /// Generates something like `PeerConnectionCreateOfferFunction`.
    fn type_alias(&self) -> syn::Ident {
        format_ident!(
            "{}{}Function",
            self.prefix.to_string().to_class_case(),
            self.name.to_string().to_class_case(),
        )
    }

    /// Returns a [`syn::Ident`] for the [`FnExpander`]'s storage.
    ///
    /// Generates something like `PEER_CONNECTION__CREATE_OFFER__FUNCTION`
    fn fn_storage(&self) -> syn::Ident {
        format_ident!(
            "{}__{}__FUNCTION",
            self.prefix.to_string().to_screaming_snake_case(),
            self.name.to_string().to_screaming_snake_case(),
        )
    }

    /// Returns a [`syn::Ident`] for the [`FnExpander`]'s error slot name.
    ///
    /// Generates something like `PEER_CONNECTION__CREATE_OFFER__ERROR`
    fn error_slot_name(&self) -> syn::Ident {
        format_ident!(
            "{}__{}__ERROR",
            self.prefix.to_string().to_screaming_snake_case(),
            self.name.to_string().to_screaming_snake_case(),
        )
    }

    /// Returns a [`syn::Ident`] for the [`FnExpander`]'s error setter function
    /// name.
    ///
    /// Generates something like `peer_connection__create_offer__set_error`.
    fn error_setter_name(&self) -> syn::Ident {
        format_ident!(
            "{}__{}__set_error",
            self.prefix.to_string().to_lowercase(),
            self.name.to_string().to_lowercase(),
        )
    }
}

/// Expander of the Dart functions declarations.
///
/// Expands `unsafe fn create_offer(peer: Dart_Handle) -> Dart_Handle` to the
/// Dart function registerer, caller and pointer store.
#[derive(Debug)]
struct FnExpander {
    /// [`syn::Ident`] of the Dart function.
    ident: syn::Ident,

    /// [`syn::Ident`] of the type alias for the extern Dart function pointer.
    type_alias_ident: syn::Ident,

    /// [`syn::Ident`] of the storage storing extern Dart function
    /// pointer.
    fn_storage_ident: syn::Ident,

    /// [`syn::Ident`] of the function error slot (`thread_local! {
    /// RefCell<Option<Error>> }`).
    error_slot_ident: syn::Ident,

    /// [`syn::Ident`] of the extern function that saves error in its slot.
    error_setter_ident: syn::Ident,

    /// [`syn::FnArg`]s of extern Dart function.
    input_args: Punctuated<syn::FnArg, token::Comma>,

    /// [`syn::ReturnType`] of the extern Dart function.
    ///
    /// This is always a [`Result`].
    ret_ty: syn::ReturnType,

    /// [`Result::Ok`] type of the functions return type.
    ret_ok_ty: syn::Type,

    /// `#[doc = "..."]` [`syn::Attribute`]s injected to the generated extern
    /// Dart function caller.
    doc_attrs: Vec<syn::Attribute>,
}

impl FnExpander {
    /// Parses the all data needed for a [`FnExpander`] to expand from the
    /// provided [`TraitItemMethod`].
    ///
    /// # Errors
    ///
    /// If provided the [`TraitItemMethod`] cannot be parsed as a [`FnExpander`]
    /// data.
    fn parse(
        item: syn::ForeignItemFn,
        prefix: &syn::Ident,
    ) -> syn::Result<Self> {
        for arg in &item.sig.inputs {
            match arg {
                syn::FnArg::Typed(a) => {
                    if !matches!(&*a.pat, syn::Pat::Ident(_)) {
                        return Err(syn::Error::new(
                            a.span(),
                            "Incorrect argument identifier",
                        ));
                    }
                }
                syn::FnArg::Receiver(_) => {
                    return Err(syn::Error::new(
                        arg.span(),
                        "`self` argument is invalid here",
                    ));
                }
            }
        }

        let ret_ok_ty = {
            let err = Err(syn::Error::new(
                item.sig.output.span(),
                "must return `Result<T, platform::Error>`",
            ));

            let syn::ReturnType::Type(_, ret_ty) = item.sig.output.clone()
            else {
                return err;
            };
            let syn::Type::Path(ret_ty_path) = *ret_ty else {
                return err;
            };
            let Some(ret_ty_args) = ret_ty_path
                .path
                .segments
                .last()
                .map(|s| s.arguments.clone())
            else {
                return err;
            };
            let syn::PathArguments::AngleBracketed(res) = &ret_ty_args else {
                return err;
            };
            let Some(syn::GenericArgument::Type(res_ok_ty)) = res.args.first()
            else {
                return err;
            };

            res_ok_ty.clone()
        };

        let ident_generator = IdentGenerator::new(prefix, &item.sig.ident);
        Ok(Self {
            type_alias_ident: ident_generator.type_alias(),
            fn_storage_ident: ident_generator.fn_storage(),
            error_slot_ident: ident_generator.error_slot_name(),
            error_setter_ident: ident_generator.error_setter_name(),
            ident: item.sig.ident,
            input_args: item.sig.inputs,
            ret_ty: item.sig.output,
            ret_ok_ty,
            doc_attrs: item
                .attrs
                .into_iter()
                .map(|attr| {
                    if attr.path().get_ident().map_or(false, |i| i == "doc") {
                        Ok(attr)
                    } else {
                        Err(syn::Error::new(
                        attr.span(),
                        "only #[doc] attributes supported on extern functions",
                    ))
                    }
                })
                .collect::<syn::Result<_>>()?,
        })
    }

    /// Generates a [`syn::FnArg`] of this [`FnExpander`] for the registerer
    /// function.
    ///
    /// # Example of the generated code
    ///
    /// ```ignore
    /// create_offer: PeerConnectionCreateOfferFunction
    /// ```
    fn gen_register_fn_input(&self) -> syn::FnArg {
        let ident = &self.ident;
        let fn_type_alias = &self.type_alias_ident;

        parse_quote! {
            #ident: #fn_type_alias
        }
    }

    /// Generates a [`syn Expr`] of this [`FnExpander`] for the registerer
    /// function.
    ///
    /// # Example of the generated code
    ///
    /// ```ignore
    /// PEER_CONNECTION__CREATE_OFFER__FUNCTION.set(Some(create_offer))
    /// ```
    fn gen_register_fn_expr(&self) -> syn::Expr {
        let fn_storage_ident = &self.fn_storage_ident;
        let ident = &self.ident;

        parse_quote! {
            #fn_storage_ident.set(Some(#ident))
        }
    }

    /// Generates type alias of the extern Dart function.
    ///
    /// # Example of the generated code
    ///
    /// ```ignore
    /// type PeerConnectionCreateOfferFunction =
    ///     extern "C" fn(peer: Dart_Handle) -> Dart_Handle;
    /// ```
    fn gen_fn_type(&self) -> TokenStream {
        let name = &self.type_alias_ident;
        let ret_ok_ty = &self.ret_ok_ty;
        let args = &self.input_args;

        quote! {
            type #name = extern "C" fn (#args) -> #ret_ok_ty;
        }
    }

    /// Generates `thread_local`s for the extern Dart function pointer storing.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// ::std::thread_local! {
    ///     static PEER_CONNECTION__CREATE_OFFER__FUNCTION:
    ///         ::std::cell::RefCell<
    ///             Option<PeerConnectionCreateOfferFunction>
    ///         > = ::std::cell::RefCell::default();
    /// }
    /// ```
    fn gen_fn_storages(&self) -> TokenStream {
        let name = &self.fn_storage_ident;
        let type_alias = &self.type_alias_ident;

        quote! {
            ::std::thread_local! {
                static #name: ::std::cell::RefCell<Option<#type_alias>> =
                    ::std::cell::RefCell::default();
            }
        }
    }

    /// Generates Dart function caller for Rust.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// pub unsafe fn create_offer(
    ///     peer: Dart_Handle,
    /// ) -> Result<Dart_Handle, Error> {
    ///     let res = PEER_CONNECTION__CREATE_OFFER__FUNCTION.with_borrow(
    ///         |__fn_storage| (*__fn_storage.as_ref().unwrap())(peer),
    ///     );
    ///
    ///     if let Some(e) = PEER_CONNECTION__CREATE_OFFER__ERROR.take() {
    ///         Err(e)
    ///     } else {
    ///         Ok(res)
    ///     }
    /// }
    /// ```
    fn gen_caller_fn(&self) -> TokenStream {
        let doc_attrs = &self.doc_attrs;
        let name = &self.ident;
        let error_slot = &self.error_slot_ident;

        let args = &self.input_args;
        let args_idents = self.input_args.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(a) = arg {
                if let syn::Pat::Ident(pat) = &*a.pat {
                    return Some(&pat.ident);
                }
            }
            None
        });

        let ret_ty = &self.ret_ty;

        let fn_storage_ident = &self.fn_storage_ident;

        quote! {
            #( #doc_attrs )*
            pub unsafe fn #name(#args) #ret_ty {
                let res = #fn_storage_ident.with_borrow(|__fn_storage| {
                    (*__fn_storage.as_ref().unwrap())(#( #args_idents ),*)
                });
                if let Some(e) = #error_slot.take() {
                  Err(e)
                } else {
                  Ok(res)
                }
            }
        }
    }

    /// Generates an error slot for this [`FnExpander`].
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// ::std::thread_local! {
    ///     static PEER_CONNECTION__CREATE_OFFER__ERROR: ::std::cell::RefCell<
    ///         Option<Error>,
    ///     > = ::std::cell::RefCell::default();
    /// }
    /// ```
    fn get_errors_slot(&self) -> TokenStream {
        let name = &self.error_slot_ident;

        quote! {
            ::std::thread_local! {
                static #name: ::std::cell::RefCell<Option<Error>> =
                    ::std::cell::RefCell::new(None);
            }
        }
    }

    /// Generates an error setter extern function.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// #[no_mangle]
    /// pub unsafe extern "C" fn peer_connection__connection_state__set_error(
    ///     err: Dart_Handle,
    /// ) {
    ///     PEER_CONNECTION__CONNECTION_STATE__ERROR
    ///         .set(Some(Error::from_handle(err)));
    /// }
    /// ```
    fn gen_error_setter(&self) -> TokenStream {
        let doc = format!("Error setter for the `{}` function", self.ident);
        let fn_name = &self.error_setter_ident;
        let error_slot = &self.error_slot_ident;

        quote! {
            #[doc = #doc]
            #[no_mangle]
            pub unsafe extern "C" fn #fn_name(err: Dart_Handle) {
                #error_slot.set(Some(Error::from_handle(err)));
            }
        }
    }
}
