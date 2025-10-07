use crate::{
    model::{ArrayInstanceVariant, OperationParameterVariant},
    unirecord::MemberType,
    Model,
};
use std::{fs::File, io::Write};

impl ToString for MemberType {
    fn to_string(&self) -> String {
        let ret;
        match self {
            MemberType::X8 => ret = "uint8_t",
            MemberType::X16 => ret = "uint16_t",
            MemberType::X32 => ret = "uint32_t",
            MemberType::X64 => ret = "uint64_t",
            MemberType::U8 => ret = "uint8_t",
            MemberType::U16 => ret = "uint8_t",
            MemberType::U32 => ret = "uint32_t",
            MemberType::U64 => ret = "uint64_t",
            MemberType::I8 => ret = "int8_t",
            MemberType::I16 => ret = "int16_t",
            MemberType::I32 => ret = "int32_t",
            MemberType::I64 => ret = "int64_t",
            MemberType::F32 => ret = "float",
            MemberType::F64 => ret = "double",
            MemberType::ArrayOfX8 => ret = "uint8_t* const",
            MemberType::ArrayOfX16 => ret = "uint16_t* const",
            MemberType::ArrayOfX32 => ret = "uint32_t* const",
            MemberType::ArrayOfX64 => ret = "uint64_t* const",
            MemberType::ArrayOfU8 => ret = "uint8_t* const",
            MemberType::ArrayOfU16 => ret = "uint16_t* const",
            MemberType::ArrayOfU32 => ret = "uint32_t* const",
            MemberType::ArrayOfU64 => ret = "uint64_t* const",
            MemberType::ArrayOfI8 => ret = "int8_t* const",
            MemberType::ArrayOfI16 => ret = "int16_t* const",
            MemberType::ArrayOfI32 => ret = "int32_t* const",
            MemberType::ArrayOfI64 => ret = "int64_t* const",
            MemberType::ArrayOfF32 => ret = "float* const",
            MemberType::ArrayOfF64 => ret = "double* const",
            MemberType::Bool => ret = "bool",
            MemberType::Identifier(enum_type) => {
                return "const enum ".to_string() + enum_type.as_str()
            }
        }
        String::from(String::from("const ") + ret)
    }
}

fn fill_array<T, F: Fn(&T) -> String>(array: &Vec<T>, format_function: F) -> String {
    let mut ret;
    let nb_elements = array.len();
    ret = String::from("{");
    for (index, element) in array.iter().enumerate() {
        ret += format_function(element).as_str();

        if index < nb_elements - 1 {
            ret = ret + ", ";
        }
    }
    ret += "}";
    return ret;
}

impl ArrayInstanceVariant {
    fn generate(&self, array_name: &str) -> String {
        let array_value;
        let array_type;
        match self {
            ArrayInstanceVariant::X8(array) => {
                array_value = fill_array(array, |element| format!("0x{:x}", element));
                array_type = "uint8_t";
            }

            ArrayInstanceVariant::X16(array) => {
                array_value = fill_array(array, |element| format!("0x{:x}", element));
                array_type = "uint16_t";
            }

            ArrayInstanceVariant::X32(array) => {
                array_value = fill_array(array, |element| format!("0x{:x}", element));
                array_type = "uint32_t";
            }
            ArrayInstanceVariant::X64(array) => {
                array_value = fill_array(array, |element| format!("0x{:x}", element));
                array_type = "uint64_t";
            }
            ArrayInstanceVariant::U8(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "uint8_t";
            }
            ArrayInstanceVariant::U16(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "uint16_t";
            }
            ArrayInstanceVariant::U32(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "uint32_t";
            }
            ArrayInstanceVariant::U64(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "uint64_t";
            }
            ArrayInstanceVariant::I8(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "int8_t";
            }
            ArrayInstanceVariant::I16(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "int16_t";
            }
            ArrayInstanceVariant::I32(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "int32_t";
            }
            ArrayInstanceVariant::I64(array) => {
                array_value = fill_array(array, |element| format!("{}", element));
                array_type = "int64_t";
            }
            _ => unimplemented!(),
        }
        format!("const {} {}[] = {};", array_type, array_name, array_value)
    }
}

