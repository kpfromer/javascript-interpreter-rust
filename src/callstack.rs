use std::collections::HashMap;

pub enum ActivationRecordValue {
  BooleanValue(bool),
  IntValue(i32),
  // FunctionDecl();
}

pub struct ActivationRecord {
  pub name: String,
  pub level: u32,
  pub parent: Option<Box<ActivationRecord>>,
  pub records: HashMap<String, ActivationRecordValue>, // type
}

impl ActivationRecord {
  pub fn get(&mut self, name: &String) -> Option<&ActivationRecordValue> {
    return self.records.get(name);
  }
  pub fn set(&mut self, name: &String, value: ActivationRecordValue) {
    self.records.insert(name.clone(), value);
  }
}
