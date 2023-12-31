use std::env;
use std::process;

mod common;
mod save_read;
mod csv_read;
mod save_write;
mod csv_write;

use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::common::{ DataIDs, CharacterSlot, SLOTS_OFFSET, SLOT_SIZE, SLOTS_TOGGLE_START };
use crate::save_read::{ file_to_buf, buf_to_save };
use crate::csv_read::{ file_to_csv, csv_to_save };
use crate::csv_write::{ save_to_csv, csv_to_file, };
use crate::save_write::{ save_to_buf, buf_to_file };

fn copy_slots(filepath: &String, origin_slot: usize, dest_slot: usize) -> Result<(), std::io::Error>
{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filepath)?;

    let mut buf: Vec<u8> = vec![0 ; SLOT_SIZE];
    file.seek(SeekFrom::Start(SLOTS_OFFSET[origin_slot] as u64)).unwrap();
    file.read_exact(&mut buf)?;
    file.seek(SeekFrom::Start(SLOTS_OFFSET[dest_slot] as u64)).unwrap();
    file.write_all(&buf)?;

    // Copy toggle flags
    let mut toggle_buf: Vec<u8> = vec![0 ; 3];
    file.seek(SeekFrom::Start(SLOTS_TOGGLE_START as u64)).unwrap();
    file.read_exact(&mut toggle_buf)?;

    toggle_buf[dest_slot] = toggle_buf[origin_slot];

    file.seek(SeekFrom::Start(SLOTS_TOGGLE_START as u64)).unwrap();
    file.write_all(&toggle_buf)?;
    file.flush()?;

    // Recompute checksum
    let mut full_buf = file_to_buf(filepath)?;
    buf_to_file(filepath, &mut full_buf)?;
    Ok(())
}

fn swap_slots(filepath: &String, slot_a: usize, slot_b: usize) -> Result<(), std::io::Error>
{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filepath)?;

    let mut buf_a: Vec<u8> = vec![0 ; SLOT_SIZE];
    file.seek(SeekFrom::Start(SLOTS_OFFSET[slot_a] as u64)).unwrap();
    file.read_exact(&mut buf_a)?;

    let mut buf_b: Vec<u8> = vec![0 ; SLOT_SIZE];
    file.seek(SeekFrom::Start(SLOTS_OFFSET[slot_b] as u64)).unwrap();
    file.read_exact(&mut buf_b)?;

    file.seek(SeekFrom::Start(SLOTS_OFFSET[slot_a] as u64)).unwrap();
    file.write_all(&buf_b)?;

    file.seek(SeekFrom::Start(SLOTS_OFFSET[slot_b] as u64)).unwrap();
    file.write_all(&buf_a)?;

    // Swap toggle flags
    let mut toggle_buf: Vec<u8> = vec![0 ; 3];
    file.seek(SeekFrom::Start(SLOTS_TOGGLE_START as u64)).unwrap();
    file.read_exact(&mut toggle_buf)?;

    toggle_buf.swap(slot_a, slot_b);

    file.seek(SeekFrom::Start(SLOTS_TOGGLE_START as u64)).unwrap();
    file.write_all(&toggle_buf)?;

    Ok(())
}

fn print_help()
{
    println!("Usage: ");
    println!("\tmh3se encode [save_file] [csv_file] [slot]");
    println!("\tmh3se decode [save_file] [csv_file] [slot]");
    println!("\tmh3se copy [save_file] [origin_slot] [destination_slot]");
    println!("\tmh3se swap [save_file] [slot_a] [slot_b]");
}

