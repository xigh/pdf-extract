extern crate pdf_extract;
extern crate lopdf;

use std::env;
use std::path::PathBuf;
use std::path;
use std::io::BufWriter;
use std::fs::File;
use pdf_extract::*;

fn main() -> std::result::Result<(), OutputError> {
    //let output_kind = "html";
    //let output_kind = "txt";
    //let output_kind = "svg";
    let Some(file) = env::args().nth(1) else {
        println!("Usage: {} <pdf_file> <output_kind>", env::args().nth(0).unwrap());
        return Ok(());
    };
    let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());
    let path = path::Path::new(&file);
    let filename = path.file_name().expect("expected a filename");
    let mut output_file = PathBuf::new();
    output_file.push(filename);
    output_file.set_extension(&output_kind);
    println!("{} -> {:?}", file, output_file);

    let mut output_file = BufWriter::new(File::create(output_file).expect("could not create output"));
    let Ok(doc) = Document::load(&path) else {
        eprintln!("Could not load document {:?}", path);
        return Ok(());
    };

    print_metadata(&doc);

    let mut output: Box<dyn OutputDev> = match output_kind.as_ref() {
        "txt" => Box::new(PlainTextOutput::new(&mut output_file as &mut dyn std::io::Write)),
        "html" => Box::new(HTMLOutput::new(&mut output_file)),
        "svg" => Box::new(SVGOutput::new(&mut output_file)),
        _ => panic!(),
    };

    output_doc(&doc, output.as_mut())
}
