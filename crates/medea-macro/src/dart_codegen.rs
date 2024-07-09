//! Dart side register functions generator of `#[dart_bridge]` macro.

use std::fmt::{self, Write};

use inflector::Inflector;
use syn::spanned::Spanned as _;

/// Types that can be passed through FFI.
#[derive(Debug, Clone, Copy)]
pub(crate) enum DartType {
    /// Type which indicates that function doesn't return anything.
    ///
    /// `void` keyword in Dart.
    Void,

    /// Boolean value.
    ///
    /// Represents [Bool] on the Dart side.
    ///
    /// [Bool]: https://api.dart.dev/stable/dart-ffi/Bool-class.html
    Bool,

    /// 8-bit integer.
    ///
    /// Represents [Int8] on the Dart side.
    ///
    /// [Int8]: https://api.dart.dev/stable/dart-ffi/Int8-class.html
    Int8,

    /// 8-bit unsigned integer.
    ///
    /// Represents [Uint8] on the Dart side.
    ///
    /// [Uint8]: https://api.dart.dev/stable/dart-ffi/Uint8-class.html
    Uint8,

    /// 32-bit integer.
    ///
    /// Represents [Int32] on the Dart side.
    ///
    /// [Int32]: https://api.dart.dev/stable/dart-ffi/Int32-class.html
    Int32,

    /// 32-bit unsigned integer.
    ///
    /// Represents [Uint32] on the Dart side.
    ///
    /// [Uint32]: https://api.dart.dev/stable/dart-ffi/Uint32-class.html
    Uint32,

    /// 64-bit integer.
    ///
    /// Represents [Int64] on the Dart side.
    ///
    /// [Int64]: https://api.dart.dev/stable/dart-ffi/Int64-class.html
    Int64,

    /// 64-bit unsigned integer.
    ///
    /// Represents [Uint64] on the Dart side.
    ///
    /// [Uint64]: https://api.dart.dev/stable/dart-ffi/Uint64-class.html
    Uint64,

    /// Pointer to the Dart `Object`.
    ///
    /// Represents a [Handle] on the Dart side.
    ///
    /// [Handle]: https://api.dart.dev/stable/dart-ffi/Handle-class.html
    Handle,

    /// Pointer to the Rust structure.
    ///
    /// Represents [Pointer] on the Dart side.
    ///
    /// [Pointer]: https://api.dart.dev/stable/dart-ffi/Pointer-class.html
    Pointer,

    /// Pointer to a boxed Dart `Object`.
    ///
    /// Represents [Pointer<Handle>][0] on the Dart side.
    ///
    /// [0]: https://api.dart.dev/stable/dart-ffi/Pointer-class.html
    HandlePointer,

    /// [`c_char`] pointer.
    ///
    /// Represents [Pointer<Utf8>][0] on the Dart side.
    ///
    /// [0]: https://pub.dev/documentation/ffi/latest/ffi/Utf8-class.html
    StringPointer,

    /// `DartValue` FFI structure which adds ability to cast more complex
    /// types.
    ForeignValue,
}