fn process_args(args: &Vec<String>) -> Result<(), ()>
{
    if args[0] == "copy" || args[0] == "swap"
    {
        let slot_a: usize;
        match args[2].parse::<usize>() {
            Ok(number) => {
                slot_a = number - 1;
                println!("Slot {} selected !", slot_a + 1);
            },
            Err(err) => {
                eprintln!("Couldn't select requested slot {} !", err);
                return Err(());
            }
        }
        let slot_b: usize;
        match args[3].parse::<usize>() {
            Ok(number) => {
                slot_b = number - 1;
                println!("Slot {} selected !", slot_b + 1);
            },
            Err(err) => {
                eprintln!("Couldn't select requested slot {} !", err);
                return Err(());
            }
        }
        if args[0] == "copy"
        {
            match copy_slots(&args[1], slot_a, slot_b)
            {
                Ok(_) => println!("Successfully copied slot {} to slot {} !", slot_a + 1, slot_b + 1),
                Err(err) => {
                    eprintln!("Couldn't copy slot {} to slot {} ({}) !", slot_a + 1, slot_b + 1, err);
                    return Err(());
                }
            }
        }
        else
        {
            match swap_slots(&args[1], slot_a, slot_b)
            {
                Ok(_) => println!("Successfully swapped slot {} and slot {} !", slot_a + 1, slot_b + 1),
                Err(err) => {
                    eprintln!("Couldn't swap slot {} and slot {} ({}) !", slot_a + 1, slot_b + 1, err);
                    return Err(());
                }
            }
        }
    }
    else if args[0] == "decode" || args[0] == "encode"
    {
        let ids: DataIDs;
        #[cfg(target_os = "windows")]
        {
            match DataIDs::new(
                &String::from("data\\genders.txt"),
                &String::from("data\\items.txt"),
                &String::from("data\\armors.txt"),
                &String::from("data\\weapons.txt"),
                &String::from("data\\gun_parts.txt"),
                &String::from("data\\skills.txt")
            )
            {
                Ok(res) => {
                    ids = res;
                    println!("Loaded database files successfully !");
                }
                Err(err) => {
                    eprintln!("Couldn't load database files {} !", err);
                    return Err(());
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            match DataIDs::new(
                &String::from("data/genders.txt"),
                &String::from("data/items.txt"),
                &String::from("data/armors.txt"),
                &String::from("data/weapons.txt"),
                &String::from("data/gun_parts.txt"),
                &String::from("data/skills.txt")
            )
            {
                Ok(res) => {
                    ids = res;
                    println!("Loaded database files successfully !");
                }
                Err(err) => {
                    eprintln!("Couldn't load database files {} !", err);
                    return Err(());
                }
            }
        }

        let save_path = &args[1];
        let csv_path = &args[2];
        let character_slot: usize;

        match args[3].parse::<usize>() {
            Ok(number) => {
                character_slot = number - 1;
                println!("Slot {} selected !", character_slot + 1);
            },
            Err(err) => {
                eprintln!("Couldn't select requested slot {} !", err);
                return Err(());
            }
        }

        if args[0] == "decode"
        {
            let buffer: Vec<u8>;
            match file_to_buf(&String::from(save_path)) {
                Ok(buf) => {
                    buffer = buf;
                    println!("Loaded save file {} successfully !", save_path);
                },
                Err(err) => {
                    eprintln!("Couldn't load save file {} ({}) !", save_path, err);
                    return Err(());
                }
            }
            let mut slot: CharacterSlot = CharacterSlot::default();
            let mut csv: Vec<String> = Vec::new();
            buf_to_save(&buffer, &mut slot, character_slot);
            save_to_csv(&slot, &mut csv, &ids);
            match csv_to_file(&String::from(csv_path), &csv) {
                Ok(_) => {
                    println!("Data written to csv file {} successfully !", csv_path);
                },
                Err(err) => {
                    eprintln!("Couldn't write data to csv file {} ({}) !", csv_path, err);
                    return Err(());
                }
            }

        }
        else
        {
            let csv: Vec<String>;
            match file_to_csv(csv_path) {
                Ok(res) => {
                    csv = res;
                    println!("Loaded csv file {} successfully !", csv_path);
                }
                Err(err) => {
                    eprintln!("Couldn't load csv file {} ({}) !", csv_path, err);
                    return Err(());
                }
            }

            let mut slot: CharacterSlot = CharacterSlot::default();
            match csv_to_save(&csv, &mut slot, &ids) {
                Ok(_) => println!("Parsed csv file {} successfully !", csv_path),
                Err(err) => {
                    eprintln!("Couldn't parse csv file {} ({})", csv_path, err);
                    return Err(());
                }
            }

            let mut buffer: Vec<u8>;
            match file_to_buf(save_path) {
                Ok(buf) => {
                    buffer = buf;
                    println!("Loaded save file {} successfully !", save_path);
                },
                Err(err) => {
                    eprintln!("Couldn't load save file {} ({}) !", save_path, err);
                    return Err(());
                }
            }
            save_to_buf(&slot, &mut buffer, character_slot);
            match buf_to_file(save_path, &mut buffer) {
                Ok(_) => {
                    println!("Data written to save file {} successfully !", save_path);
                },
                Err(err) => {
                    eprintln!("Couldn't write data to save file {} ({}) !", save_path, err);
                    return Err(());
                }
            }
        }
    }
    else {
        print_help();
        return Err(());
    }
    Ok(())
}

fn main()
{
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("mh3se v{} | mh3 save-editor", VERSION);

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 4 {
        let _ = process_args(&args);
    }
    else {
        let mut manual_args: Vec<String> = vec![ String::from("") ; 4 ];
        println!("Enter the desired operation (encode, decode, copy or swap) : ");
        let _ = std::io::stdin().read_line(&mut manual_args[0]);
        println!("Enter the save file path : ");
        let _ = std::io::stdin().read_line(&mut manual_args[1]);

        manual_args[0] = manual_args[0].trim().to_string();
        manual_args[1] = manual_args[1].trim().to_string();

        if manual_args[0] == "encode" || manual_args[0] == "decode" {
            println!("Enter the spreadsheet path : ");
            let _ = std::io::stdin().read_line(&mut manual_args[2]);
            println!("Enter the character slot (1, 2, 3) : ");
            let _ = std::io::stdin().read_line(&mut manual_args[3]);
        }
        else if manual_args[0] == "copy" {
            println!("Enter the character origin slot (1, 2, 3) : ");
            let _ = std::io::stdin().read_line(&mut manual_args[2]);
            println!("Enter the character destination slot (1, 2, 3) : ");
            let _ = std::io::stdin().read_line(&mut manual_args[3]);
        }
        else if manual_args[0] == "swap" {
            println!("Enter the character slot a (1, 2, 3) : ");
            let _ = std::io::stdin().read_line(&mut manual_args[2]);
            println!("Enter the character slot b (1, 2, 3) : ");
            let _ = std::io::stdin().read_line(&mut manual_args[3]);
        }
        else {
            println!("Unknown operation ({})", manual_args[0]);
        }
        manual_args[2] = manual_args[2].trim().to_string();
        manual_args[3] = manual_args[3].trim().to_string();
        let res = process_args(&manual_args);
        print!("Press any key to exit...");
        let _ = std::io::stdin().lines().next();
        match res {
            Ok(_) => process::exit(0),
            Err(_) => process::exit(1)
        }
    }
    process::exit(0);
}