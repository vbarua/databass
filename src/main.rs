use std::collections::HashMap;

mod lexer;

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

impl Source for CsvSource<'_> {
    fn scan(&self) -> Vec<Vec<String>> {
        let data = std::fs::read_to_string(format!("data/{}", self.filename)).unwrap();
        data.lines()
            .map(|row| row.split(self.separator).map(String::from).collect())
            .collect()
    }
}

// Supports statements of the form
// * SELECT * FROM <table_name>
fn parse_sql(sql: &str) -> Scan {
    let x = sql.rsplit("FROM").next();
    match x {
        None => panic!("SQL input did not contain FROM clause"),
        Some(table_name) => Scan {
            table_name: table_name.trim(),
        },
    }
}

fn interpret_query<'a>(query: Scan, schemas: HashMap<&str, &'a CsvSource>) -> Vec<Vec<String>> {
    let source = schemas.get(query.table_name).unwrap();
    source.scan()
}

fn main() {
    let query = parse_sql("SELECT * FROM fish");
    let fish_source = CsvSource {
        filename: "fish.csv",
        separator: ",",
    };
    let mut schemas: HashMap<&str, &CsvSource> = HashMap::new();
    schemas.insert("fish", &fish_source);
    let results = interpret_query(query, schemas);
    for row in results {
        println!("{:?}", row);
    }
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
