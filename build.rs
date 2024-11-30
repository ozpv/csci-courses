use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // always trigger the build script
    println!("cargo:rerun-if-changed=.");

    let pwd = std::env::current_dir()?;
    let input = pwd.join("style").join("tailwind.css");
    let output = pwd.join("assets").join("csci-courses.css");

    println!("Building tailwindcss");
    let output = Command::new("tailwindcss")
        .args([
            "-i",
            input.as_os_str().to_str().unwrap(),
            "-o",
            output.as_os_str().to_str().unwrap(),
            "--minify",
        ])
        .output()
        .expect("Failed to run tailwindcss");

    println!(
        "{}",
        String::from_utf8(output.stdout).expect("stdout contained invalid UTF-8")
    );

    Ok(())
}
