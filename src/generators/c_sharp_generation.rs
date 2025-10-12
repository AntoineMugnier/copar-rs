use crate::{
    code_generation_commons::generate_blank_line, // Removed unused pascal_to_macro_case, pascal_to_snake_case
    model::{ArrayInstanceVariant, OperationParameterVariant}, // Assuming OperationRefTableEntry is part of model
    unirecord::MemberType,
    Model,
};

use stringcase::pascal_case;

mod private {
    use super::*;
    use crate::unirecord::MemberType;

    pub trait Sealed {
        fn generate_cs_file(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_namespace_open(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_namespace_close(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_static_class_open(
            &mut self,
            output_file: &mut impl std::io::Write,
            class_name: &str,
        );
        fn generate_cs_static_class_close(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_operation_id_enum(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_operation_definition(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_enums(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_structs(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_arrays(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_instances(&mut self, output_file: &mut impl std::io::Write);
        fn generate_cs_operation_list(&mut self, output_file: &mut impl std::io::Write);
        fn member_type_to_cs_type_string(member_type: &MemberType) -> String;
        fn fmt_cs_array_value<T, F: Fn(&T) -> String>(array: &[T], format_function: F) -> String;
        fn fmt_cs_array_instance(
            array_instance_variant: &ArrayInstanceVariant,
            array_name: &str,
        ) -> String;
        fn fmt_cs_struct_member(operation_parameter_variant: &OperationParameterVariant) -> String;
    }
}

use private::Sealed;

/// Trait allowing copar model to generate C# code
pub trait CSharpGeneration: private::Sealed {
    fn compute_to_cs(&mut self, output_cs_file: &mut impl std::io::Write);
}

impl CSharpGeneration for Model {
    fn compute_to_cs(&mut self, output_cs_file: &mut impl std::io::Write) {
        self.generate_cs_file(output_cs_file);
    }
}

impl private::Sealed for Model {
    fn generate_cs_file(&mut self, output_file: &mut impl std::io::Write) {
        self.generate_cs_namespace_open(output_file);
        generate_blank_line(output_file);

        self.generate_cs_operation_id_enum(output_file);
        self.generate_cs_operation_definition(output_file);

        self.generate_cs_enums(output_file);
        self.generate_cs_structs(output_file);

        let base_class_name = self.sequence_name.as_ref().map_or("Playdisc", |s| s);
        let static_class_name = format!("{}Constants", pascal_case(base_class_name));

        self.generate_cs_static_class_open(output_file, &static_class_name);
        generate_blank_line(output_file);

        self.generate_cs_arrays(output_file);
        self.generate_cs_instances(output_file);
        self.generate_cs_operation_list(output_file); // Add this call

        self.generate_cs_static_class_close(output_file);
        generate_blank_line(output_file);

        self.generate_cs_namespace_close(output_file);
    }

    fn generate_cs_namespace_open(&mut self, output_file: &mut impl std::io::Write) {
        // Consider making the namespace configurable or derived from sequence_name
        let namespace = self.sequence_name.as_ref().map_or_else(
            || "GeneratedPlaydisc".to_string(),
            |s_name| format!("Generated{}", pascal_case(s_name)),
        );
        write!(output_file, "namespace {}\n{{\n", namespace).unwrap();
    }

    fn generate_cs_namespace_close(&mut self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "}}").unwrap();
    }

    fn generate_cs_static_class_open(
        &mut self,
        output_file: &mut impl std::io::Write,
        class_name: &str,
    ) {
        write!(
            output_file,
            "    public static class {}\n    {{\n",
            class_name
        )
        .unwrap();
    }

    fn generate_cs_static_class_close(&mut self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "    }}").unwrap();
    }

    fn generate_cs_operation_id_enum(&mut self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "    public enum OperationId {{").unwrap();
        // Assuming operation types are derived from the keys of defined_records
        // or from a dedicated list of operation types if not all records are operations.
        // For C#, enum members are typically PascalCase.
        // self.defined_records.keys() are assumed to be PascalCase (e.g. "HciCommand")
        for record_name in self.defined_records.keys() {
            writeln!(output_file, "        {},", record_name).unwrap();
        }
        writeln!(output_file, "    }}").unwrap();
        generate_blank_line(output_file);
    }

    fn generate_cs_operation_definition(&mut self, output_file: &mut impl std::io::Write) {
        writeln!(output_file, "    public readonly struct Operation {{").unwrap();
        writeln!(output_file, "        public readonly OperationId Id;").unwrap();
        write!(output_file, "        public readonly object Variant;\n\n").unwrap();
        writeln!(
            output_file,
            "        public Operation(OperationId id, object variant) {{"
        )
        .unwrap();
        writeln!(output_file, "            Id = id;").unwrap();
        writeln!(output_file, "            Variant = variant;").unwrap();
        writeln!(output_file, "        }}").unwrap();
        writeln!(output_file, "    }}").unwrap();
        generate_blank_line(output_file);
    }

    fn generate_cs_enums(&mut self, output_file: &mut impl std::io::Write) {
        for (enum_type_name, enum_members) in self.defined_enums.iter() {
            writeln!(output_file, "    public enum {} {{", enum_type_name).unwrap();
            for enum_member in enum_members.iter() {
                writeln!(output_file, "        {},", enum_member).unwrap();
            }
            writeln!(output_file, "    }}").unwrap();
            generate_blank_line(output_file); // Add blank line after each enum
        }
    }

    fn generate_cs_structs(&mut self, output_file: &mut impl std::io::Write) {
        for (struct_name, struct_members) in self.defined_records.iter() {
            writeln!(output_file, "    public struct {} {{", struct_name).unwrap();
            for struct_member in struct_members {
                let member_name = &struct_member.member_name; // Assuming PascalCase
                let member_cs_type =
                    Self::member_type_to_cs_type_string(&struct_member.member_type);
                writeln!(
                    output_file,
                    "        public {} {} {{ get; set; }}",
                    member_cs_type, member_name
                )
                .unwrap();
            }
            writeln!(output_file, "    }}").unwrap();
            generate_blank_line(output_file); // Add blank line after each struct
        }
    }

    fn generate_cs_arrays(&mut self, output_file: &mut impl std::io::Write) {
        for (array_variant, array_instance_name) in self.instanciated_arrays.iter() {
            // Assuming array_instance_name is already in PascalCase for C#
            let csharp_array_name = pascal_case(array_instance_name);
            writeln!(
                output_file,
                "        {}",
                Self::fmt_cs_array_instance(array_variant, &csharp_array_name),
            )
            .unwrap();
        }
        if !self.instanciated_arrays.is_empty() {
            generate_blank_line(output_file);
        }
    }

    fn generate_cs_instances(&mut self, output_file: &mut impl std::io::Write) {
        for (operation, operation_instance_name) in self.operation_instances.iter() {
            let operation_type = operation.operation_type.as_str(); // Assumed PascalCase
                                                                    // Assuming operation_instance_name needs conversion to PascalCase for C#
            let csharp_op_instance_name = pascal_case(operation_instance_name);
            write!(
                output_file,
                "        public static readonly {} {} = new {} {{ ",
                operation_type, csharp_op_instance_name, operation_type
            )
            .unwrap();
            let nb_parameters = operation.parameters.len();

            for (index, operation_parameter) in operation.parameters.iter().enumerate() {
                // fmt_cs_struct_member assumes param.name is PascalCase
                let param_assignment = Self::fmt_cs_struct_member(operation_parameter);
                write!(output_file, "{}", param_assignment).unwrap();
                if index < nb_parameters - 1 {
                    write!(output_file, ", ").unwrap();
                }
            }
            writeln!(output_file, " }};").unwrap();
        }
        if !self.operation_instances.is_empty() {
            generate_blank_line(output_file);
        }
    }

    fn generate_cs_operation_list(&mut self, output_file: &mut impl std::io::Write) {
        let array_name = pascal_case(self.sequence_name.as_ref().unwrap());

        writeln!(
            output_file,
            "        public static readonly Operation[] {} = {{",
            array_name
        )
        .unwrap();

        for op_ref in self.operation_ref_table.iter() {
            // op_ref.operation_type is the struct name (e.g., "HciCommand"), used for OperationId
            // op_ref.operation_variant_ref_name is the instance name (e.g., "hci_command_0")
            // This instance name must match a C# PascalCase static field.
            let operation_id_member = &op_ref.operation_type; // Assumed PascalCase
            let csharp_instance_name = pascal_case(&op_ref.operation_variant_ref_name);

            writeln!(
                output_file,
                "            new Operation(OperationId.{}, {}),",
                operation_id_member, csharp_instance_name
            )
            .unwrap();
        }

        writeln!(output_file, "        }};").unwrap();
        if !self.operation_ref_table.is_empty() {
            generate_blank_line(output_file);
        }
    }

    fn member_type_to_cs_type_string(member_type: &MemberType) -> String {
        match member_type {
            MemberType::X8 | MemberType::U8 => "byte".to_string(),
            MemberType::X16 | MemberType::U16 => "ushort".to_string(),
            MemberType::X32 | MemberType::U32 => "uint".to_string(),
            MemberType::X64 | MemberType::U64 => "ulong".to_string(),
            MemberType::I8 => "sbyte".to_string(),
            MemberType::I16 => "short".to_string(),
            MemberType::I32 => "int".to_string(),
            MemberType::I64 => "long".to_string(),
            MemberType::F32 => "float".to_string(),
            MemberType::F64 => "double".to_string(),
            MemberType::ArrayOfX8 | MemberType::ArrayOfU8 => "byte[]".to_string(),
            MemberType::ArrayOfX16 | MemberType::ArrayOfU16 => "ushort[]".to_string(),
            MemberType::ArrayOfX32 | MemberType::ArrayOfU32 => "uint[]".to_string(),
            MemberType::ArrayOfX64 | MemberType::ArrayOfU64 => "ulong[]".to_string(),
            MemberType::ArrayOfI8 => "sbyte[]".to_string(),
            MemberType::ArrayOfI16 => "short[]".to_string(),
            MemberType::ArrayOfI32 => "int[]".to_string(),
            MemberType::ArrayOfI64 => "long[]".to_string(),
            MemberType::ArrayOfF32 => "float[]".to_string(),
            MemberType::ArrayOfF64 => "double[]".to_string(),
            MemberType::Bool => "bool".to_string(),
            MemberType::Identifier(enum_type) => enum_type.clone(), // Assumed PascalCase
        }
    }

    fn fmt_cs_array_value<T, F: Fn(&T) -> String>(array: &[T], format_function: F) -> String {
        let mut ret = String::from("{ ");
        let nb_elements = array.len();
        for (index, element) in array.iter().enumerate() {
            ret += &format_function(element);
            if index < nb_elements - 1 {
                ret += ", ";
            }
        }
        ret += " }";
        ret
    }

    fn fmt_cs_array_instance(
        array_instance_variant: &ArrayInstanceVariant,
        array_name: &str, // This is now expected to be PascalCase
    ) -> String {
        let (array_value, cs_type) = match array_instance_variant {
            ArrayInstanceVariant::X8(array) => (
                Self::fmt_cs_array_value(array, |e| format!("0x{e:x}")),
                "byte",
            ),
            ArrayInstanceVariant::U8(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "byte")
            }
            ArrayInstanceVariant::X16(array) => (
                Self::fmt_cs_array_value(array, |e| format!("0x{e:x}")),
                "ushort",
            ),
            ArrayInstanceVariant::U16(array) => (
                Self::fmt_cs_array_value(array, |e| format!("{e}")),
                "ushort",
            ),
            ArrayInstanceVariant::X32(array) => (
                Self::fmt_cs_array_value(array, |e| format!("0x{e:x}")),
                "uint",
            ),
            ArrayInstanceVariant::U32(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "uint")
            }
            ArrayInstanceVariant::X64(array) => (
                Self::fmt_cs_array_value(array, |e| format!("0x{e:x}")),
                "ulong",
            ),
            ArrayInstanceVariant::U64(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "ulong")
            }
            ArrayInstanceVariant::I8(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "sbyte")
            }
            ArrayInstanceVariant::I16(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "short")
            }
            ArrayInstanceVariant::I32(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "int")
            }
            ArrayInstanceVariant::I64(array) => {
                (Self::fmt_cs_array_value(array, |e| format!("{e}")), "long")
            }
            _ => {
                unimplemented!("Array type in ArrayInstanceVariant not supported for C# generation")
            }
        };
        format!("public static readonly {cs_type}[] {array_name} =  {array_value};")
    }