impl DartType {
    /// Converts this [`DartType`] to the Dart side FFI type.
    pub(crate) const fn to_ffi_type(self) -> &'static str {
        match self {
            Self::Void => "Void",
            Self::Bool => "Bool",
            Self::Int8 => "Int8",
            Self::Uint8 => "Uint8",
            Self::Int32 => "Int32",
            Self::Uint32 => "Uint32",
            Self::Int64 => "Int64",
            Self::Uint64 => "Uint64",
            Self::Handle => "Handle",
            Self::Pointer => "Pointer",
            Self::HandlePointer => "Pointer<Handle>",
            Self::StringPointer => "Pointer<Utf8>",
            Self::ForeignValue => "ForeignValue",
        }
    }

    /// Converts this [`DartType`] to the string representation that can be
    /// used in Dart code.
    pub(crate) const fn to_dart_type(self) -> &'static str {
        match self {
            Self::Void => "void",
            Self::Bool => "bool",
            Self::Int8
            | Self::Uint8
            | Self::Int32
            | Self::Uint32
            | Self::Int64
            | Self::Uint64 => "int",
            Self::Handle => "Object",
            Self::Pointer => "Pointer",
            Self::HandlePointer => "Pointer<Handle>",
            Self::StringPointer => "Pointer<Utf8>",
            Self::ForeignValue => "ForeignValue",
        }
    }

    /// String representing the default value of this [`DartType`] intended to
    /// be used in return statement if the function have completed with error.
    pub(crate) const fn default_value(self) -> &'static str {
        match self {
            Self::Void => "",
            Self::Bool => "false",
            Self::Int8
            | Self::Uint8
            | Self::Int32
            | Self::Uint32
            | Self::Int64
            | Self::Uint64
            | Self::Handle
            | Self::HandlePointer
            | Self::ForeignValue => "0",
            Self::StringPointer | Self::Pointer => "Pointer.fromAddress(0)",
        }
    }

    /// String representing the value that should be passed as
    /// `exceptionalReturn` argument of [Pointer.fromFunction][1] method.
    ///
    /// [1]: https://api.dart.dev/stable/dart-ffi/Pointer/fromFunction.html
    pub(crate) const fn exceptional_return(self) -> &'static str {
        match self {
            Self::Bool => "false",
            Self::Int8
            | Self::Uint8
            | Self::Int32
            | Self::Uint32
            | Self::Int64
            | Self::Uint64
            | Self::HandlePointer
            | Self::ForeignValue => "0",
            Self::StringPointer | Self::Pointer | Self::Handle | Self::Void => {
                ""
            }
        }
    }

    /// Parses [`ptr::NonNull`] [`DartType`] from the provided
    /// [`PathArguments`].
    ///
    /// [`ptr::NonNull`]: std::ptr::NonNull
    pub(crate) fn from_non_null_generic(
        args: &syn::PathArguments,
    ) -> syn::Result<Self> {
        let syn::PathArguments::AngleBracketed(bracketed) = args else {
            return Err(syn::Error::new(
                args.span(),
                "Unsupported `NonNull` path arguments",
            ));
        };

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
            | syn::GenericArgument::Const(_)
            | syn::GenericArgument::AssocType(_)
            | syn::GenericArgument::AssocConst(_)
            | syn::GenericArgument::Constraint(_) => Err(syn::Error::new(
                bracketed.span(),
                "Unsupported generic argument",
            )),
            _ => Err(syn::Error::new(
                bracketed.span(),
                "Unsupported unknown generic argument",
            )),
        }
    }
}

impl TryFrom<syn::Type> for DartType {
    type Error = syn::Error;