impl ToString for OperationParameterVariant {
    fn to_string(&self) -> String {
        let ret;
        match self {
            OperationParameterVariant::X8(param) => {
                ret = format!(".{} = 0x{:x}", param.name, param.value);
            }
            OperationParameterVariant::X16(param) => {
                ret = format!(".{} = 0x{:x}", param.name, param.value);
            }
            OperationParameterVariant::X32(param) => {
                ret = format!(".{} = 0x{:x}", param.name, param.value);
            }
            OperationParameterVariant::X64(param) => {
                ret = format!(".{} = 0x{:x}", param.name, param.value);
            }
            OperationParameterVariant::U8(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::U16(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::U32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::U64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::I8(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::I16(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::I32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::I64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::F32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::F64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfX8(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfX16(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfX32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfX64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfU8(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfU16(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfU32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfU64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfI8(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfI16(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfI32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfI64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfF32(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::ArrayOfF64(param) => {
                ret = format!(".{} = {}", param.name, param.value);
            }
            OperationParameterVariant::Bool(param) => {
                let val = if param.value {
                    "true".to_string()
                } else {
                    "false".to_string()
                };
                ret = format!(".{} = {}", param.name, val);
            }
            OperationParameterVariant::Identifier(param) => {
                let val = param.enum_type.clone() + "_" + param.value.as_str();
                ret = format!(".{} = {}", param.name, val);
            }
        }
        return ret;
    }
}

impl Model {
    fn generate_header_pre(&mut self, output_file: &mut File) {
        let sequence_name = self.sequence_name.as_mut().unwrap().to_uppercase();
        let sequence_name = sequence_name.as_str();
        write!(output_file, "#ifndef _{sequence_name}_H\n#define _{sequence_name}_H\n#include <stdint.h>\n#include <stdbool.h>\n").unwrap();
    }

    fn generate_header_post(&mut self, output_file: &mut File) {
        write!(output_file, "#endif\n").unwrap();
    }

    fn pascal_to_macro_case(input: &str) -> String {
        let mut result = String::new();
        
        for (i, c) in input.chars().enumerate() {
            if c.is_uppercase() {
                if i != 0 {
                    result.push('_');
                }
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_uppercase());
            }
        }

        result
    }

    fn generate_operation_id_enum(&mut self, output_file: &mut File) {
        write!(output_file, "enum OperationId{{\n").unwrap();

        for (record_name, _record_members) in self.defined_records.iter() {
            let record_name = Self::pascal_to_macro_case(record_name);
                write!(
                output_file,
                "   OPERATION_ID_{},\n",
                record_name
            )
            .unwrap();
        }
        write!(output_file, "}};\n").unwrap();
    }

    fn generate_blank_line(output_file: &mut File) {
        write!(output_file, "\n").unwrap();
    }

    fn generate_operation_variants_definition(&mut self, output_file: &mut File) {
        write!(output_file, "union OperationVariant{{\n").unwrap();
        for (struct_name, _) in self.defined_records.iter() {
            let struct_member_variant = struct_name.to_lowercase();
            write!(
                output_file,
                "   const {struct_name}* const {struct_member_variant};\n"
            )
            .unwrap();
        }
        write!(output_file, "}};\n").unwrap();
    }

    fn generate_operation_definition(&mut self, output_file: &mut File) {
        write!(output_file, "typedef struct{{\n   const enum OperationId id;\n   const union OperationVariant variant;\n}}Operation;\n").unwrap();
    }

    fn generate_enum_definitions(&mut self, output_file: &mut File) {
        for (enum_type_name, enum_members) in self.defined_enums.iter() {
            write!(output_file, "enum {enum_type_name}{{\n").unwrap();
            for enum_member in enum_members.iter() {
                write!(output_file, "   {enum_type_name}_{enum_member},\n").unwrap();
            }
            write!(output_file, "}};\n").unwrap();
        }
    }

    fn generate_struct_definitions(&mut self, output_file: &mut File) {
        for (struct_name, struct_members) in self.defined_records.iter() {
            write!(output_file, "typedef struct{{\n").unwrap();
            for struct_member in struct_members {
                let struct_member_name = struct_member.member_name.as_str();
                let struct_member_type = struct_member.member_type.to_string();
                write!(
                    output_file,
                    "   {struct_member_type} {struct_member_name};\n"
                )
                .unwrap();
            }
            write!(output_file, "}}{struct_name};\n",).unwrap();
            Self::generate_blank_line(output_file);
        }
    }

    fn generate_operation_list(&mut self, output_file: &mut File) {
        write!(
            output_file,
            "const Operation {}_operations[] = {{\n",
            self.sequence_name.as_ref().unwrap()
        )
        .unwrap();

        let nb_operations = self.operation_ref_table.len();
        for (index, operation_table_member) in self.operation_ref_table.iter().enumerate() {
            let record_name = Self::pascal_to_macro_case(&operation_table_member.operation_type);
            let operation_id = format!(
                "OPERATION_ID_{}",
                record_name
            );
            let operation_variant_instance_ref =
                operation_table_member.operation_variant_ref_name.as_str();
            let operation_variant_name = operation_table_member.operation_type.to_lowercase();
            write!(output_file, "   {{.id = {operation_id}, .variant={{.{operation_variant_name}=&{operation_variant_instance_ref}}}}}",).unwrap();
            if index < nb_operations - 1 {
                write!(output_file, ",\n").unwrap();
            }
        }

        write!(output_file, "\n}};\n",).unwrap();
        Self::generate_blank_line(output_file);
    }

    fn generate_array_instances(&mut self, output_file: &mut File) {
        for (operation, operation_instance_name) in self.instanciated_arrays.iter() {
            write!(
                output_file,
                "{}\n",
                operation.generate(operation_instance_name),
            )
            .unwrap();
        }
        Self::generate_blank_line(output_file);
    }

    fn generate_operation_instances(&mut self, output_file: &mut File) {
        for (operation, operation_instance_name) in self.operation_instances.iter() {
            let operation_type = operation.operation_type.as_str();
            write!(
                output_file,
                "const {operation_type} {operation_instance_name} = {{"
            )
            .unwrap();
            let nb_parameters = operation.parameters.len();

            for (index, operation_parameter) in operation.parameters.iter().enumerate() {
                let parameter_name = operation_parameter.to_string();
                write!(output_file, "{}", parameter_name).unwrap();
                if index < nb_parameters - 1 {
                    write!(output_file, ", ").unwrap();
                }
            }

            write!(output_file, "}};\n").unwrap();
        }
        Self::generate_blank_line(output_file);
    }

    fn generate_source_pre(&mut self, output_file: &mut File) {
        write!(output_file, "#include \"playdisc.h\"\n").unwrap();
        Self::generate_blank_line(output_file);
    }

    fn generate_source_post(&mut self, output_file: &mut File) {
        let sequence_name = self.sequence_name.as_ref().unwrap().to_lowercase();
        write!(
            output_file,
            "const uint32_t nb_{}_operations = sizeof({}_operations)/sizeof(Operation);\n",
            sequence_name, sequence_name
        )
        .unwrap();
    }

    fn generate_c_source(&mut self, output_file: &mut File) {
        self.generate_source_pre(output_file);
        self.generate_array_instances(output_file);
        self.generate_operation_instances(output_file);
        self.generate_operation_list(output_file);
        self.generate_source_post(output_file);
    }

    fn generate_c_header(&mut self, output_file: &mut File) {
        self.generate_header_pre(output_file);
        Self::generate_blank_line(output_file);
        self.generate_operation_id_enum(output_file);
        Self::generate_blank_line(output_file);
        self.generate_enum_definitions(output_file);
        Self::generate_blank_line(output_file);
        self.generate_struct_definitions(output_file);
        self.generate_operation_variants_definition(output_file);
        Self::generate_blank_line(output_file);
        self.generate_operation_definition(output_file);
        Self::generate_blank_line(output_file);
        self.generate_header_post(output_file);
    }

    pub fn compute_to_c(&mut self, output_c_file: &mut File, output_h_file: &mut File) {
        self.generate_c_header(output_h_file);
        self.generate_c_source(output_c_file);
    }
}
