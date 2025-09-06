use docx_rust::DocxFile;

fn main() {
    let path = std::path::Path::new("./tests/pandoc/default.docx");
    println!("Attempting to parse: {:?}", path);
    
    match DocxFile::from_file(path) {
        Ok(docx_file) => {
            println!("DocxFile created successfully");
            match docx_file.parse() {
                Ok(_) => println!("Parsed successfully!"),
                Err(err) => println!("Parse error: {:?}", err),
            }
        }
        Err(err) => {
            println!("Error creating DocxFile: {:?}", err);
        }
    }
}