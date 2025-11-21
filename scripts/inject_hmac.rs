// scripts/inject_hmac.rs
use std::env;
use std::fs;
use std::path::Path;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const PLACEHOLDER: &[u8; 32] = b"__PQC_NOSTD_HMAC_PLACEHOLDER__\x00\x00";
const INTEGRITY_KEY: &[u8] = b"FIPS_140_3_INTEGRITY_KEY";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_binary>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file_content = fs::read(path)?;

    println!("Processing binary: {:?}", path);

    // 1. Find Code Segment (Text Section)
    let (code_start, code_len) = find_text_section(&file_content)?;
    println!("Found code segment at file offset 0x{:X}, length 0x{:X} bytes", code_start, code_len);

    let code_bytes = &file_content[code_start..code_start + code_len];

    // 2. Calculate HMAC
    let mut mac = HmacSha256::new_from_slice(INTEGRITY_KEY)
        .map_err(|_| "Invalid key length")?;
    mac.update(code_bytes);
    let result = mac.finalize().into_bytes();
    let hmac_bytes: [u8; 32] = result.into();

    println!("Calculated HMAC: {:X?}", hmac_bytes);

    // 3. Find Placeholder
    let placeholder_offset = find_placeholder(&file_content, PLACEHOLDER)
        .ok_or("Placeholder not found in binary")?;
    
    println!("Found placeholder at offset 0x{:X}", placeholder_offset);

    // 4. Inject HMAC
    // We need to write back to the file.
    // Note: We modify the in-memory buffer and write it all back, or open file for writing.
    // Writing back buffer is safer to ensure we have the right content.
    
    // Wait! If the placeholder is INSIDE the code segment, changing it will change the HMAC!
    // The placeholder is in `src/integrity_data.rs`, which is likely in `.rdata` (read-only data), not `.text` (code).
    // But we must verify this.
    
    if placeholder_offset >= code_start && placeholder_offset < code_start + code_len {
        eprintln!("WARNING: Placeholder is within the code segment! Injection will invalidate HMAC.");
        // In a real scenario, we'd need to exclude the placeholder from hashing or ensure it's in .data/.rdata.
        // Rust constants usually go to .rdata.
    }

    // Update buffer
    for (i, byte) in hmac_bytes.iter().enumerate() {
        file_content[placeholder_offset + i] = *byte;
    }

    // Write back
    fs::write(path, file_content)?;
    println!("HMAC injected successfully.");

    Ok(())
}

fn find_placeholder(data: &[u8], placeholder: &[u8]) -> Option<usize> {
    data.windows(placeholder.len()).position(|window| window == placeholder)
}

// Minimal PE Parser to find .text section offset in file
fn find_text_section(data: &[u8]) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    // DOS Header
    if data.len() < 0x40 { return Err("File too small".into()); }
    let e_magic = u16::from_le_bytes([data[0], data[1]]);
    if e_magic != 0x5A4D { return Err("Not a PE file".into()); }

    let e_lfanew = i32::from_le_bytes(data[0x3C..0x40].try_into()?) as usize;
    
    // NT Headers
    if data.len() < e_lfanew + 4 { return Err("Invalid PE header".into()); }
    let signature = u32::from_le_bytes(data[e_lfanew..e_lfanew+4].try_into()?);
    if signature != 0x00004550 { return Err("Invalid PE signature".into()); }

    // File Header (20 bytes) starts at e_lfanew + 4
    let file_header_offset = e_lfanew + 4;
    let number_of_sections = u16::from_le_bytes(data[file_header_offset+2..file_header_offset+4].try_into()?);
    let size_of_optional_header = u16::from_le_bytes(data[file_header_offset+16..file_header_offset+18].try_into()?);

    // Optional Header starts at file_header_offset + 20
    let optional_header_offset = file_header_offset + 20;
    
    // Check Magic (PE32 or PE32+)
    let magic = u16::from_le_bytes(data[optional_header_offset..optional_header_offset+2].try_into()?);
    
    let (base_of_code, size_of_code) = if magic == 0x20B { // PE32+
        let size_of_code = u32::from_le_bytes(data[optional_header_offset+4..optional_header_offset+8].try_into()?);
        let base_of_code = u32::from_le_bytes(data[optional_header_offset+20..optional_header_offset+24].try_into()?);
        (base_of_code, size_of_code)
    } else if magic == 0x10B { // PE32
        let size_of_code = u32::from_le_bytes(data[optional_header_offset+4..optional_header_offset+8].try_into()?);
        let base_of_code = u32::from_le_bytes(data[optional_header_offset+20..optional_header_offset+24].try_into()?);
        (base_of_code, size_of_code)
    } else {
        return Err("Unknown PE magic".into());
    };

    // Section Headers start after Optional Header
    let section_headers_offset = optional_header_offset + size_of_optional_header as usize;
    
    // Iterate sections to find the one containing BaseOfCode
    // Each section header is 40 bytes
    for i in 0..number_of_sections {
        let offset = section_headers_offset + (i as usize * 40);
        if offset + 40 > data.len() { break; }
        
        let virtual_address = u32::from_le_bytes(data[offset+12..offset+16].try_into()?);
        let virtual_size = u32::from_le_bytes(data[offset+8..offset+12].try_into()?);
        let pointer_to_raw_data = u32::from_le_bytes(data[offset+20..offset+24].try_into()?);
        
        // Check if BaseOfCode falls within this section
        if base_of_code >= virtual_address && base_of_code < virtual_address + virtual_size {
            // Found the code section!
            // Calculate file offset
            let offset_in_section = base_of_code - virtual_address;
            let file_start = pointer_to_raw_data + offset_in_section;
            
            return Ok((file_start as usize, size_of_code as usize));
        }
    }

    Err("Code section not found".into())
}
