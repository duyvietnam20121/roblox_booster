fn main() {
    // Chỉ build metadata trên Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        
        // Metadata để giảm false positive từ antivirus
        res.set_icon("icon.ico") // Optional: nếu có icon
           .set("ProductName", "Roblox Booster")
           .set("FileDescription", "Safe Roblox performance optimizer")
           .set("CompanyName", "Made by AI")
           .set("LegalCopyright", "MIT License")
           .set("ProductVersion", env!("CARGO_PKG_VERSION"))
           .set("FileVersion", env!("CARGO_PKG_VERSION"));
        
        // Compile resource
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {e}");
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Cross-compile: winres không cần thiết
        println!("cargo:warning=Skipping Windows resource compilation (cross-compile)");
    }
}