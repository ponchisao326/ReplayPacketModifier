use std::collections::HashMap; // <-- Añadir esta importación
use std::fs::File;
use std::io::{self, Read, Write, Cursor, Result};
use std::path::Path;
use zip::read::ZipArchive;
use zip::write::{FileOptions, ZipWriter};
use crate::utils::{read_int, write_int, read_varint};

fn process_tmcpr(data: &[u8], filter_ids: &[u32]) -> Result<(Vec<u8>, HashMap<u32, u32>)> {
    let mut cursor = Cursor::new(data);
    let mut output = Vec::new();
    let mut filter_counts = HashMap::new();

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

        let (packet_id, _) = read_varint(&packet_data).unwrap_or((0xFFFF, 0));

        if filter_ids.contains(&packet_id) {
            println!("Filtering packet ID: 0x{:02X}", packet_id);
            *filter_counts.entry(packet_id).or_insert(0) += 1;
            continue;
        }

        write_int(&mut output, timestamp)?;
        write_int(&mut output, length)?;
        output.extend_from_slice(&packet_data);
        println!("Processed packet ID: 0x{:02X}, timestamp: {}, length: {}", packet_id, timestamp, length);
    }
    Ok((output, filter_counts))
}

pub fn process_mcpr(input_path: &Path, output_path: &Path, filter_ids: &[u32]) -> Result<()> {
    let input_file = File::open(input_path)?;
    let mut zip_archive = ZipArchive::new(input_file)?;

    let output_file = File::create(output_path)?;
    let mut zip_writer = ZipWriter::new(output_file);
    let options: FileOptions<()> = FileOptions::default();

    let mut total_counts = HashMap::new();

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let name = file.name().to_string();
        zip_writer.start_file(name.clone(), options)?;

        if name == "recording.tmcpr" {
            let mut original_data = Vec::new();
            file.read_to_end(&mut original_data)?;
            let (modified_data, counts) = process_tmcpr(&original_data, filter_ids)?;
            zip_writer.write_all(&modified_data)?;

            for (id, count) in counts {
                *total_counts.entry(id).or_insert(0) += count;
            }
        } else {
            io::copy(&mut file, &mut zip_writer)?;
        }
    }


    if !total_counts.is_empty() {
        println!("\nFiltered packet counts:");
        for (id, count) in total_counts {
            println!("Packet ID 0x{:02X}: {} packets removed", id, count);
        }
    } else {
        println!("\nNo packets were filtered.");
    }

    zip_writer.finish()?;
    Ok(())
}