use std::env;
use std::fs;

fn main() {
    // Get the current build profile (debug or release)
    let profile = env::var("PROFILE").unwrap();

    // Specify the source file path relative to the project root
    let source_file = "./analyzer_module.py";

    // Specify the target file path relative to the build folder based on the profile
    let target_file = match profile.as_str() {
        "debug" => "target/debug/analyzer_module.py",
        "release" => "target/release/analyzer_module.py",
        _ => panic!("Unsupported build profile"),
    };

    // Copy the file from the source to the target location
    fs::copy(source_file, target_file).expect("Failed to copy file");
}