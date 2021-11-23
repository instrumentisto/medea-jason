//! Dart side register functions generator of `#[dart_bridge]` macro.

use std::{
    convert::TryFrom,
    fmt::{self, Write},
};

use inflector::Inflector;
use syn::spanned::Spanned as _;

/// Types that can be passed through FFI.
#[derive(Debug, Clone, Copy)]
pub(crate) enum DartType {
    /// Pointer to the Dart `Object`.
    ///
    /// Represents a [Handle] on the Dart side.
    ///
    /// [Handle]: https://api.dart.dev/stable/dart-ffi/Handle-class.html
    Handle,

    /// [`c_char`] pointer.
    ///
    /// Represents [Pointer<Utf8>][0] on the Dart side.
    ///
    /// [0]: https://pub.dev/documentation/ffi/latest/ffi/Utf8-class.html
    StringPointer,

    /// Pointer to a boxed Dart `Object`.
    ///
    /// Represents [Pointer<Handle>][0] on the Dart side.
    ///
    /// [0]: https://api.dart.dev/stable/dart-ffi/Pointer-class.html
    HandlePointer,

    /// 8-bit integer.
    ///
    /// Represents [Int8] on the Dart side.
    ///
    /// [Int8]: https://api.dart.dev/stable/dart-ffi/Int8-class.html
    Int8,

    /// 32-bit integer.
    ///
    /// Represents [Int32] on the Dart side.
    ///
    /// [Int32]: https://api.dart.dev/stable/dart-ffi/Int32-class.html
    Int32,

    /// 64-bit integer.
    ///
    /// Represents [Int64] on the Dart side.
    ///
    /// [Int64]: https://api.dart.dev/stable/dart-ffi/Int64-class.html
    Int64,

    /// Pointer to the Rust structure.
    ///
    /// Represents [Pointer] on the Dart side.
    ///
    /// [Pointer]: https://api.dart.dev/stable/dart-ffi/Pointer-class.html
    Pointer,

    /// Type which indicates that function doesn't return anything.
    ///
    /// `void` keyword in Dart.
    Void,

    /// `DartValue` FFI structure which adds ability to cast more complex
    /// types.
    ForeignValue,
}

impl DartType {
    /// Converts this [`DartType`] to the Dart side FFI type.
    pub(crate) const fn to_ffi_type(self) -> &'static str {
        match self {
            Self::Handle => "Handle",
            Self::StringPointer => "Pointer<Utf8>",
            Self::HandlePointer => "Pointer<Handle>",
            Self::Int8 => "Int8",
            Self::Int32 => "Int32",
            Self::Int64 => "Int64",
            Self::Pointer => "Pointer",
            Self::Void => "Void",
            Self::ForeignValue => "ForeignValue",
        }
    }

    /// Parses [`ptr::NonNull`] [`DartType`] from the provided
    /// [`PathArguments`].
    ///
    /// [`ptr::NonNull`]: std::ptr::NonNull
    pub(crate) fn from_non_null_generic(
        args: &syn::PathArguments,
    ) -> syn::Result<Self> {
        if let syn::PathArguments::AngleBracketed(bracketed) = args {
            match bracketed.args.last().ok_or_else(|| {
                syn::Error::new(bracketed.span(), "Empty generics list")
            })? {
                syn::GenericArgument::Type(syn::Type::Path(p)) => {
                    let segment = p.path.segments.last().ok_or_else(|| {
                        syn::Error::new(p.span(), "Empty generic path")
                    })?;
                    Ok(if segment.ident.to_string().as_str() == "c_char" {
                        Self::StringPointer
                    } else {
                        Self::Pointer
                    })
                }
                syn::GenericArgument::Lifetime(_)
                | syn::GenericArgument::Type(_)
                | syn::GenericArgument::Binding(_)
                | syn::GenericArgument::Constraint(_)
                | syn::GenericArgument::Const(_) => Err(syn::Error::new(
                    bracketed.span(),
                    "Unsupported generic argument",
                )),
            }
        } else {
            Err(syn::Error::new(
                args.span(),
                "Unsupported `NonNull` path arguments",
            ))
        }
    }
}

impl TryFrom<syn::Type> for DartType {
    type Error = syn::Error;

    #[allow(clippy::wildcard_enum_match_arm)] // false positive: non_exhaustive
    fn try_from(value: syn::Type) -> syn::Result<Self> {
        Ok(match value {
            syn::Type::Path(p) => {
                let ty =
                    p.path.segments.last().ok_or_else(|| {
                        syn::Error::new(p.span(), "Empty path")
                    })?;
                match ty.ident.to_string().as_str() {
                    "Dart_Handle" => Self::Handle,
                    "NonNull" => Self::from_non_null_generic(&ty.arguments)?,
                    "DartValueArg" | "DartValue" => Self::ForeignValue,
                    "DartError" => Self::HandlePointer,
                    "i32" => Self::Int32,
                    "i64" => Self::Int64,
                    "i8" => Self::Int8,
                    _ => {
                        return Err(syn::Error::new(
                            ty.ident.span(),
                            "Unsupported type",
                        ));
                    }
                }
            }
            _ => {
                return Err(syn::Error::new(value.span(), "Unsupported type"));
            }
        })
    }
}

