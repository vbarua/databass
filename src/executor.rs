use std::collections::HashMap;

use crate::planner::PlanNode;
use crate::sources::{CsvSource, Records, Source};

pub type Schemas<'a> = HashMap<&'a str, &'a CsvSource<'a>>;

pub struct Executor<'a> {
    pub schemas: &'a Schemas<'a>,
}

impl Executor<'_> {
    pub fn execute(&self, node: PlanNode) -> Records {
        match node {
            PlanNode::Scan { from } => {
                let source = match self.schemas.get(&from as &str) {
                    None => panic!("Schema for '{}' not found", from),
                    Some(s) => s,
                };
                source.records()
            }
            PlanNode::Project { fields, node } => todo!(),
        }
    }
}
