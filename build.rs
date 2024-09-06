fn main() {
    let npm = check_npm();

    // Compile TailwindCSS .css file
    std::process::Command::new(npm)
        .args([
            "tailwindcss",
            "-i",
            "./input.css",
            "-c",
            "./tailwind.config.js",
            "-o",
            "./assets/tailwind.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}

fn check_npm() -> &'static str {
    let npm = "npm";
    let npx = "npx";

    match std::process::Command::new(npm).arg("install").spawn() {
        Ok(_) => npx,
        Err(e) => panic!("ERROR: npm installation is needed.\n{e}"),
    }
}
