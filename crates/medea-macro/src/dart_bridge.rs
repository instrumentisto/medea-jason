//! `#[dart_bridge]` macro implementation.

use std::{convert::TryFrom, io::Write, path::PathBuf};
use std::fs::File;

use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::TokenStreamExt;
use syn::{
    parse::{Error, Parse, Parser, Result},
    punctuated::Punctuated,
    spanned::Spanned as _,
    Attribute, Expr, ExprLit, FnArg, ForeignItemFn, Ident, Item, ItemMod,
    ItemUse, Lit, ReturnType, Token, Visibility,
};

use crate::dart_codegen::{DartCodegen, FnRegistrationBuilder};

/// Creates a [`Ident`] using interpolation of runtime expressions.
///
/// Works same as [`format`] macro, but generated [`Ident`].
macro_rules! format_ident {
    ($($arg:tt)*) => {{
        Ident::new(
            &format!($($arg)*),
            Span::call_site(),
        )
    }}
}

/// Returns [`String`] from the provided [`ExprLit`].
///
/// # Errors
///
/// If provided [`ExprLit`] isn't [`Lit::Str`].
pub fn get_path_arg(arg: &ExprLit) -> Result<String> {
    if let Lit::Str(arg) = &arg.lit {
        Ok(arg.value())
    } else {
        Err(Error::new(
            Span::call_site(),
            "Expected str literal with a Dart file path",
        ))
    }
}

/// Expander of the `#[dart_bridge]` macro.
///
/// Expands to module with a registerers of the Dart functions and it's callers.
#[derive(Debug)]
struct ModExpander {
    /// Visibility of the expanded module.
    vis: Visibility,

    /// Identifier of the expanded module.
    ident: Ident,

    /// Uses (`use foo::bar::baz`) of the expanded module.
    uses: Vec<ItemUse>,

    /// [`FnExpander`]s of the all functions which this module should contain.
    fn_expanders: Vec<FnExpander>,

    register_fn_name: Ident,
}

impl ModExpander {
    /// Expands Dart functions type aliases of all
    /// [`ModExpander::fn_expanders`].
    fn expand_type_aliases(&self) -> TokenStream2 {
        let mut out = TokenStream2::new();
        for f in &self.fn_expanders {
            out.append_all(f.expand_fn_type());
        }
        out
    }

    /// Expands `static mut`s of all [`ModExpander::fn_expanders`]
    fn expand_static_muts(&self) -> TokenStream2 {
        let mut out = TokenStream2::new();
        for f in &self.fn_expanders {
            out.append_all(f.expand_fn_static_mut());
        }
        out
    }

    /// Expands Dart functions registerers of all [`ModExpander::fn_expanders`].
    fn expand_register_fns(&self) -> TokenStream2 {
        let mut inputs: Punctuated<FnArg, Token![,]> = Punctuated::new();
        let mut assigns: Vec<Expr> = Vec::new();
        let name = &self.register_fn_name;

        for f in &self.fn_expanders {
            inputs.push(f.expand_register_fn_input());
            assigns.push(f.expand_register_fn_expr());
        }

        quote::quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #name(
                #inputs
            ) {
                #(#assigns;)*
            }
        }
    }

    /// Expands Dart functions of all [`ModExpander::fn_expanders`].
    fn expand_fns(&self) -> TokenStream2 {
        let mut out = TokenStream2::new();
        for f in &self.fn_expanders {
            out.append_all(f.expand_fn());
        }
        out
    }

    /// Fully expands `#[dart_bridge]` macro.
    fn expand(&self) -> TokenStream2 {
        let type_aliases = self.expand_type_aliases();
        let static_muts = self.expand_static_muts();
        let register_fns = self.expand_register_fns();
        let fns = self.expand_fns();
        let mod_ident = &self.ident;
        let mod_vis = &self.vis;
        let uses = self.uses.iter();

        quote::quote! {
            #mod_vis mod #mod_ident {
                #(#uses)*

                #type_aliases

                #static_muts

                #register_fns

                #fns
            }
        }
    }

    /// Generates all required Dart code at the provided `relative_path`.
    fn generate_dart(&self, relative_path: &ExprLit) -> Result<()> {
        let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = PathBuf::from(root_path);
        path.push(get_path_arg(relative_path)?);

        let mut f = File::create(path).map_err(|e| {
            Error::new(
                relative_path.span(),
                format!("Failed to create file at the provided path: {:?}", e),
            )
        })?;

        let mut registerers = Vec::new();
        for f in &self.fn_expanders {
            let registerer = FnRegistrationBuilder {
                inputs: f.inputs.iter().cloned().collect(),
                output: f.out_type.clone(),
                name: f.ident.clone(),
            };
            registerers.push(registerer);
        }
        let generated_code =
            DartCodegen::new(&self.register_fn_name, registerers)?.generate();

        f.write_all(generated_code.as_bytes()).map_err(|e| {
            Error::new(
                relative_path.span(),
                format!(
                    "Failed to write generated Dart \
        code at the provided path: {:?}",
                    e
                ),
            )
        })?;

        Ok(())
    }
}