    fn try_from(value: syn::Type) -> syn::Result<Self> {
        Ok(match &value {
            syn::Type::Path(p) => {
                let ty =
                    p.path.segments.last().ok_or_else(|| {
                        syn::Error::new(p.span(), "Empty path")
                    })?;
                match ty.ident.to_string().as_str() {
                    "bool" => Self::Bool,
                    "i8" => Self::Int8,
                    "u8" => Self::Uint8,
                    "i32" => Self::Int32,
                    "u32" => Self::Uint32,
                    "i64" => Self::Int64,
                    "u64" => Self::Uint64,
                    "Dart_Handle" => Self::Handle,
                    "NonNull" => Self::from_non_null_generic(&ty.arguments)?,
                    "DartValueArg" | "DartValue" => Self::ForeignValue,
                    "DartError" => Self::HandlePointer,
                    _ => {
                        return Err(syn::Error::new(
                            ty.ident.span(),
                            "Unsupported type",
                        ));
                    }
                }
            }
            syn::Type::Tuple(t) => {
                if t.elems.is_empty() {
                    Self::Void
                } else {
                    return Err(syn::Error::new(
                        value.span(),
                        "Unsupported type",
                    ));
                }
            }
            syn::Type::Array(_)
            | syn::Type::BareFn(_)
            | syn::Type::Group(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Paren(_)
            | syn::Type::Ptr(_)
            | syn::Type::Reference(_)
            | syn::Type::Slice(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Verbatim(_) => {
                return Err(syn::Error::new(value.span(), "Unsupported type"));
            }
            _ => return Err(syn::Error::new(value.span(), "Unknown type")),
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

    /// Name of the error setter extern function.
    error_setter_fn_name: String,
}

impl TryFrom<FnRegistrationBuilder> for FnRegistration {
    type Error = syn::Error;

    fn try_from(from: FnRegistrationBuilder) -> syn::Result<Self> {
        let inputs = from
            .inputs
            .into_iter()
            .map(|input| {
                if let syn::FnArg::Typed(arg) = input {
                    DartType::try_from(*arg.ty)
                } else {
                    Err(syn::Error::new(
                        input.span(),
                        "Self types are unsupported here",
                    ))
                }
            })
            .collect::<syn::Result<_>>()?;

        Ok(Self {
            inputs,
            output: DartType::try_from(from.output)?,
            name: from.name.to_string(),
            error_setter_fn_name: from.error_setter_ident.to_string(),
        })
    }
}

/// Builder for a [`FnRegistration`].
#[derive(Debug)]
pub(crate) struct FnRegistrationBuilder {
    /// Inputs of the registering function.
    pub(crate) inputs: Vec<syn::FnArg>,

    /// Output of the registering function.
    pub(crate) output: syn::Type,

    /// Name of the registering function.
    pub(crate) name: syn::Ident,

    /// [`syn::Ident`] of the extern function that saves error in its slot.
    pub(crate) error_setter_ident: syn::Ident,
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

        writeln!(out, "import 'dart:ffi';")?;
        writeln!(out, "import 'package:ffi/ffi.dart';")?;
        writeln!(
            out,
            "import 'package:medea_jason/src/native/ffi/foreign_value.dart';"
        )?;

        writeln!(out, "typedef _ErrorSetterFnC = Void Function(Handle);")?;
        writeln!(out, "typedef _ErrorSetterFnDart = void Function(Object);\n")?;

        self.generate_fns_storage(&mut out)?;
        writeln!(out)?;

        self.generate_set_errors_storage(&mut out)?;
        writeln!(out)?;

        writeln!(out, "void registerFunction(DynamicLibrary dl, {{")?;
        self.generate_args(&mut out)?;
        writeln!(out, "}} ) {{")?;

        // Save provided Dart functions. E.g.:
        //
        // _iceConnectionState = iceConnectionState;
        // _onConnectionStateChange = onConnectionStateChange;
        // _connectionState = connectionState;
        for f in &self.registrators {
            writeln!(out, "_{name} = {name};", name = f.name.to_camel_case())?;
        }
        writeln!(out)?;

        self.gen_set_error_lookup(&mut out)?;
        writeln!(out)?;

        self.generate_pointers_to_proxy_fns(&mut out)?;
        writeln!(out)?;

        self.generate_lookup(&mut out)?;
        writeln!(out)?;

        self.generate_functions_registration(&mut out)?;
        writeln!(out, ");}}")?;

        self.generate_proxy_fns(&mut out)?;

        Ok(out)
    }

    /// Generates arguments of the register function.
    fn generate_args<T: Write>(&self, out: &mut T) -> fmt::Result {
        for f in &self.registrators {
            let mut inputs = String::new();
            for i in &f.inputs {
                write!(inputs, "{}, ", i.to_dart_type())?;
            }
            if !inputs.is_empty() {
                inputs.truncate(inputs.len() - 2);
            }
            writeln!(
                out,
                "required {ret_ty}  Function({inputs}) {name},",
                ret_ty = f.output.to_dart_type(),
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
            writeln!(out, "{name}_native,")?;
        }

        Ok(())
    }

    /// Generates variables that stores Dart functions that will be called
    /// by Rust side.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// int Function(Object)? _iceConnectionState;
    /// void Function(Object, Object)? _onConnectionStateChange;
    /// Pointer Function(Object)? _connectionState;
    /// ```
    fn generate_fns_storage<T: Write>(&self, out: &mut T) -> fmt::Result {
        for f in &self.registrators {
            let mut inputs = String::new();

            for i in &f.inputs {
                write!(inputs, "{}, ", i.to_dart_type())?;
            }
            if !inputs.is_empty() {
                inputs.truncate(inputs.len() - 2);
            }

            writeln!(
                out,
                "{ret_ty} Function({inputs})? _{name};",
                ret_ty = f.output.to_dart_type(),
                name = f.name.to_camel_case()
            )?;
        }

        Ok(())
    }

    /// Generates variables that store Dart bindings to Rust functions that
    /// save execution errors.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// _ErrorSetterFnDart? _peer_connection__rollback__set_error;
    /// _ErrorSetterFnDart? _peer_connection__get_stats__set_error;
    /// _ErrorSetterFnDart? _peer_connection__on_track__set_error;
    /// ```
    fn generate_set_errors_storage<T: Write>(
        &self,
        out: &mut T,
    ) -> fmt::Result {
        for f in &self.registrators {
            writeln!(
                out,
                "_ErrorSetterFnDart? _{name};",
                name = f.error_setter_fn_name
            )?;
        }

        Ok(())
    }

    /// Generates variables that store Dart bindings to Rust functions that
    /// save execution errors.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// Pointer<NativeFunction<Int32 Function(Handle)>>
    ///     iceConnectionState_native =
    ///         Pointer.fromFunction(_iceConnectionStateProxy, 0);
    ///
    /// Pointer<NativeFunction<Void Function(Handle, Handle)>>
    ///       onConnectionStateChange_native = Pointer.fromFunction(
    ///     _onConnectionStateChangeProxy,
    ///   );
    /// ```
    fn generate_pointers_to_proxy_fns<T: Write>(
        &self,
        out: &mut T,
    ) -> fmt::Result {
        for f in &self.registrators {
            let mut inputs = String::new();
            for i in &f.inputs {
                write!(inputs, "{}, ", i.to_ffi_type())?;
            }
            if !inputs.is_empty() {
                inputs.truncate(inputs.len() - 2);
            }
            writeln!(
                out,
                "Pointer<NativeFunction<{ret_ty} Function({inputs})>> \
                 {name}_native = Pointer.fromFunction(_{name}Proxy,{exc_ret});",
                ret_ty = f.output.to_ffi_type(),
                name = f.name.to_camel_case(),
                exc_ret = f.output.exceptional_return(),
            )?;
        }

        Ok(())
    }

    /// Generates variables that store Dart bindings to Rust functions that
    /// save execution errors.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// _ErrorSetterFnDart? _peer_connection__rollback__set_error;
    /// _ErrorSetterFnDart? _peer_connection__get_stats__set_error;
    /// _ErrorSetterFnDart? _peer_connection__on_track__set_error;
    /// ```
    fn gen_set_error_lookup<T: Write>(&self, out: &mut T) -> fmt::Result {
        for f in &self.registrators {
            writeln!(
                out,
                "_{name} = dl.lookupFunction<\
                    _ErrorSetterFnC,\
                    _ErrorSetterFnDart>('{name}');",
                name = f.error_setter_fn_name
            )?;
        }

        Ok(())
    }

    /// Generates proxy functions that wrap calls in a try-catch blocks that
    /// pass any exceptions to Rust.
    ///
    /// # Example of generated code
    ///
    /// ```ignore
    /// int _iceConnectionStateProxy(Object arg0) {
    ///   try {
    ///     return _iceConnectionState!(arg0);
    ///   } catch (e) {
    ///     _peer_connection__ice_connection_state__set_error!(e);
    ///     return 0;
    ///   }
    /// }
    /// ```
    fn generate_proxy_fns<T: Write>(&self, out: &mut T) -> fmt::Result {
        for f in &self.registrators {
            let mut inputs = String::new();
            let mut arg_names = String::new();

            for (i, t) in f.inputs.iter().enumerate() {
                write!(inputs, "{arg_t} arg{i}, ", arg_t = t.to_dart_type())?;
                write!(arg_names, "arg{i}, ")?;
            }
            if !inputs.is_empty() {
                inputs.truncate(inputs.len() - 2);
                arg_names.truncate(arg_names.len() - 2);
            }

            writeln!(
                out,
                "{ret_ty} _{name}Proxy({inputs}) {{\
                     try {{
                        return _{name}!({arg_names}); \
                     }} catch (e) {{ \
                        _{error_setter}!(e); \
                        return {return_value};
                     }} \
                }}",
                ret_ty = f.output.to_dart_type(),
                name = f.name.to_camel_case(),
                error_setter = f.error_setter_fn_name,
                inputs = inputs,
                return_value = f.output.default_value(),
            )?;
        }

        Ok(())
    }
}
