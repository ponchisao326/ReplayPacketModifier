use std::fs::File;
use std::io::{self, Read, Write, Cursor, Result};
use std::path::Path;
use zip::read::ZipArchive;
use zip::write::{FileOptions, ZipWriter};
use crate::utils::{read_int, write_int, read_varint};

/// Processes the content of the recording.tmcpr file, filtering packets whose codes
/// are in `filter_ids`. Assumes the format:
/// uint32_t timestamp; uint32_t packet_size; uint8_t packet_data[];
fn process_tmcpr(data: &[u8], filter_ids: &[u32]) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(data);
    let mut output = Vec::new();

    while (cursor.position() as usize) < data.len() {
        let timestamp = match read_int(&mut cursor) {
            Ok(val) => val,
            Err(_) => break,
        };
        let length = match read_int(&mut cursor) {
            Ok(val) => val,
            Err(_) => break,
        };

        if length <= 0 || (length as usize) > 10_000_000 {
            eprintln!("Invalid length: {}", length);
            break;
        }

        let mut packet_data = vec![0u8; length as usize];
        cursor.read_exact(&mut packet_data)?;

        // Read the VarInt at the start of the data to get the packet_id
        let (packet_id, _) = read_varint(&packet_data).unwrap_or((0xFFFF, 0));

        // If the packet_id is in the filter list, skip it
        if filter_ids.contains(&packet_id) {
            println!("Filtering packet ID: 0x{:02X}", packet_id);
            continue;
        }

        write_int(&mut output, timestamp)?;
        write_int(&mut output, length)?;
        output.extend_from_slice(&packet_data);
        println!("Processed packet ID: 0x{:02X}, timestamp: {}, length: {}", packet_id, timestamp, length);
    }
    Ok(output)
}

/// Processes the .mcpr file: extracts the "recording.tmcpr" entry, modifies it, and generates a new .mcpr.
pub fn process_mcpr(input_path: &Path, output_path: &Path, filter_ids: &[u32]) -> Result<()> {
    let input_file = File::open(input_path)?;
    let mut zip_archive = ZipArchive::new(input_file)?;

    let output_file = File::create(output_path)?;
    let mut zip_writer = ZipWriter::new(output_file);
    let options: FileOptions<()> = FileOptions::default();


    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let name = file.name().to_string();
        zip_writer.start_file(name.clone(), options)?;

        if name == "recording.tmcpr" {
            let mut original_data = Vec::new();
            file.read_to_end(&mut original_data)?;
            let modified_data = process_tmcpr(&original_data, filter_ids)?;
            zip_writer.write_all(&modified_data)?;
        } else {
            io::copy(&mut file, &mut zip_writer)?;
        }
    }
    zip_writer.finish()?;
    Ok(())
}