mod mod_parser {
    //! Parser utils for the [`ModExpander`].
    //!
    //! [`ModExpander`]: super::ModExpander

    use proc_macro2::Span;
    use syn::{
        spanned::Spanned as _, token, Error, ForeignItem, ForeignItemFn, Item,
        Result,
    };

    /// Tries to parse [`TraitItemMethod`] from the provided [`TokenStream2`].
    ///
    /// # Errors
    ///
    /// If provided [`TokenStream2`] can't be parsed as [`TraitItemMethod`].
    pub fn get_extern_fn(item: ForeignItem) -> Result<ForeignItemFn> {
        if let ForeignItem::Fn(item) = item {
            Ok(item)
        } else {
            Err(Error::new(item.span(), "Unsupported item"))
        }
    }

    /// Tries to unwrap provided [`ItemMod::content`].
    ///
    /// # Errors
    ///
    /// If [`ItemMod::content`] is `None`.
    pub fn try_unwrap_mod_content(
        item: Option<(token::Brace, Vec<Item>)>,
    ) -> Result<Vec<Item>> {
        if let Some((_, items)) = item {
            Ok(items)
        } else {
            Err(Error::new(Span::call_site(), "Empty module provided"))
        }
    }
}

impl TryFrom<ItemMod> for ModExpander {
    type Error = Error;

    fn try_from(item: ItemMod) -> Result<Self> {
        use mod_parser as parser;

        let mod_item_span = item.span();

        let mut extern_functions: Vec<FnExpander> = Vec::new();
        let mut use_items = Vec::new();
        let register_prefix = &item.ident;
        for item in parser::try_unwrap_mod_content(item.content)? {
            match item {
                Item::ForeignMod(item) => {
                    for item in item.items {
                        extern_functions.push(FnExpander::parse(
                            parser::get_extern_fn(item)?,
                            register_prefix,
                        )?);
                    }
                }
                Item::Use(item) => {
                    use_items.push(item);
                }
                _ => {
                    return Err(Error::new(
                        item.span(),
                        "Module contains unsupported content",
                    ));
                }
            }
        }

        if extern_functions.is_empty() {
            return Err(Error::new(
                mod_item_span,
                "At least one extern \"C\" mod required",
            ));
        }

        Ok(Self {
            register_fn_name: format_ident!("register_{}", item.ident),
            ident: item.ident,
            vis: item.vis,
            uses: use_items,
            fn_expanders: extern_functions,
        })
    }
}

/// Generator of the [`Ident`]s for the [`FnExpander`].
struct IdentGenerator<'a> {
    /// Prefix which will be used in generated [`Ident`]s.
    prefix: &'a Ident,

    /// Name which will be concatenated with a [`IdentGenerator::prefix`].
    name: &'a Ident,
}

