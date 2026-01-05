// Build script to add Windows metadata and reduce AV false positives
fn main() {
    // Only run on Windows or when cross-compiling to Windows
    #[cfg(any(target_os = "windows", target_env = "gnu"))]
    {
        let mut res = winres::WindowsResource::new();
        
        // Comprehensive metadata helps AV understand the file
        res.set("ProductName", "Roblox Booster")
            .set("FileDescription", "System performance optimizer for Roblox - Safe and legitimate tool")
            .set("CompanyName", "Open Source Community")
            .set("LegalCopyright", "Copyright © 2026 - Open Source")
            .set("ProductVersion", "0.1.0")
            .set("FileVersion", "0.1.0.0")
            .set("InternalName", "roblox_booster")
            .set("OriginalFilename", "roblox_booster.exe")
            .set("Comments", "Safe system optimizer - No malware - Open source on GitHub");
        
        // Set manifest to hide console (redundant with windows_subsystem but good to have)
        res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
    version="0.1.0.0"
    processorArchitecture="*"
    name="RobloxBooster"
    type="win32"
  />
  <description>Roblox System Performance Booster</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#);
        
        // Set icon if available (helps with trust)
        if std::path::Path::new("icon.ico").exists() {
            res.set_icon("icon.ico");
        }
        
        // Compile resource
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
            eprintln!("This may increase antivirus false positives.");
        }
    }
}