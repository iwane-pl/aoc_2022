use std::fs;
use std::env;
use std::path;

pub fn load(in_path: &str) -> String {

    println!("--- Input data ---");
    let mut exe = env::current_exe().unwrap();
    let file_path_disp = exe.display();
    println!("Executable path: {file_path_disp}");
    
    let file_path = path::PathBuf::from(in_path);
    let file_path_disp = file_path.display();
    println!("Input path: {file_path_disp}");
    
    exe.push(file_path);
    let file_path_disp = exe.display();
    println!("Absolute input path: {file_path_disp}");

    let contents = fs::read_to_string(exe).expect("Should have been able to read the file");
    return contents;
}