/// Generator for the function registration Dart code.
#[derive(Debug)]
struct FnRegistration {
    /// Inputs of the registering function.
    inputs: Vec<DartType>,

    /// Output of the registering function.
    output: DartType,

    /// Name of the registering function.
    name: String,
}

impl TryFrom<FnRegistrationBuilder> for FnRegistration {
    type Error = syn::Error;

    fn try_from(from: FnRegistrationBuilder) -> syn::Result<Self> {
        let inputs = from
            .inputs
            .into_iter()
            .map(|input| {
                if let syn::FnArg::Typed(input) = input {
                    DartType::try_from(*input.ty)
                } else {
                    Err(syn::Error::new(
                        input.span(),
                        "Self types are unsupported here",
                    ))
                }
            })
            .collect::<syn::Result<_>>()?;

        let output = match from.output {
            syn::ReturnType::Default => DartType::Void,
            syn::ReturnType::Type(_, ty) => DartType::try_from(*ty)?,
        };

        Ok(Self {
            inputs,
            output,
            name: from.name.to_string(),
        })
    }
}

/// Builder for a [`FnRegistration`].
#[derive(Debug)]
pub(crate) struct FnRegistrationBuilder {
    /// Inputs of the registering function.
    pub(crate) inputs: Vec<syn::FnArg>,

    /// Output of the registering function.
    pub(crate) output: syn::ReturnType,

    /// Name of the registering function.
    pub(crate) name: syn::Ident,
}

/// Generator of the Dart code registering functions.
#[derive(Debug)]
pub(crate) struct DartCodegen {
    /// FFI name of the registerer function.
    register_fn_name: String,

    /// All generators of the registration Dart code.
    registrators: Vec<FnRegistration>,
}

impl DartCodegen {
    /// Creates a new [`DartCodegen`] for the provided inputs.
    pub(crate) fn new(
        register_fn_name: &syn::Ident,
        builders: Vec<FnRegistrationBuilder>,
    ) -> syn::Result<Self> {
        Ok(Self {
            register_fn_name: register_fn_name.to_string(),
            registrators: builders
                .into_iter()
                .map(FnRegistration::try_from)
                .collect::<syn::Result<_>>()?,
        })
    }

    /// Generates all the needed Dart code of this [`DartCodegen`].
    pub(crate) fn generate(&self) -> Result<String, fmt::Error> {
        let mut out = String::new();

        writeln!(&mut out, "import 'dart:ffi';")?;
        writeln!(&mut out, "import 'package:ffi/ffi.dart';")?;
        writeln!(
            &mut out,
            "import 'package:medea_jason/src/native/ffi/foreign_value.dart';"
        )?;
        writeln!(&mut out, "void registerFunction(DynamicLibrary dl, {{")?;
        self.generate_args(&mut out)?;
        writeln!(&mut out, "}} ) {{")?;
        self.generate_lookup(&mut out)?;
        self.generate_functions_registration(&mut out)?;
        writeln!(&mut out, ");}}")?;

        Ok(out)
    }

    /// Generates arguments of the register function.
    fn generate_args<T: Write>(&self, out: &mut T) -> fmt::Result {
        for f in &self.registrators {
            let mut inputs = String::new();
            for i in &f.inputs {
                inputs.push_str(&format!("{}, ", i.to_ffi_type()));
            }
            if !inputs.is_empty() {
                inputs.truncate(inputs.len() - 2);
            }
            writeln!(
                out,
                "required Pointer<NativeFunction<{ret_ty} \
                Function({inputs})>> {name},",
                ret_ty = f.output.to_ffi_type(),
                inputs = inputs,
                name = f.name.to_camel_case()
            )?;
        }

        Ok(())
    }

    /// Generates function lookup code.
    fn generate_lookup<T: Write>(&self, out: &mut T) -> fmt::Result {
        let mut inputs = String::new();
        for _ in 0..self.registrators.len() {
            inputs.push_str("Pointer, ");
        }
        if !inputs.is_empty() {
            inputs.truncate(inputs.len() - 2);
        }
        writeln!(
            out,
            "dl.lookupFunction<\
                Void Function({inputs}), \
                void Function({inputs})>('{f_name}')(",
            inputs = inputs,
            f_name = self.register_fn_name,
        )
    }

    /// Generates functions registration code.
    fn generate_functions_registration<T: Write>(
        &self,
        out: &mut T,
    ) -> fmt::Result {
        for f in &self.registrators {
            let name = f.name.to_camel_case();
            writeln!(out, "{},", name)?;
        }

        Ok(())
    }
}
