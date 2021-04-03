use std::collections::HashMap;

mod lexer;
mod parser;
mod planner;

#[derive(Debug, PartialEq)]
struct Scan<'a> {
    table_name: &'a str,
}

trait Source {
    fn scan(&self) -> Vec<Vec<String>>;
}

#[derive(Debug, Clone)]
struct CsvSource<'a> {
    filename: &'a str,
    separator: &'a str,
}

type Row = Vec<String>;

impl Source for CsvSource<'_> {
    fn scan(&self) -> Vec<Row> {
        let data = std::fs::read_to_string(format!("data/{}", self.filename)).unwrap();
        data.lines()
            .map(|row| row.split(self.separator).map(String::from).collect())
            .collect()
    }
}

type Schemas<'a> = HashMap<&'a str, &'a CsvSource<'a>>;

fn run_query(query_string: &str, schemas: &Schemas) {
    let query = parser::parse(query_string);
    let plan = planner::plan(query);

    let source = schemas.get(&plan.from as &str).unwrap();
    let rows: Vec<Row> = source.scan();
    for row in rows.into_iter().take(10) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_select() {
        let sql_input = "SELECT * FROM fishes";
        let ast = parse_sql(sql_input);
        assert_eq!(
            ast,
            Scan {
                table_name: "fishes"
            }
        )
    }
}
