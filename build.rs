use std::process::Command;

fn main() {
    // always trigger the build script
    println!("cargo:rerun-if-changed=NULL");

    println!("Building tailwindcss");
    let output = Command::new("pnpm")
        .args([
            "tailwindcss",
            "-i",
            "/style/tailwind.css",
            "-o",
            "/assets/csci-courses.css",
            "--minify",
        ])
        .output()
        .expect("Failed to run tailwindcss");

    println!(
        "{}",
        String::from_utf8(output.stdout).expect("stdout contained invalid UTF-8")
    );
}