impl<'a> IdentGenerator<'a> {
    /// Returns new [`IdentGenerator`] with a provided `prefix` and `name`.
    fn new(prefix: &'a Ident, name: &'a Ident) -> Self {
        Self { prefix, name }
    }

    /// Returns [`Ident`] for the [`FnExpander`]'s type alias.
    ///
    /// Generates something like `PeerConnectionCreateOfferFunction`.
    fn type_alias(&self) -> Ident {
        format_ident!(
            "{}{}Function",
            self.prefix.to_string().to_class_case(),
            self.name.to_string().to_class_case(),
        )
    }

    /// Returns [`Ident`] for the [`FnExpander`]'s `static mut`s.
    ///
    /// Generates something like `PEER_CONNECTION__CREATE_OFFER__FUNCTION`
    fn static_mut(&self) -> Ident {
        format_ident!(
            "{}__{}__FUNCTION",
            self.prefix.to_string().to_screaming_snake_case(),
            self.name.to_string().to_screaming_snake_case(),
        )
    }
}

mod fn_parser {
    //! Parser utils for the [`FnExpander`].
    //!
    //! [`FnExpander`]: super::FnExpander

    use syn::{
        punctuated::Punctuated, spanned::Spanned as _, Attribute, Error, FnArg,
        Ident, Pat, Result, Token,
    };

    /// Returns [`Punctuated`] [`Ident`]s of the all provided [`FnArg`]s.
    ///
    /// # Errors
    ///
    /// If some [`FnArg`] is something like `self`.
    ///
    /// If some [`FnArg`] doesn't have [`Ident`].
    pub fn get_input_idents<'a, I>(
        args: I,
    ) -> Result<Punctuated<Ident, Token![,]>>
    where
        I: IntoIterator<Item = &'a FnArg>,
    {
        let mut out = Punctuated::new();
        for item in args {
            match item {
                FnArg::Typed(item) => {
                    if let Pat::Ident(item) = item.pat.as_ref() {
                        out.push(item.ident.clone());
                    } else {
                        return Err(Error::new(
                            item.span(),
                            "Incorrect argument identifier",
                        ));
                    }
                }
                FnArg::Receiver(_) => {
                    return Err(Error::new(
                        item.span(),
                        "self argument is invalid here",
                    ));
                }
            }
        }

        Ok(out)
    }

    /// Removes all [`Attribute`]s which are not `#[doc = "..."]`.
    pub fn filter_doc_attributes<I>(attrs: I) -> Result<Vec<Attribute>>
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut out = Vec::new();
        for attr in attrs {
            if attr.path.get_ident().map_or(false, |i| i == "doc") {
                out.push(attr);
            } else {
                return Err(Error::new(
                    attr.span(),
                    "Only #[doc] attributes supported on extern fns",
                ));
            }
        }

        Ok(out)
    }
}

/// Expander of the Dart functions declarations.
///
/// Expands `unsafe fn create_offer(peer: Dart_Handle) -> Dart_Handle` to the
/// Dart function registerer, caller and store.
#[derive(Debug)]
struct FnExpander {
    /// [`Ident`] of the Dart function.
    ident: Ident,

    /// [`Ident`] of the type alias for the extern Dart function.
    fn_type_alias_ident: Ident,

    /// [`Ident`] of the `static mut` which stores extern Dart function.
    fn_static_mut_ident: Ident,

    /// [`FnArg`]s of extern Dart function.
    inputs: Punctuated<FnArg, Token![,]>,

    /// [`Punctuated`] [`Ident`]s of the all [`FnArg`]s of extern Dart
    /// function.
    input_idents: Punctuated<Ident, Token![,]>,

    /// [`ReturnType`] of the extern Dart function.
    out_type: ReturnType,

    /// `#[doc = "..."]` [`Attribute`]s which will be injected to the generated
    /// extern Dart function caller.
    doc_attrs: Vec<Attribute>,
}

