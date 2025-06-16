use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Generate icon if SVG exists and ICO doesn't exist
    let svg_path = Path::new("../../assets/icon.svg");
    let ico_path = Path::new("../../assets/icon.ico");

    if svg_path.exists() && !ico_path.exists() {
        println!("cargo:warning=SVG icon found but ICO missing. Attempting to generate ICO...");
        generate_icon_from_svg();
    }

    // Only build resources on Windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();

        // Set the icon if it exists
        if ico_path.exists() {
            res.set_icon_with_id(&ico_path.to_string_lossy(), "1");
            println!("cargo:warning=Using icon: {}", ico_path.display());
        } else {
            println!("cargo:warning=No icon file found at {}", ico_path.display());
        }

        // Set version information
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("ProductName", "shimexe");
        res.set(
            "FileDescription",
            "A modern, cross-platform executable shim manager",
        );
        res.set("CompanyName", "shimexe");
        res.set("LegalCopyright", "Copyright (c) 2025 Hal");
        res.set("OriginalFilename", "shimexe.exe");
        res.set("InternalName", "shimexe");

        // Compile the resources
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
            // Don't fail the build if resource compilation fails
        }
    }

    // Tell Cargo to rerun this build script if the icon changes
    println!("cargo:rerun-if-changed=../../assets/icon.ico");
    println!("cargo:rerun-if-changed=../../assets/icon.svg");
}

fn generate_icon_from_svg() {
    // Try to generate ICO from SVG using ImageMagick
    let output = Command::new("magick")
        .args(&[
            "convert",
            "-background",
            "transparent",
            "-define",
            "icon:auto-resize=256,128,64,48,32,16",
            "../../assets/icon.svg",
            "../../assets/icon.ico",
        ])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("cargo:warning=Successfully generated icon.ico from icon.svg");
            } else {
                println!(
                    "cargo:warning=Failed to generate ICO: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
        Err(_) => {
            println!("cargo:warning=ImageMagick not found. Skipping icon generation.");
            println!("cargo:warning=To include an icon, install ImageMagick and run: magick convert -background transparent -define icon:auto-resize=256,128,64,48,32,16 assets/icon.svg assets/icon.ico");
        }
    }
}
