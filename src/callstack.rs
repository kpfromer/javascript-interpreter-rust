use std::collections::HashMap;

#[derive(Debug)]
pub enum ActivationRecordValue<INT> {
  BooleanValue(bool),
  IntValue(INT),
  // FunctionDecl();
}

pub struct ActivationRecord<INT> {
  pub name: String,
  pub level: u32,
  pub parent: Option<Box<ActivationRecord<INT>>>,
  pub records: HashMap<String, ActivationRecordValue<INT>>, // type
}

impl<INT> ActivationRecord<INT> {
  // pub fn set(&)
  pub fn get(&mut self, name: &String) -> Option<&ActivationRecordValue<INT>> {
    return self.records.get(name);
  }
  pub fn set(&mut self, name: String, value: ActivationRecordValue<INT>) {
    self.records.insert(name, value);
  }
  pub fn remove(&mut self, name: &String) -> Option<ActivationRecordValue<INT>> {
    self.records.remove(name)
  }
}

impl std::fmt::Display for ActivationRecordValue<i64> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let display = match self {
      ActivationRecordValue::BooleanValue(bool_value) => {
        if *bool_value {
          String::from("BooleanValue: True")
        } else {
          String::from("BooleanValue: False")
        }
      }
      ActivationRecordValue::IntValue(int_value) => format!("IntValue: {}", int_value.to_string()),
    };
    write!(f, "{}", display)
  }
}

impl std::fmt::Display for ActivationRecord<i64> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "\"{}\" Callstack\nSTART\n{:?}\nEND\n",
      self.name, self.records
    )
  }
}