impl FnExpander {
    /// Parses all data needed for [`FnExpander`] expanding from the provided
    /// [`TraitItemMethod`].
    ///
    /// # Errors
    ///
    /// If provided [`TraitItemMethod`] can't be parsed as data for the
    /// [`FnExpander`].
    fn parse(item: ForeignItemFn, prefix: &Ident) -> Result<Self> {
        use fn_parser as parser;

        let ident_generator = IdentGenerator::new(prefix, &item.sig.ident);
        Ok(Self {
            fn_type_alias_ident: ident_generator.type_alias(),
            fn_static_mut_ident: ident_generator.static_mut(),
            input_idents: parser::get_input_idents(&item.sig.inputs)?,
            ident: item.sig.ident,
            inputs: item.sig.inputs,
            out_type: item.sig.output,
            doc_attrs: parser::filter_doc_attributes(item.attrs)?,
        })
    }

    /// Generates [`FnArg`] of this [`FnExpander`] for the registerer function.
    ///
    /// # Example of the generated code
    ///
    /// ```ignore
    /// create_offer: PeerConnectionCreateOfferFunction
    /// ```
    fn expand_register_fn_input(&self) -> FnArg {
        let ident = &self.ident;
        let fn_type_alias = &self.fn_type_alias_ident;
        FnArg::parse
            .parse2(quote::quote! {
                #ident: #fn_type_alias
            })
            .unwrap()
    }

    /// Generates [`Expr`] of this [`FnExpander`] for the registerer function.
    ///
    /// # Example of the generated code
    ///
    /// ```ignore
    /// PEER_CONNECTION__CREATE_OFFER__FUNCTION.write(create_offer)
    /// ```
    fn expand_register_fn_expr(&self) -> Expr {
        let fn_static_mut = &self.fn_static_mut_ident;
        let ident = &self.ident;
        Expr::parse
            .parse2(quote::quote! {
                #fn_static_mut.write(#ident)
            })
            .unwrap()
    }

    /// Generates type alias of the extern Dart function.
    ///
    /// # Example of the generated code
    ///
    /// ```ignore
    /// type PeerConnectionCreateOfferFunction =
    ///     extern "C" fn(peer: Dart_Handle) -> Dart_Handle;
    /// ```
    fn expand_fn_type(&self) -> TokenStream2 {
        let name = &self.fn_type_alias_ident;
        let out_type = &self.out_type;
        let inputs = &self.inputs;

        quote::quote! {
            type #name = extern "C" fn (#inputs) #out_type;
        }
    }

    /// Generates `static mut` for extern Dart function storing.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// static mut PEER_CONNECTION__CREATE_OFFER__FUNCTION:
    ///     std::mem::MaybeUninit<PeerConnectionCreateOfferFunction> =
    ///     std::mem::MaybeUninit::uninit();
    /// ```
    fn expand_fn_static_mut(&self) -> TokenStream2 {
        let name = &self.fn_static_mut_ident;
        let type_alias_ident = &self.fn_type_alias_ident;

        quote::quote! {
            static mut #name: std::mem::MaybeUninit<#type_alias_ident> =
                std::mem::MaybeUninit::uninit();
        }
    }

    /// Generates Dart function caller.
    fn expand_fn(&self) -> TokenStream2 {
        let ident = &self.ident;
        let inputs = &self.inputs;
        let out_type = &self.out_type;
        let static_mut_ident = &self.fn_static_mut_ident;
        let input_idents = &self.input_idents;
        let doc_attrs = self.doc_attrs.iter();

        quote::quote! {
            #(#doc_attrs)*
            pub unsafe fn #ident(#inputs) #out_type {
                (#static_mut_ident.assume_init_ref())(#input_idents)
            }
        }
    }
}

/// Expands `#[dart_bridge]` macro based on the provided [`ItemMod`].
pub fn expand(path: &ExprLit, item: ItemMod) -> Result<TokenStream> {
    let expander = ModExpander::try_from(item)?;
    expander.generate_dart(path)?;
    Ok(expander.expand().into())
}
