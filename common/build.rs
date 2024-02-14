use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Locate the .env file relative to the build script
    let env_file_path = Path::new("build.rs").parent()
        .unwrap()
        .join(".env");

    // Read the .env file content
    let env_file_content = fs::read_to_string(&env_file_path)?;

    // Parse each line as a key-value pair
    for line in env_file_content.lines() {
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();

        // Set the environment variable using `env::set_var`
        println!("env: {}={}", key, value);
        env::set_var(key, value);
    }

    // Compile with updated environment variables
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
