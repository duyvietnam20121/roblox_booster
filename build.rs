// Build script v2.0 - Enhanced metadata to minimize AV false positives
fn main() {
    #[cfg(any(target_os = "windows", target_env = "gnu"))]
    {
        let mut res = winres::WindowsResource::new();
        
        // Comprehensive metadata helps AVs understand this is legitimate software
        res.set("ProductName", "Roblox Booster v2")
            .set(
                "FileDescription",
                "Legitimate Roblox Performance Optimizer - Open Source Tool",
            )
            .set("CompanyName", "Open Source Community - duyvietnam20121")
            .set("LegalCopyright", "Copyright © 2026 - MIT License - Open Source")
            .set("ProductVersion", "2.0.0.0")
            .set("FileVersion", "2.0.0.0")
            .set("InternalName", "roblox_booster")
            .set("OriginalFilename", "roblox_booster.exe")
            .set(
                "Comments",
                "Safe system performance optimizer for Roblox. Uses only standard Windows APIs. \
                 No malware, no data collection, no remote access. \
                 Source code available on GitHub: github.com/duyvietnam20121/roblox_booster. \
                 This tool only adjusts CPU/GPU priority via standard Windows API calls. \
                 Safe for gaming performance enhancement.",
            );
        
        // Enhanced manifest with explicit trust indicators
        res.set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
    version="2.0.0.0"
    processorArchitecture="*"
    name="RobloxBoosterV2.PerformanceOptimizer"
    type="win32"
  />
  <description>Roblox System Performance Booster v2 - Legitimate Gaming Tool</description>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <!-- Windows 10 and 11 compatibility -->
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
      <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>
      <supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}"/>
    </application>
  </compatibility>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <!-- Standard user privileges only - no admin required -->
        <requestedExecutionLevel level="asInvoker" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <!-- Declare DPI awareness for better UI -->
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#,
        );

        // Set icon if available
        if std::path::Path::new("icon.ico").exists() {
            res.set_icon("icon.ico");
        }
        
        // Compile resource with enhanced error handling
        match res.compile() {
            Ok(_) => {
                println!("cargo:warning=✓ Windows metadata compiled successfully");
                println!("cargo:warning=✓ This helps reduce antivirus false positives");
            }
            Err(e) => {
                eprintln!(
                    "cargo:warning=⚠️ Failed to compile Windows resources: {}",
                    e
                );
                eprintln!("cargo:warning=⚠️ This may increase antivirus false positives");
                eprintln!("cargo:warning=💡 Solution: Install required build tools");
            }
        }
    }

    // Print build info
    println!("cargo:warning=");
    println!("cargo:warning=🔨 Building Roblox Booster v2.0.0");
    println!("cargo:warning=📋 Features: GPU Boost + Custom Path + AV-Safe");
    println!("cargo:warning=");
}