    fn fmt_cs_struct_member(operation_parameter_variant: &OperationParameterVariant) -> String {
        // param.name is assumed to be PascalCase from the model or converted before this call
        // For struct member assignment, param.name should be PascalCase.
        // If param.name in OperationParameterVariant is snake_case, it needs conversion here or earlier.
        // Assuming param.name is already C# style (PascalCase).
        match operation_parameter_variant {
            OperationParameterVariant::X8(param) => format!("{} = 0x{:x}", param.name, param.value),
            OperationParameterVariant::U8(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::I8(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::X16(param) => {
                format!("{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::U16(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::I16(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::X32(param) => {
                format!("{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::U32(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::I32(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::X64(param) => {
                format!("{} = 0x{:x}", param.name, param.value)
            }
            OperationParameterVariant::U64(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::I64(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::F32(param) => format!("{} = {}f", param.name, param.value), // C# float literal
            OperationParameterVariant::F64(param) => format!("{} = {}", param.name, param.value),
            OperationParameterVariant::ArrayOfX8(param)
            | OperationParameterVariant::ArrayOfU8(param)
            | OperationParameterVariant::ArrayOfI8(param)
            | OperationParameterVariant::ArrayOfX16(param)
            | OperationParameterVariant::ArrayOfU16(param)
            | OperationParameterVariant::ArrayOfI16(param)
            | OperationParameterVariant::ArrayOfX32(param)
            | OperationParameterVariant::ArrayOfU32(param)
            | OperationParameterVariant::ArrayOfI32(param)
            | OperationParameterVariant::ArrayOfX64(param)
            | OperationParameterVariant::ArrayOfU64(param)
            | OperationParameterVariant::ArrayOfI64(param)
            | OperationParameterVariant::ArrayOfF32(param)
            | OperationParameterVariant::ArrayOfF64(param) => {
                // param.value is the array instance name, needs to be PascalCase
                let csharp_array_ref_name = pascal_case(&param.value);
                format!("{} = {}", param.name, csharp_array_ref_name)
            }
            OperationParameterVariant::Bool(param) => {
                let val_str = if param.value { "true" } else { "false" }; // C# bool literals
                format!("{} = {}", param.name, val_str)
            }
            OperationParameterVariant::Identifier(param) => {
                // param.enum_type and param.value are assumed to be PascalCase
                format!("{} = {}.{}", param.name, param.enum_type, param.value)
            }
        }
    }
}
