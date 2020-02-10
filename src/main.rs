#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate tinytemplate;

mod file_reader;
mod html_output;

use std::fs::File;
use std::io::prelude::*;
use tinytemplate::TinyTemplate;


#[derive(Debug, Clone, Serialize)]
struct Table {
    fields: Vec<String>,
    ranges: Vec<u32>,
}
impl Table {
    fn new() -> Table {
        Table { fields: Vec::new(), ranges: Vec::new() }
    }
    fn push(&mut self, field: String, range: u32) {
        self.fields.push(field);
        self.ranges.push(range);
    }
}

#[derive(Serialize)]
struct TemplateTableEntry { field: String, range_start: u32, range_end: u32 }
#[derive(Serialize)]
struct TemplateTable { entries: Vec<TemplateTableEntry> }
#[derive(Serialize)]
struct TemplateContext {
    tables: Vec<TemplateTable>,
}
static TABLES_TEMPLATE: &'static str = include_str!("tables.html");

fn main() {
    let mut demo_reader = FileReader::from_file("assets/inputs/demo.tables");
    let mut tables: Vec<Table> = Vec::new();
    let mut table = Table::new();

    let latest_version = String::from("v1");
    let mut version = demo_reader.next_word().unwrap_or("*".to_string());
    if version == "*" {
        version = latest_version.clone();
    }
    if version > latest_version {
        panic!("Unknown Version");
    }

    while let Some(word) = demo_reader.next_word() {
        if word == "table" {
            table = Table::new();
        } else if word == "endtable" {
            tables.push(table.clone());
        } else if let Ok(number) = word.parse() {
            let word = demo_reader.to_end_of_line().unwrap_or_default();
            table.push(word, number);
        } else {
            break; // TODO: error case?
        }
    }

    let mut tt = TinyTemplate::new();
    tt.add_template("tables", TABLES_TEMPLATE).unwrap();

    let context = TemplateContext{
        tables: tables,
    };
    let rendered_html = tt.render("tables", &context).unwrap();
    println!("{}", rendered_html);

    if let Ok(mut output) = File::create("assets/outputs/output.html") {
        output.write_all(rendered_html.as_bytes()).unwrap();
    }
}
