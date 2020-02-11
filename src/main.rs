#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate tinytemplate;

mod file_reader;

use std::fs::File;
use std::io::prelude::*;
use tinytemplate::TinyTemplate;


#[derive(Serialize, Clone, Debug)]
struct TemplateTableEntry {
    field: String,
    range_start: u32,
    range_end:   u32,
}
#[derive(Serialize, Clone, Debug)]
struct TemplateTable {
    entries: Vec<TemplateTableEntry>
}
#[derive(Serialize, Debug)]
struct TemplateContext {
    tables: Vec<TemplateTable>,
    style: String,
}
static TABLES_TEMPLATE: &'static str = include_str!("tables.html");
static TEMPLATE_STYLES: &'static str = include_str!("styles.css");

fn main() {
    let mut demo_reader = file_reader::FileReader::from_file("assets/inputs/demo.tables");


    let latest_version = String::from("v1");
    let mut version = demo_reader.next_word().unwrap_or("*".to_string());
    if version == "*" {
        version = latest_version.clone();
    }
    if version > latest_version {
        panic!("Unknown Version");
    }
    

    let mut context = TemplateContext{
        tables: Vec::new(),
        style: String::from(TEMPLATE_STYLES),
    };
    let mut current_table = TemplateTable{ entries: Vec::new() };
    let mut current_range = 0u32;

    while let Some(word) = demo_reader.next_word() {
        if word == "table" {
            current_table = TemplateTable{ entries: Vec::new() };
            current_range = 0u32;

        } else if word == "endtable" {
            context.tables.push(current_table.clone());

        } else if let Ok(number) = word.parse::<u32>() {
            let word = demo_reader.to_end_of_line().unwrap_or_default();
            current_table.entries.push(TemplateTableEntry{
                field: word,
                range_start: current_range,
                range_end: current_range + number,
            });
            current_range += number;

        } else {
            break; // TODO: error case?
        }
    }


    let mut tt = TinyTemplate::new();
    tt.add_template("tables", TABLES_TEMPLATE).unwrap();

    let rendered_html = tt.render("tables", &context).unwrap();
    println!("{}", rendered_html);

    if let Ok(mut output) = File::create("assets/outputs/output.html") {
        output.write_all(rendered_html.as_bytes()).unwrap();
    }
}
