extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tinytemplate;
extern crate web_view;
extern crate nfd;

mod file_reader;

use tinytemplate::TinyTemplate;
use web_view::*;


#[derive(Serialize, Clone, Debug)]
struct TemplateTableEntry {
    field: String,
    only_use_range_start: bool,
    range_start: u32,
    range_end:   u32,
}
#[derive(Serialize, Clone, Debug, Default)]
struct TemplateTable {
    title: String,
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
    let default_filename: &'static str = "assets/inputs/demo.tables";
    let exe_path: String = std::env::current_exe().unwrap().into_os_string().into_string().unwrap();
    
    /*
    let nfd_result = nfd::open_file_dialog(None, Some(exe_path.as_str())).unwrap();
    let filename = match &nfd_result {
        Response::Okay(path) => path,
        Response::OkayMultiple(_paths) => panic!("This *can't* happen"),
        Response::Cancel => default_filename,
    };
    */
    let filename = default_filename;
    
    let mut demo_reader = file_reader::FileReader::from_file(filename);

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
    let mut current_table = Default::default();
    let mut current_range = 0u32;

    while let Some(word) = demo_reader.next_word() {
        if word == "table" {
            let title = demo_reader.to_end_of_line().unwrap_or_default();
            current_table = TemplateTable{ entries: Vec::new(), title: title };
            current_range = 0u32;

        } else if word == "endtable" {
            context.tables.push(current_table.clone());

        } else if let Ok(number) = word.parse::<u32>() {
            let word = demo_reader.to_end_of_line().unwrap_or_default();
            current_table.entries.push(TemplateTableEntry{
                field: word,
                range_start: current_range + 1,
                range_end: current_range + number,
                only_use_range_start: number == 1,
            });
            current_range += number;

        } else {
            break; // TODO: error case?
        }
    }


    let mut tt = TinyTemplate::new();
    tt.add_template("tables", TABLES_TEMPLATE).unwrap();

    let rendered_html = tt.render("tables", &context).unwrap();
    // println!("{}", rendered_html);

    // webview HIDPI "fix": https://github.com/zserge/webview/issues/54
    web_view::builder()
        .title("Tables 1.0.0")
        .content(Content::Html(rendered_html))
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    /*
    if let Ok(mut output) = File::create("assets/outputs/output.html") {
        output.write_all(rendered_html.as_bytes()).unwrap();
    }
    */
}
