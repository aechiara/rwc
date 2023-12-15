fn main() {

    // parse and hold arguments passed from command-line
    let args: Vec<String> = std::env::args().collect();

    // check if receives at least one argument
    if args.len() < 2 {
        println!("Please provide a file path");
        return;
    }

    // get file path from arguments
    let file_path = &args[1];

    // check if file exists
    let file_path = std::path::Path::new(file_path);
    if !file_path.is_file() || !file_path.exists() {
        println!("File [{}] not found", file_path.file_name().unwrap().to_str().unwrap());
        return;
    }

    let file_content = std::fs::read_to_string(file_path).expect("Should be able to read the file");
    let total_lines: u32 = file_content.lines().count() as u32;

    println!("{} has {total_lines} lines", file_path.file_name().unwrap().to_str().unwrap());
}
