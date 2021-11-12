//! `extern_dart!` macro implementation.

use std::convert::TryFrom;

use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::TokenStreamExt;
use syn::{
    parse::{Error, Result},
    punctuated::Punctuated,
    spanned::Spanned as _,
    Attribute, FnArg, Ident, Item, ItemMod, ItemUse, ReturnType, Token,
    TraitItemMethod, Visibility,
};

/// Expander of the `extern_dart!` macro.
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
        let mut out = TokenStream2::new();
        for f in &self.fn_expanders {
            out.append_all(f.expand_register_fn());
        }
        out
    }

    /// Expands Dart functions of all [`ModExpander::fn_expanders`].
    fn expand_fns(&self) -> TokenStream2 {
        let mut out = TokenStream2::new();
        for f in &self.fn_expanders {
            out.append_all(f.expand_fn());
        }
        out
    }

    /// Fully expands `extern_dart!` macro.
    fn expand(&self) -> TokenStream2 {
        let type_aliases = self.expand_type_aliases();
        let static_muts = self.expand_static_muts();
        let register_fns = self.expand_register_fns();
        let fns = self.expand_fns();
        let mod_ident = &self.ident;
        let mod_vis = &self.vis;
        let uses = self.uses.iter();

        quote::quote! {
            pub use self::sys::registerers::*;

            #mod_vis mod #mod_ident {
                #(#uses)*

                #type_aliases

                #static_muts

                pub mod registerers {
                    use super::*;

                    #register_fns
                }

                #fns
            }
        }
    }
}

mod mod_parser {
    //! Parser utils for the [`ModExpander`].
    //!
    //! [`ModExpander`]: super::ModExpander

    use proc_macro2::{Span, TokenStream as TokenStream2};
    use syn::{
        spanned::Spanned as _, token, Error, Ident, Item, ItemMacro, Result,
        TraitItem, TraitItemMethod,
    };

    /// Tries to parse prefix for the Dart functions registerers.
    ///
    /// Expects to receive `extern_prefix!(PrefixName)` macro as argument.
    ///
    /// ## Errors
    ///
    /// If identifier of the provided macro isn't `extern_prefix`.
    ///
    /// If macro's body not contains valid prefix.
    pub fn get_prefix(item: ItemMacro) -> Result<Ident> {
        let is_valid_ident = item
            .mac
            .path
            .segments
            .last()
            .as_ref()
            .map_or(false, |i| i.ident == "extern_prefix");
        if !is_valid_ident {
            return Err(Error::new(
                item.span(),
                "Only extern_prefix! macro supported here",
            ));
        }

        syn::parse2(item.mac.tokens)
    }

    /// Tries to parse [`TraitItemMethod`] from the provided [`TokenStream2`].
    ///
    /// # Errors
    ///
    /// If provided [`TokenStream2`] can't be parsed as [`TraitItemMethod`].
    pub fn get_extern_fn(item: TokenStream2) -> Result<TraitItemMethod> {
        let item = syn::parse2(item)?;
        if let TraitItem::Method(item) = item {
            Ok(item)
        } else {
            Err(Error::new(item.span(), "Unsupported item"))
        }
    }

