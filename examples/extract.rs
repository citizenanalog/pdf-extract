extern crate pdf_extract;
extern crate lopdf;

use std::env;
use std::path::PathBuf;
use std::path;
use std::io::BufWriter;
use std::fs::File;
use std::fs;
use pdf_extract::*;
use lopdf::*;

fn main() {
    //let output_kind = "html";
    //let output_kind = "txt";
    //let output_kind = "svg";
    let directory_path = env::args().nth(1).unwrap();

    // Read the directory
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file = entry.path();
                if let Some(extension) = file.extension() {
                    if extension == "pdf" {
                        // Process the PDF file
                        println!("Found PDF file: {:?}", file);
                        
                        // Add your code to process the PDF file here
                        let output_kind = env::args().nth(2).unwrap_or_else(|| "txt".to_owned());
                        println!("{:?}", file);
                        let path = path::Path::new(&file);
                        let filename = path.file_name().expect("expected a filename");
                        let mut output_file = PathBuf::new();
                        output_file.push(filename);
                        output_file.set_extension(&output_kind);
                        let mut output_file = BufWriter::new(File::create(output_file).expect("could not create output"));
                        let doc = Document::load(path).unwrap();
                    
                        print_metadata(&doc);
                    
                        let mut output: Box<dyn OutputDev> = match output_kind.as_ref() {
                            "txt" => Box::new(PlainTextOutput::new(&mut output_file as &mut dyn std::io::Write)),
                            "html" => Box::new(HTMLOutput::new(&mut output_file)),
                            "svg" => Box::new(SVGOutput::new(&mut output_file)),
                            _ => panic!(),
                        };
                    
                        output_doc(&doc, output.as_mut());
                    }
                }
            }
        }
    } else {
        println!("Failed to read the directory");
    }

   
}
