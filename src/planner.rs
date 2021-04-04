use crate::parser::Select;

#[derive(Debug, PartialEq)]
pub enum PlanNode {
    Scan {
        from: String,
    },
    Project {
        fields: Vec<String>,
        node: Box<PlanNode>,
    },
}

pub fn plan(query: Select) -> PlanNode {
    PlanNode::Scan { from: query.from }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    #[test]
    fn parse_select() {
        let sql_input = "SELECT * FROM fishes";
        let plan = plan(parse(sql_input));
        assert_eq!(
            plan,
            PlanNode::Scan {
                from: String::from("fishes")
            }
        )
    }
}
