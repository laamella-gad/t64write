extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::prelude::*;
use byteorder::*;

fn main() -> std::io::Result<()> {
    let matches = App::new("t64write")
        .version("1.0.0")
        .author("Danny van Bruggen <hexagonaal@gmail.com>")
        .about("Commodore 64 tape image creator")
        .arg(Arg::with_name("tapefile")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("name of the t64 file to create"))
        .arg(Arg::with_name("prg-file")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("name of the prg file to put on the tape"))
        .get_matches();

    let tape_name = matches.value_of("tapefile").unwrap();
    println!("tape file {}", tape_name);
    let prg_name = matches.value_of("prg-file").unwrap();
    println!("prg file {}", prg_name);

    //
    let mut file = File::create(tape_name)?;
    write_tape_record(&mut file);
    let (prg, start_address) = read_prg(prg_name);

    write_file_record(&mut file, start_address, prg.len(), make_c64_file_name(prg_name));
    write_prg(&mut file, prg);

    Ok(())
}

fn make_c64_file_name(prg_name: &str) -> String {
    let y = prg_name.to_ascii_uppercase();
    let x: Vec<&str> = y.split('.').collect();

    String::from(x[0])
}

//struct TapeRecord {
//    dos_tape_description: [u8; 32],
//    tape_version: u16,
//    number_of_directory_entries: u16,
//    number_of_used_entries: u16,
//    unused: u16,
//    user_description: [u8; 24],
//}
//
fn write_tape_record(file: &mut File) {
    // T64 ID
    file.write_all(format!("{:\0<32}", "C64S tape file").as_bytes()).unwrap();
    // Tape version
    file.write_u16::<LittleEndian>(0x0100).unwrap();
    // Number of tape entries
    file.write_u16::<LittleEndian>(0x0001).unwrap();
    // Number of used entries
    file.write_u16::<LittleEndian>(0x0001).unwrap();
    // Unused
    file.write_u16::<LittleEndian>(0xcafe).unwrap();
    // Tape container name
    file.write_all("DEMOTAPEDEMOTAPEDEMOTAPE".as_bytes()).unwrap();
}

//struct FileRecord {
//    entry_type: u8,
//    c64_file_type: u8,
//    start_address: u16,
//    end_address: u16,
//    unused: u16,
//    offset_of_file_contents_start_within_t64_file: u32,
//    unused2: u32,
//    c64_file_name: [u8; 16],
//}
//
//enum EntryType {
//    FreeEntry = 0,
//    NormalTapeFile = 1,
//    TapeFileWithHeader = 2,
//    MemorySnapshot = 3,
//    TapeBlock = 4,
//    DigitizedStream = 5,
//}
fn write_file_record(file: &mut File, start_address: u16, prg_size: usize, c64_file_name: String) {
    // Entry type
    file.write_u8(1).unwrap();
    // C64 file type
    file.write_u8(0x82).unwrap();
    //  Start address
    file.write_u16::<LittleEndian>(start_address).unwrap();
    // End address
    file.write_u16::<LittleEndian>(start_address + prg_size as u16).unwrap();
    // Unused
    file.write_u16::<LittleEndian>(0xcafe).unwrap();
    // Offset of file contents start withing file
    file.write_u32::<LittleEndian>(0x0068).unwrap();
    // Unused
    file.write_u32::<LittleEndian>(0xcafecafe).unwrap();
    //  C64 filename
    file.write_all(format!("{: <24}", c64_file_name).as_bytes()).unwrap();
}

fn read_prg(prg_name: &str) -> (Vec<u8>, u16) {
    let mut buffer = [0; 0x10000];

    let mut prg_file = File::open(prg_name).unwrap();
    let start = prg_file.read_u16::<LittleEndian>().unwrap();
    let len_read = prg_file.read(&mut buffer).unwrap();

    let prg = buffer[..len_read].to_vec();

    (prg, start)
}

fn write_prg(file: &mut File, prg: Vec<u8>) {
    file.write_all(&prg).unwrap();
}