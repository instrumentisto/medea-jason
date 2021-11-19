//! Implementation of the Dart side register functions generator based on
//! `#[dart_bridge]` macro.

use std::{
    convert::TryFrom,
    fmt::{self, Write},
};

use inflector::Inflector;
use syn::{
    spanned::Spanned, Error, FnArg, GenericArgument, Ident, PathArguments,
    ReturnType, Type,
};

/// Types that can be passed through FFI.
#[derive(Debug, Clone, Copy)]
pub enum DartType {
    /// Pointer to the Dart `Object`.
    ///
    /// Represents [Handle] on the Dart side.
    ///
    /// [Handle]: https://api.dart.dev/stable/dart-ffi/Handle-class.html
    Handle,

    /// [`c_char`] pointer.
    ///
    /// Represents [Pointer<Utf8>] on the Dart side.
    ///
    /// [Pointer<Utf8>]:
    /// https://pub.dev/documentation/ffi/latest/ffi/Utf8-class.html
    StringPointer,

    /// Pointer to the boxed Dart `Object`.
    ///
    /// Represents [Pointer<Handle>] on the Dart side.
    ///
    /// [Pointer<Handle>]:
    /// https://api.dart.dev/stable/dart-ffi/Pointer-class.html
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

    /// Type which indicates that function doesn't returns enything.
    ///
    /// `void` keyword in Dart.
    Void,

    /// `DartValue` FFI structure which adds ability to cast more complex
    /// types.
    ForeignValue,
}

impl DartType {
    /// Converts this [`DartType`] to the Dart side FFI type.
    pub fn to_ffi_type(self) -> &'static str {
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
    pub fn from_non_null_generic(args: &PathArguments) -> Result<Self, Error> {
        if let PathArguments::AngleBracketed(args) = args {
            match args
                .args
                .last()
                .ok_or_else(|| Error::new(args.span(), "Empty generics list"))?
            {
                GenericArgument::Type(Type::Path(path)) => {
                    let segment =
                        path.path.segments.last().ok_or_else(|| {
                            Error::new(path.span(), "Empty generic path")
                        })?;
                    Ok(if segment.ident.to_string().as_str() == "c_char" {
                        Self::StringPointer
                    } else {
                        Self::Pointer
                    })
                }
                _ => {
                    Err(Error::new(args.span(), "Unsupported generic argument"))
                }
            }
        } else {
            Err(Error::new(
                args.span(),
                "Unsupported NonNull path arguments",
            ))
        }
    }
}

impl TryFrom<Type> for DartType {
    type Error = Error;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        let res = match value {
            Type::Path(path) => {
                let ty = path
                    .path
                    .segments
                    .last()
                    .ok_or_else(|| Error::new(path.span(), "Empty path"))?;
                match ty.ident.to_string().as_str() {
                    "Dart_Handle" => Self::Handle,
                    "NonNull" => Self::from_non_null_generic(&ty.arguments)?,
                    "DartValueArg" | "DartValue" => Self::ForeignValue,
                    "DartError" => Self::HandlePointer,
                    "i32" => Self::Int32,
                    "i64" => Self::Int64,
                    "i8" => Self::Int8,
                    _ => {
                        return Err(Error::new(
                            ty.ident.span(),
                            "Unsupported type",
                        ));
                    }
                }
            }
            _ => {
                return Err(Error::new(value.span(), "Unsupported type"));
            }
        };

        Ok(res)
    }
}

/// Generator for the fn registration Dart code.
#[derive(Debug)]
struct FnRegistration {
    /// Inputs of the registering function.
    inputs: Vec<DartType>,

    /// Output of the registering function.
    output: DartType,

    /// Name of the registering function.
    name: String,
}

/// Generator for the functions regiterer Dart code.
#[derive(Debug)]
pub struct DartCodegen {
    /// FFI name of the registerer function.
    register_fn_name: String,

    /// All generators of the registration Dart code.
    registrators: Vec<FnRegistration>,
}

/// Builder for the [`FnRegistration`].
#[derive(Debug)]
pub struct FnRegistrationBuilder {
    /// Inputs of the registering function.
    pub inputs: Vec<FnArg>,

    /// Output of the registering function.
    pub output: ReturnType,

    /// Name of the registering function.
    pub name: Ident,
}

impl TryFrom<FnRegistrationBuilder> for FnRegistration {
    type Error = Error;

    fn try_from(from: FnRegistrationBuilder) -> Result<Self, Self::Error> {
        let mut inputs = Vec::new();
        for input in from.inputs {
            if let FnArg::Typed(input) = input {
                inputs.push(DartType::try_from(*input.ty)?);
            } else {
                return Err(Error::new(
                    input.span(),
                    "Self types are unsupported here",
                ));
            }
        }
        let output = match from.output {
            ReturnType::Default => DartType::Void,
            ReturnType::Type(_, ty) => DartType::try_from(*ty)?,
        };
        let name = from.name.to_string();

        Ok(Self {
            inputs,
            output,
            name,
        })
    }
}

impl DartCodegen {
    /// Creates new [`DartCodegen`] for the provided inputs.
    pub fn new(
        register_fn_name: &Ident,
        builders: Vec<FnRegistrationBuilder>,
    ) -> Result<Self, Error> {
        let mut registrators = Vec::new();
        for f in builders {
            registrators.push(FnRegistration::try_from(f)?);
        }

        let this = Self {
            register_fn_name: register_fn_name.to_string(),
            registrators,
        };

        Ok(this)
    }

    /// Generates all needed Dart code of this [`DartCodegen`].
    pub fn generate(&self) -> Result<String, fmt::Error> {
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

    /// Generates fn lookup code.
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
            let name = f.name.to_string().to_camel_case();
            writeln!(out, "{},", name)?;
        }

        Ok(())
    }
}
