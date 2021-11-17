use std::{
    convert::{Infallible, TryFrom},
    fmt,
    fmt::Formatter,
    hint::unreachable_unchecked,
};

use syn::{
    Error, FnArg, GenericArgument, Ident, PathArguments, ReturnType, Type,
};

#[derive(Debug, Clone, Copy)]
pub enum DartType {
    Handle,
    StringPointer,
    Int8,
    Int32,
    Pointer,
    Void,
}

impl DartType {
    pub fn is_int(&self) -> bool {
        match self {
            Self::Int8 | Self::Int32 => true,
            _ => false,
        }
    }

    pub fn to_ffi_type(&self) -> &'static str {
        match self {
            Self::Handle => "handle",
            Self::StringPointer => "Pointer<Utf8>",
            Self::Int8 => "Int8",
            Self::Int32 => "Int32",
            Self::Pointer => "Pointer",
            Self::Void => "Void",
        }
    }

    pub fn to_dart_type(&self) -> &'static str {
        match self {
            Self::Handle => "Object",
            Self::StringPointer => "Pointer<Utf8>",
            Self::Int8 | Self::Int32 => "int",
            Self::Pointer => "Pointer",
            Self::Void => "void",
        }
    }
}

impl TryFrom<Type> for DartType {
    type Error = Infallible;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        let res = match value {
            Type::Path(path) => {
                let ty = path.path.segments.last().unwrap();
                let stringified_ty = ty.ident.to_string();
                match stringified_ty.as_str() {
                    "Dart_Handle" => Self::Handle,
                    "NonNull" => match &ty.arguments {
                        PathArguments::AngleBracketed(args) => {
                            let ty = args.args.last().unwrap();
                            match ty {
                                GenericArgument::Type(ty) => match ty {
                                    Type::Path(path) => {
                                        let ty =
                                            path.path.segments.last().unwrap();
                                        match ty.ident.to_string().as_str() {
                                            "c_char" => Self::StringPointer,
                                            _ => Self::Pointer,
                                        }
                                    }
                                    _ => unreachable!("1"),
                                },
                                _ => unreachable!("2"),
                            }
                        }
                        _ => unreachable!("3"),
                    },
                    "i32" => Self::Int32,
                    "i8" => Self::Int8,
                    _ => unreachable!("{}", stringified_ty),
                }
            }
            _ => unreachable!("5"),
        };

        Ok(res)
    }
}

#[derive(Debug)]
struct FnRegistration {
    inputs: Vec<DartType>,
    output: DartType,
    name: String,
}

impl FnRegistration {
    fn is_returns_int(&self) -> bool {
        self.output.is_int()
    }
}

#[derive(Debug)]
pub struct DartCodegen {
    register_fn_name: String,
    registrators: Vec<FnRegistration>,
}

#[derive(Debug)]
pub struct FnRegistrationBuilder {
    pub inputs: Vec<FnArg>,
    pub output: ReturnType,
    pub name: Ident,
}

impl TryFrom<FnRegistrationBuilder> for FnRegistration {
    type Error = Error;

    fn try_from(from: FnRegistrationBuilder) -> Result<Self, Self::Error> {
        let mut inputs = Vec::new();
        for input in from.inputs {
            match input {
                FnArg::Typed(input) => {
                    inputs.push(DartType::try_from(*input.ty).unwrap());
                }
                _ => unreachable!("6"),
            }
        }
        let output = match from.output {
            ReturnType::Default => DartType::Void,
            ReturnType::Type(_, ty) => DartType::try_from(*ty).unwrap(),
        };
        let name = from.name.to_string();

        Ok(Self {
            inputs,
            output,
            name,
        })
    }
}

struct FormattedGenerator {
    generated: String,
    space_count: u32,
}

impl FormattedGenerator {
    pub fn new() -> Self {
        Self {
            generated: String::new(),
            space_count: 0,
        }
    }

    pub fn push_line(&mut self, line: &str) {
        for _ in 0..self.space_count {
            self.generated.push(' ');
        }
        self.generated.push_str(line);
        self.generated.push('\n');
    }

    pub fn tab(&mut self) {
        self.space_count += 4;
    }

    pub fn untab(&mut self) {
        if self.space_count >= 4 {
            self.space_count -= 4;
        } else {
            self.space_count = 0;
        }
    }
}

impl DartCodegen {
    pub fn new(
        register_fn_name: Ident,
        builders: Vec<FnRegistrationBuilder>,
    ) -> Result<Self, Error> {
        let mut registrators = Vec::new();
        for f in builders {
            registrators.push(FnRegistration::try_from(f).unwrap());
        }

        let this = Self {
            register_fn_name: register_fn_name.to_string(),
            registrators,
        };

        Ok(this)
    }

    fn generate_args(&self, g: &mut FormattedGenerator) {
        for f in &self.registrators {
            let mut inputs = String::new();
            for i in &f.inputs {
                inputs.push_str(&format!("{}, ", i.to_dart_type()));
            }
            inputs.truncate(inputs.len() - 2);
            g.push_line(&format!(
                "required {ret_ty} Function({inputs}) {name},",
                ret_ty = f.output.to_dart_type(),
                inputs = inputs,
                name = f.name,
            ));
        }
    }

    fn generate_lookup(&self, g: &mut FormattedGenerator) {
        let mut inputs = String::new();
        for _ in 0..self.registrators.len() {
            inputs.push_str("Pointer, ");
        }
        inputs.truncate(inputs.len() - 2);
        g.push_line(
            &format!(
                "dl.lookupFunction<Void Function({inputs}), void Function({inputs})>('{f_name}')(",
                inputs=inputs,
                f_name=self.register_fn_name,
            )
        );
    }

    fn generate_functions_registration(&self, g: &mut FormattedGenerator) {
        for f in &self.registrators {
            let out_type = f.output.to_ffi_type();
            let mut inputs = String::new();
            for i in &f.inputs {
                inputs.push_str(&format!("{}, ", i.to_ffi_type()));
            }
            inputs.truncate(inputs.len() - 2);
            let name = f.name.to_string();
            g.push_line(&format!(
                "Pointer.fromFunction<{out_ty} Function({inputs})>({name}),",
                out_ty = out_type,
                inputs = inputs,
                name = name,
            ));
        }
    }

    pub fn generate(&self) -> String {
        let mut g = FormattedGenerator::new();
        g.push_line("import 'dart:ffi';");
        g.push_line("import 'package:ffi/ffi.dart';");
        g.push_line("");
        g.push_line("void registerFunction(");
        g.tab();
        g.push_line("DynamicLibrary dl,");
        g.push_line("{");
        g.tab();
        self.generate_args(&mut g);
        g.untab();
        g.push_line("}");
        g.untab();
        g.push_line(") {");
        g.tab();
        self.generate_lookup(&mut g);
        g.tab();
        self.generate_functions_registration(&mut g);
        g.untab();
        g.push_line(");");
        g.untab();
        g.push_line("}");

        g.generated
    }
}
