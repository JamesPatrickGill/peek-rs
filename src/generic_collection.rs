use std::collections::{hash_map::IntoIter, HashMap};

#[derive(Debug, Clone)]
pub struct PeekMap {
    data: HashMap<String, PeekValue>,
}

impl PeekMap {
    pub fn new(data: HashMap<String, PeekValue>) -> PeekMap {
        PeekMap { data }
    }

    pub fn entries(&self) -> Vec<(String, PeekValue)> {
        self.data.clone().into_iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct PeekArray {
    pub data: Vec<PeekValue>,
}

impl PeekArray {
    pub fn new(data: Vec<PeekValue>) -> PeekArray {
        PeekArray { data }
    }
}

#[derive(Debug, Clone)]
pub enum PeekValue {
    Null,
    String(String),
    Float(f64),
    Integer(i64),
    Bool(bool),
    Array(PeekArray),
    Map(PeekMap),
}
