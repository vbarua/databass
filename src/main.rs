use crate::executor::{Executor, Schemas};
use crate::sources::CsvSource;
use std::collections::HashMap;

mod executor;
mod lexer;
mod parser;
mod planner;
mod sources;

fn run_query(query_string: &str, schemas: &Schemas) {
    let query = parser::parse(query_string);
    let plan = planner::plan(query);

    let executor = Executor { schemas };
    let records = executor.execute(plan);

    println!("Field Names: {:?}", records.field_names);

    for row in records.data.into_iter().take(10) {
        println!("{:?}", row);
    }
}

fn main() {
    let fish_source = CsvSource {
        filename: "fish.csv",
        separator: ",",
    };
    let mut schemas: Schemas = HashMap::new();
    schemas.insert("fish", &fish_source);

    run_query("SELECT * FROM fish", &schemas);
}
