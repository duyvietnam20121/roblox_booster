fn main() {
    // Windows resource compilation
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();

        // Professional metadata to reduce antivirus false positives
        res.set("ProductName", "Roblox Performance Optimizer")
            .set(
                "FileDescription",
                "Game performance optimizer using Windows priority control",
            )
            .set("CompanyName", "Open Source Project")
            .set("LegalCopyright", "MIT License - Open Source")
            .set("ProductVersion", env!("CARGO_PKG_VERSION"))
            .set("FileVersion", env!("CARGO_PKG_VERSION"))
            .set("OriginalFilename", "roblox_booster.exe")
            .set("InternalName", "roblox_booster");

        // Compile resources
        if let Err(error) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {error}");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:warning=Skipping Windows resource compilation (cross-compile mode)");
    }
}