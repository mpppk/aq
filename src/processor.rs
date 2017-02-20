use serde_json::Value;
pub type Adapter = Box<Fn(&Value) -> Option<&Value>>;

pub enum Processor {
    Adapter(Box<Fn(&Value) -> Option<&Value>>)
}

pub fn select(key: &str) -> Processor {
    let key = key.to_string();
    Processor::Adapter(Box::new(move |v| v.get(&key)))
}

pub fn do_nothing() -> Processor {
    Processor::Adapter(Box::new(move |v| Some(v)))
}
