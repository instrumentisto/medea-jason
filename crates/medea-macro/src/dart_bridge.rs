//! `#[dart_bridge]` macro implementation.

use std::convert::TryFrom;
#[cfg(feature = "dart-codegen")]
use std::{env, fs::File, io::Write as _, path::PathBuf};

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned as _, token};

#[cfg(feature = "dart-codegen")]
use crate::dart_codegen::{DartCodegen, FnRegistrationBuilder};

/// Expands `#[dart_bridge]` attribute placed on a Rust module declaration.
#[allow(clippy::needless_pass_by_value)] // due to feature
pub(crate) fn expand(
    #[cfg(feature = "dart-codegen")] args: TokenStream,
    #[cfg(not(feature = "dart-codegen"))] _: TokenStream,
    input: TokenStream,
) -> syn::Result<TokenStream> {
    let expander = ModExpander::try_from(syn::parse2::<syn::ItemMod>(input)?)?;

    #[cfg(feature = "dart-codegen")]
    expander.generate_dart_code(&syn::parse2(args)?)?;

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

    fn try_from(item: syn::ItemMod) -> syn::Result<Self> {
        use self::mod_parser as parser;

        let mod_span = item.span();

        let mut extern_fns: Vec<FnExpander> = Vec::new();
        let mut use_items = Vec::new();
        let register_prefix = &item.ident;
        for item in parser::try_unwrap_mod_content(item.content)? {
            match item {
                syn::Item::ForeignMod(item) => {
                    for item in item.items {
                        extern_fns.push(FnExpander::parse(
                            parser::get_extern_fn(item)?,
                            register_prefix,
                        )?);
                    }
                }
                syn::Item::Use(item) => {
                    use_items.push(item);
                }
                _ => {
                    return Err(syn::Error::new(
                        item.span(),
                        "Module contains unsupported content",
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
            register_fn_name: format_ident!("register_{}", item.ident),
            vis: item.vis,
            ident: item.ident,
            attrs: item.attrs,
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

        let static_muts =
            self.fn_expanders.iter().map(FnExpander::gen_fn_static_mut);

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

        quote! {
            #[automatically_derived]
            #( #attrs )*
            #vis mod #ident {
                #( #uses )*

                #( #type_aliases )*

                #( #static_muts )*

                #[no_mangle]
                pub unsafe extern "C" fn #register_fn_name(
                    #( #register_fn_inputs, )*
                ) {
                    #( #register_fn_assigns; )*
                }

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
        let root_path = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = PathBuf::from(root_path);
        path.push(get_path_arg(relative_path)?);

        let mut f = File::create(path).map_err(|e| {
            syn::Error::new(
                relative_path.span(),
                format!("Failed to create file at the provided path: {}", e),
            )
        })?;

        let registerers = self
            .fn_expanders
            .iter()
            .map(|f| FnRegistrationBuilder {
                inputs: f.input_args.iter().cloned().collect(),
                output: f.ret_ty.clone(),
                name: f.ident.clone(),
            })
            .collect::<Vec<_>>();
        let generated_code =
            DartCodegen::new(&self.register_fn_name, registerers)?
                .generate()
                .map_err(|e| {
                    syn::Error::new(
                        relative_path.span(),
                        format!("Failed to generate Dart code: {}", e),
                    )
                })?;

        f.write_all(generated_code.as_bytes()).map_err(|e| {
            let msg = format!(
                "Failed to write generated Dart code to the file: {}",
                e,
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
pub fn get_path_arg(arg: &syn::ExprLit) -> syn::Result<String> {
    use proc_macro2::Span;

    if let syn::Lit::Str(arg) = &arg.lit {
        Ok(arg.value())
    } else {
        let msg = format!(
            "Expected a str literal with a Dart file path, got: {:?}",
            arg,
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
    pub fn get_extern_fn(
        item: syn::ForeignItem,
    ) -> syn::Result<syn::ForeignItemFn> {
        if let syn::ForeignItem::Fn(item) = item {
            Ok(item)
        } else {
            Err(syn::Error::new(item.span(), "Unsupported item"))
        }
    }

    /// Tries to unwrap the provided [`ItemMod::content`].
    ///
    /// # Errors
    ///
    /// If the [`ItemMod::content`] is [`None`].
    pub fn try_unwrap_mod_content(
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
    fn new(prefix: &'a syn::Ident, name: &'a syn::Ident) -> Self {
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

    /// Returns a [`syn::Ident`] for the [`FnExpander`]'s `static mut`.
    ///
    /// Generates something like `PEER_CONNECTION__CREATE_OFFER__FUNCTION`
    fn static_mut(&self) -> syn::Ident {
        format_ident!(
            "{}__{}__FUNCTION",
            self.prefix.to_string().to_screaming_snake_case(),
            self.name.to_string().to_screaming_snake_case(),
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

    /// [`syn::Ident`] of the `static mut` storing extern Dart function
    /// pointer.
    static_mut_ident: syn::Ident,

    /// [`syn::FnArg`]s of extern Dart function.
    input_args: Punctuated<syn::FnArg, token::Comma>,

    /// [`syn::ReturnType`] of the extern Dart function.
    ret_ty: syn::ReturnType,

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
        for item in &item.sig.inputs {
            match item {
                syn::FnArg::Typed(item) => {
                    if !matches!(&*item.pat, syn::Pat::Ident(_)) {
                        return Err(syn::Error::new(
                            item.span(),
                            "Incorrect argument identifier",
                        ));
                    }
                }
                syn::FnArg::Receiver(_) => {
                    return Err(syn::Error::new(
                        item.span(),
                        "`self` argument is invalid here",
                    ));
                }
            }
        }

        let ident_generator = IdentGenerator::new(prefix, &item.sig.ident);
        Ok(Self {
            type_alias_ident: ident_generator.type_alias(),
            static_mut_ident: ident_generator.static_mut(),
            ident: item.sig.ident,
            input_args: item.sig.inputs,
            ret_ty: item.sig.output,
            doc_attrs: item
                .attrs
                .into_iter()
                .map(|attr| {
                    if attr.path.get_ident().map_or(false, |i| i == "doc") {
                        Ok(attr)
                    } else {
                        Err(syn::Error::new(
                        attr.span(),
                        "Only #[doc] attributes supported on extern functions",
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
    /// PEER_CONNECTION__CREATE_OFFER__FUNCTION.write(create_offer)
    /// ```
    fn gen_register_fn_expr(&self) -> syn::Expr {
        let fn_static_mut = &self.static_mut_ident;
        let ident = &self.ident;

        parse_quote! {
            #fn_static_mut.write(#ident)
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
        let ret_ty = &self.ret_ty;
        let args = &self.input_args;

        quote! {
            type #name = extern "C" fn (#args) #ret_ty;
        }
    }

    /// Generates `static mut` for the extern Dart function pointer storing.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// static mut PEER_CONNECTION__CREATE_OFFER__FUNCTION:
    ///     std::mem::MaybeUninit<PeerConnectionCreateOfferFunction> =
    ///     std::mem::MaybeUninit::uninit();
    /// ```
    fn gen_fn_static_mut(&self) -> TokenStream {
        let name = &self.static_mut_ident;
        let type_alias = &self.type_alias_ident;

        quote! {
            static mut #name: ::std::mem::MaybeUninit<#type_alias> =
                ::std::mem::MaybeUninit::uninit();
        }
    }

    /// Generates Dart function caller for Rust.
    fn gen_caller_fn(&self) -> TokenStream {
        let doc_attrs = &self.doc_attrs;
        let name = &self.ident;

        let args = &self.input_args;
        let args_idents = self.input_args.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(arg) = arg {
                if let syn::Pat::Ident(pat) = &*arg.pat {
                    return Some(&pat.ident);
                }
            }
            None
        });

        let ret_ty = &self.ret_ty;

        let static_mut = &self.static_mut_ident;

        quote! {
            #( #doc_attrs )*
            pub unsafe fn #name(#args) #ret_ty {
                (#static_mut.assume_init_ref())(#( #args_idents ),*)
            }
        }
    }
}
