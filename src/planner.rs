use crate::parser::Select;

pub struct Scan {
    pub from: String,
}

pub fn plan(query: Select) -> Scan {
    Scan { from: query.from }
}
