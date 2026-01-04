// Build script to add Windows metadata
#[cfg(target_os = "windows")]
fn main() {
    // Add Windows resource file
    let mut res = winres::WindowsResource::new();
    
    res.set_icon("icon.ico") // Optional: add if you have icon
        .set("ProductName", "Roblox Booster")
        .set("FileDescription", "System performance optimizer for Roblox")
        .set("CompanyName", "Your Name")
        .set("LegalCopyright", "Copyright 2026")
        .set("ProductVersion", "0.1.0")
        .set("FileVersion", "0.1.0");
    
    // Compile resource
    if let Err(e) = res.compile() {
        eprintln!("Warning: Failed to compile Windows resources: {}", e);
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Do nothing on non-Windows platforms
}