    /// Tries to unwrap provided `prefix`.
    ///
    /// # Errors
    ///
    /// If provided `prefix` is `None`.
    pub fn try_unwrap_register_prefix(
        prefix: &Option<Ident>,
    ) -> Result<&Ident> {
        prefix.as_ref().ok_or_else(|| {
            Error::new(
                Span::call_site(),
                "extern_prefix! should be declared before extern functions",
            )
        })
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

        let mut register_prefix: Option<Ident> = None;
        let mut extern_functions: Vec<FnExpander> = Vec::new();
        let mut use_items = Vec::new();
        for item in parser::try_unwrap_mod_content(item.content)? {
            match item {
                Item::Macro(item) => {
                    if register_prefix.is_some() {
                        return Err(Error::new(
                            item.span(),
                            "Duplicating extern_prefix! macro",
                        ));
                    }
                    register_prefix = Some(parser::get_prefix(item)?);
                }
                Item::Verbatim(item) => {
                    let register_prefix =
                        parser::try_unwrap_register_prefix(&register_prefix)?;
                    extern_functions.push(FnExpander::parse(
                        parser::get_extern_fn(item)?,
                        register_prefix,
                    )?);
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
                "At least one extern fn required",
            ));
        }

        Ok(Self {
            ident: item.ident,
            vis: item.vis,
            uses: use_items,
            fn_expanders: extern_functions,
        })
    }
}

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
            self.prefix,
            self.name.to_string().to_class_case(),
        )
    }

    /// Returns [`Ident`] for the [`FnExpander`]'s registerer function.
    ///
    /// Generates something like `register_PeerConnection__create_offer`.
    fn registerer_fn(&self) -> Ident {
        format_ident!(
            "register_{}__{}",
            self.prefix.to_string(),
            self.name.to_string(),
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
        spanned::Spanned as _, Attribute, Error, FnArg, Ident, Pat, Result,
    };

    /// Returns [`Ident`]s of the all provided [`FnArg`]s.
    ///
    /// # Errors
    ///
    /// If some [`FnArg`] is something like `self`.
    ///
    /// If some [`FnArg`] doesn't have [`Ident`].
    pub fn get_input_idents<'a, I>(args: I) -> Result<Vec<Ident>>
    where
        I: IntoIterator<Item = &'a FnArg>,
    {
        let mut out = Vec::new();
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

    /// [`Ident`] of the registerer function for the extern Dart function.
    register_fn_ident: Ident,

    /// [`Ident`] of the `static mut` which stores extern Dart function.
    fn_static_mut_ident: Ident,

    /// [`FnArg`]s of extern Dart function.
    inputs: Punctuated<FnArg, Token![,]>,

    /// [`Ident`]s of the all [`FnArg`]s of extern Dart function.
    input_idents: Vec<Ident>,

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
    fn parse(item: TraitItemMethod, prefix: &Ident) -> Result<Self> {
        use fn_parser as parser;

        let ident_generator = IdentGenerator::new(prefix, &item.sig.ident);
        Ok(Self {
            fn_type_alias_ident: ident_generator.type_alias(),
            register_fn_ident: ident_generator.registerer_fn(),
            fn_static_mut_ident: ident_generator.static_mut(),
            input_idents: parser::get_input_idents(&item.sig.inputs)?,
            ident: item.sig.ident,
            inputs: item.sig.inputs,
            out_type: item.sig.output,
            doc_attrs: parser::filter_doc_attributes(item.attrs)?,
        })
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
    /// static mut PEER_CONNECTION__CREATE_OFFER__FUNCTION: Option<
    ///     PeerConnectionCreateOfferFunction,
    /// > = None;
    /// ```
    fn expand_fn_static_mut(&self) -> TokenStream2 {
        let name = &self.fn_static_mut_ident;
        let type_alias_ident = &self.fn_type_alias_ident;

        quote::quote! {
            static mut #name: Option<#type_alias_ident> = None;
        }
    }

    /// Generates `extern "C"` function which should be called by Dart side for
    /// Dart function registering.
    fn expand_register_fn(&self) -> TokenStream2 {
        let name = &self.register_fn_ident;
        let type_alias_ident = &self.fn_type_alias_ident;
        let static_mut_ident = &self.fn_static_mut_ident;

        quote::quote! {
            pub unsafe extern "C" fn #name(f: #type_alias_ident) {
                #static_mut_ident = Some(f);
            }
        }
    }

    /// Generates Dart function caller.
    fn expand_fn(&self) -> TokenStream2 {
        let ident = &self.ident;
        let inputs = &self.inputs;
        let out_type = &self.out_type;
        let static_mut_ident = &self.fn_static_mut_ident;
        let input_idents = self.input_idents.iter();
        let doc_attrs = self.doc_attrs.iter();

        quote::quote! {
            #(#doc_attrs)*
            pub unsafe fn #ident(#inputs) #out_type {
                (#static_mut_ident.unwrap())(#(#input_idents)*,)
            }
        }
    }
}

/// Expands `extern_dart!` macro based on the provided [`ItemMod`].
pub fn expand(item: ItemMod) -> Result<TokenStream> {
    Ok(ModExpander::try_from(item)?.expand().into())
}
