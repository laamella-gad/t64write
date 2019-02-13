use std::fs::File;
use std::io::prelude::*;
use byteorder::*;
use std::io;
use std::result;
use std::env;
use std::error::Error;
use std::process;

struct Config {
    tape_name: String,
    prg_name: String,
}

impl Config {
    fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return result::Result::Err("not enough arguments");
        }

        let tape_name = args[1].clone();
        let prg_name = args[2].clone();

        Ok(Config { tape_name, prg_name })
    }
}

struct Prg {
    data: Vec<u8>,
    start_address: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("Usage: t64write TAPEFILE PRGFILE");
        process::exit(1);
    });

    let prg = read_prg(&config.prg_name)?;

    let mut file = File::create(config.tape_name)?;
    write_tape_record(&mut file)?;
    write_file_record(&mut file, prg.start_address, prg.data.len(), &make_c64_file_name(&config.prg_name))?;
    write_prg(&mut file, prg.data)?;

    Ok(())
}

fn make_c64_file_name(prg_name: &String) -> String {
    let y = prg_name.to_ascii_uppercase();
    let x: Vec<&str> = y.split('.').collect();

    String::from(x[0])
}

fn write_tape_record(file: &mut File) -> Result<(), io::Error> {
    // T64 ID
    file.write_all(format!("{:\0<32}", "C64S tape file").as_bytes())?;
    // Tape version
    file.write_u16::<LittleEndian>(0x0100)?;
    // Number of tape entries
    file.write_u16::<LittleEndian>(0x0001)?;
    // Number of used entries
    file.write_u16::<LittleEndian>(0x0001)?;
    // Unused
    file.write_u16::<LittleEndian>(0xcafe)?;
    // Tape container name
    file.write_all("DEMOTAPEDEMOTAPEDEMOTAPE".as_bytes())?;

    Ok(())
}

enum EntryType {
    _FreeEntry = 0,
    NormalTapeFile = 1,
    _TapeFileWithHeader = 2,
    _MemorySnapshot = 3,
    _TapeBlock = 4,
    _DigitizedStream = 5,
}

fn write_file_record(file: &mut File, start_address: u16, prg_size: usize, c64_file_name: &String) -> Result<(), io::Error> {
    // Entry type
    file.write_u8(EntryType::NormalTapeFile as u8)?;
    // C64 file type
    file.write_u8(0x82)?;
    //  Start address
    file.write_u16::<LittleEndian>(start_address)?;
    // End address
    file.write_u16::<LittleEndian>(start_address + prg_size as u16)?;
    // Unused
    file.write_u16::<LittleEndian>(0xcafe)?;
    // Offset of file contents start withing file
    file.write_u32::<LittleEndian>(0x0068)?;
    // Unused
    file.write_u32::<LittleEndian>(0xcafecafe)?;
    //  C64 filename
    file.write_all(format!("{: <24}", c64_file_name).as_bytes())?;

    Ok(())
}

fn read_prg(prg_name: &String) -> Result<Prg, io::Error> {
    let mut buffer = [0; 0x10000];

    let mut prg_file = File::open(prg_name)?;
    let start_address = prg_file.read_u16::<LittleEndian>()?;
    let len_read = prg_file.read(&mut buffer)?;

    let data = buffer[..len_read].to_vec();

    Ok(Prg { data, start_address })
}

fn write_prg(file: &mut File, prg: Vec<u8>) -> Result<(), io::Error> {
    file.write_all(&prg)?;

    Ok(())
}