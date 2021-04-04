type Row = Vec<String>;

pub struct Records {
    pub field_names: Vec<String>,
    pub data: Vec<Row>,
}

pub trait Source {
    fn records(&self) -> Records;
}

#[derive(Debug, Clone)]
pub struct CsvSource<'a> {
    pub filename: &'a str,
    pub separator: &'a str,
}

impl Source for CsvSource<'_> {
    fn records(&self) -> Records {
        let file_contents = std::fs::read_to_string(format!("data/{}", self.filename)).unwrap();
        let mut rows = file_contents.lines();
        let field_names: Vec<String> = match rows.next() {
            None => panic!("No header row in CSV"),
            Some(row) => row.split(self.separator).map(String::from).collect(),
        };
        let data: Vec<Row> = rows
            .map(|row| row.split(self.separator).map(String::from).collect())
            .collect();
        Records { field_names, data }
    }
}
