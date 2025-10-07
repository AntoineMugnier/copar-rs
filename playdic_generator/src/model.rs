use crate::unirecord::{MemberType, UniRecord};
use std::collections::HashMap;
use std::fs::File;

struct StructureMember {
    member_name: String,
    member_type: MemberType,
}
struct DeclaredStructure {
    name: String,
    members: Vec<StructureMember>,
}

impl DeclaredStructure {
    pub fn from(unirecord: &UniRecord) -> DeclaredStructure {
        let mut members = Vec::new();
        for record_arg in unirecord.args().iter() {
            let member = StructureMember {
                member_name: record_arg.get_name().clone(),
                member_type: record_arg.get_type(),
            };
            members.push(member);
        }
        DeclaredStructure {
            name: unirecord.name().clone(),
            members,
        }
    }
}

pub struct Model {
    declared_enums: HashMap<String, Vec<String>>,
    declared_records: HashMap<String, Vec<StructureMember>>,
    operations: Vec<UniRecord>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            declared_enums: HashMap::new(),
            declared_records: HashMap::new(),
            operations: Vec::new(),
        }
    }

    pub fn compute(&mut self, output_file : &mut File, sequence_name: String) {

    }
    
    pub fn add_record(&mut self, record: UniRecord) {
        println!("{:?}", record);
        let record_type = record.name().clone();

        // If no declaration exists for this record, create it
        if self.declared_records.get(&record_type).is_none() {
            let declared_structure = DeclaredStructure::from(&record);
            self.declared_records.insert(declared_structure.name, declared_structure.members);
        }

        self.operations.push(record);
    }
}
