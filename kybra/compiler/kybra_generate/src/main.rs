use std::fs::File;
use std::io::Write;

use kybra_generate::generate_canister;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let py_file_names_path = &args[1];
    let py_entry_module_name = &args[2];
    let output_file_path = &args[3];

    let py_file_names_string = std::fs::read_to_string(py_file_names_path).unwrap();
    let py_file_names: Vec<&str> = py_file_names_string.split(",").collect();

    let lib_file = match generate_canister(&py_file_names, &py_entry_module_name) {
        Ok(canister) => canister,
        Err(errors) => {
            eprintln!("Canister Compilation failed:");
            for error in errors {
                eprintln!("{}", error)
            }
            return;
        }
    }
    .to_string();

    let mut f = File::create(output_file_path).expect("Unable to create file");
    f.write_all(lib_file.as_bytes())
        .expect("Unable to write data");
}
