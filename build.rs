use std::io::Write;

fn main() {
    // Chỉ build trên Windows
    if cfg!(target_os = "windows") {
        generate_icon();
        embed_manifest();
    }
}

fn generate_icon() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Tạo icon 32x32 với gradient xanh dương -> xanh lá
    let size = 32;
    let mut rgba = Vec::with_capacity(size * size * 4);
    
    for y in 0..size {
        for x in 0..size {
            let center_x = size as f32 / 2.0;
            let center_y = size as f32 / 2.0;
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            let max_dist = center_x;
            
            if dist < max_dist - 2.0 {
                // Gradient từ xanh dương (#3498db) đến xanh lá (#2ecc71)
                let t = y as f32 / size as f32;
                let r = (52.0 * (1.0 - t) + 46.0 * t) as u8;
                let g = (152.0 * (1.0 - t) + 204.0 * t) as u8;
                let b = (219.0 * (1.0 - t) + 113.0 * t) as u8;
                rgba.extend_from_slice(&[b, g, r, 255]); // BGRA format for Windows
            } else if dist < max_dist {
                // Border trắng
                rgba.extend_from_slice(&[255, 255, 255, 255]);
            } else {
                // Transparent
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    
    // Tạo ICO file
    let ico_data = create_ico(&rgba, size);
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let icon_path = format!("{}/icon.ico", out_dir);
    
    std::fs::write(&icon_path, ico_data).expect("Failed to write icon file");
    println!("cargo:rustc-env=ICON_PATH={}", icon_path);
}

fn create_ico(rgba: &[u8], size: usize) -> Vec<u8> {
    let mut ico = Vec::new();
    
    // ICO header
    ico.extend_from_slice(&[0, 0]); // Reserved
    ico.extend_from_slice(&[1, 0]); // Type: 1 = ICO
    ico.extend_from_slice(&[1, 0]); // Count: 1 image
    
    // Image directory
    ico.push(size as u8);           // Width
    ico.push(size as u8);           // Height
    ico.push(0);                    // Color palette
    ico.push(0);                    // Reserved
    ico.extend_from_slice(&[1, 0]); // Color planes
    ico.extend_from_slice(&[32, 0]); // Bits per pixel
    
    // Image data size (BMP header + pixel data)
    let bmp_size = 40 + rgba.len();
    ico.extend_from_slice(&(bmp_size as u32).to_le_bytes());
    
    // Image data offset
    ico.extend_from_slice(&22u32.to_le_bytes());
    
    // BMP header (DIB)
    ico.extend_from_slice(&40u32.to_le_bytes()); // Header size
    ico.extend_from_slice(&(size as i32).to_le_bytes()); // Width
    ico.extend_from_slice(&(size as i32 * 2).to_le_bytes()); // Height * 2 (color + mask)
    ico.extend_from_slice(&[1, 0]); // Planes
    ico.extend_from_slice(&[32, 0]); // Bits per pixel
    ico.extend_from_slice(&[0; 4]); // Compression
    ico.extend_from_slice(&(rgba.len() as u32).to_le_bytes()); // Image size
    ico.extend_from_slice(&[0; 4]); // X pixels per meter
    ico.extend_from_slice(&[0; 4]); // Y pixels per meter
    ico.extend_from_slice(&[0; 4]); // Colors used
    ico.extend_from_slice(&[0; 4]); // Important colors
    
    // Pixel data (bottom-up)
    for y in (0..size).rev() {
        for x in 0..size {
            let idx = (y * size + x) * 4;
            ico.extend_from_slice(&rgba[idx..idx + 4]);
        }
    }
    
    ico
}

fn embed_manifest() {
    // Tạo Windows manifest
    let manifest = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
    name="RobloxBooster"
    version="1.0.0.0"
    type="win32"
  />
  <description>Roblox Performance Booster</description>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <!-- Windows 10 -->
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
      <!-- Windows 11 -->
      <supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>
    </application>
  </compatibility>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_path = format!("{}/roblox_booster.exe.manifest", out_dir);
    
    let mut file = std::fs::File::create(&manifest_path)
        .expect("Failed to create manifest file");
    file.write_all(manifest.as_bytes())
        .expect("Failed to write manifest");
    
    // Embed manifest và icon vào exe
    println!("cargo:rerun-if-changed=build.rs");
    
    // Tạo .rc file để embed resources
    let rc_content = format!(
        r#"1 ICON "{}/icon.ico"
1 24 "{}/roblox_booster.exe.manifest"
"#,
        out_dir.replace("\\", "/"),
        out_dir.replace("\\", "/")
    );
    
    let rc_path = format!("{}/resources.rc", out_dir);
    std::fs::write(&rc_path, rc_content).expect("Failed to write .rc file");
    
    // Compile .rc file (Windows only)
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        // Tìm rc.exe
        let rc_exe = find_rc_exe();
        if let Some(rc) = rc_exe {
            let res_path = format!("{}/resources.res", out_dir);
            
            let status = Command::new(rc)
                .args(&["/fo", &res_path, &rc_path])
                .status();
            
            if status.is_ok() {
                println!("cargo:rustc-link-arg={}", res_path);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn find_rc_exe() -> Option<String> {
    use std::path::PathBuf;
    // Tìm Windows SDK
    let program_files = std::env::var("ProgramFiles(x86)")
        .or_else(|_| std::env::var("ProgramFiles"))
        .ok()?;
    
    let sdk_path = PathBuf::from(program_files)
        .join("Windows Kits")
        .join("10")
        .join("bin");
    
    if !sdk_path.exists() {
        return None;
    }
    
    // Tìm version mới nhất
    let mut versions: Vec<_> = std::fs::read_dir(&sdk_path)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok()?.is_dir())
        .collect();
    
    versions.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
    
    for version in versions {
        let rc_path = version.path().join("x64").join("rc.exe");
        if rc_path.exists() {
            return Some(rc_path.to_string_lossy().to_string());
        }
        
        let rc_path = version.path().join("x86").join("rc.exe");
        if rc_path.exists() {
            return Some(rc_path.to_string_lossy().to_string());
        }
    }
    
    None
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn find_rc_exe() -> Option<String> {
    None
}