use std::io::{ Write };
use std::fs::File;

use crate::common::{CharacterSlot, ItemSlots, CHECKSUM_OFFSET, SLOTS_OFFSET, SLOTS_TOGGLE_OFFSET, RGBValue };

fn write_u8(buf: &mut Vec<u8>, address: usize, data: u8)
{
    buf[address] = data;
}

fn write_u16(buf: &mut Vec<u8>, address: usize, data: u16)
{
    buf[address    ] = ((data >> 8) & 0xFF) as u8;
    buf[address + 1] = ((data)      & 0xFF) as u8;
}

fn write_u32(buf: &mut Vec<u8>, address: usize, data: u32)
{
    buf[address    ] = ((data >> 24) & 0xFF) as u8;
    buf[address + 1] = ((data >> 16) & 0xFF) as u8;
    buf[address + 2] = ((data >> 8)  & 0xFF) as u8;
    buf[address + 3] = ((data)       & 0xFF) as u8;
}

fn write_name(buf: &mut Vec<u8>, address: usize, str: [u8; 8])
{
    for k in 0..8 {
        buf[address + k] = str[k];
    }
}

fn write_rgb(buf: &mut Vec<u8>, address: usize, data: RGBValue)
{
    buf[address    ] = data.0;
    buf[address + 1] = data.1;
    buf[address + 2] = data.2;
}

fn item_slots_to_buf(container: &ItemSlots, buf: &mut Vec<u8>, slot_n: usize)
{
    for k in 0..container.data.len() {
        write_u16(buf,
            SLOTS_OFFSET[slot_n] + container.offset + (k * 4), container.data[k].id);
        write_u16(buf,
            SLOTS_OFFSET[slot_n] + container.offset + (k * 4) + 2, container.data[k].qty as u16);
    }
}

pub fn save_to_buf(slot: &CharacterSlot, buf: &mut Vec<u8>, slot_n: usize)
{
    write_u8(buf,  SLOTS_TOGGLE_OFFSET[slot_n] + slot.slot_enabled.offset, slot.slot_enabled.data );
    write_u8(buf,  SLOTS_OFFSET[slot_n] + slot.gender.offset  , slot.gender.data     );
    write_name(buf, SLOTS_OFFSET[slot_n] + slot.name.offset    , slot.name.data  );
    write_u32(buf, SLOTS_OFFSET[slot_n] + slot.zenny.offset   , slot.zenny.data      );
    write_u32(buf, SLOTS_OFFSET[slot_n] + slot.playtime.offset, slot.playtime.data   );
    item_slots_to_buf(&slot.b_pouch, buf, slot_n);
    item_slots_to_buf(&slot.g_pouch, buf, slot_n);
    item_slots_to_buf(&slot.item_box, buf, slot_n);
    for k in 0..slot.equipment_box.data.len() {
        write_u8(buf,
            SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12), slot.equipment_box.data[k].0 as u8);
        write_u8(buf,
            SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 1, slot.equipment_box.data[k].1);
        write_u16(buf,
            SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 2, slot.equipment_box.data[k].2);
        write_u8(buf,
            SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 4, slot.equipment_box.data[k].3);
        write_u8(buf,
            SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 5, slot.equipment_box.data[k].4);

        for i in 0..3 {
            write_u16(buf,
                SLOTS_OFFSET[slot_n] + slot.equipment_box.offset + (k * 12) + 6 + (i * 2),
                slot.equipment_box.data[k].5[i]);
        }
    }
    write_u32(buf,  SLOTS_OFFSET[slot_n] + slot.hrp.offset , slot.hrp.data     );
    write_u16(buf,  SLOTS_OFFSET[slot_n] + slot.hr.offset  , slot.hr.data      );

    write_u8(buf ,  SLOTS_OFFSET[slot_n] + slot.face_type.offset   , slot.face_type.data     );
    write_u8(buf ,  SLOTS_OFFSET[slot_n] + slot.hair_type.offset   , slot.hair_type.data     );
    write_rgb(buf,  SLOTS_OFFSET[slot_n] + slot.hair_color.offset  , slot.hair_color.data    );
    write_u8(buf ,  SLOTS_OFFSET[slot_n] + slot.cloth_type.offset  , slot.cloth_type.data    );
    write_u8(buf ,  SLOTS_OFFSET[slot_n] + slot.voice_type.offset  , slot.voice_type.data    );
    write_rgb(buf,  SLOTS_OFFSET[slot_n] + slot.cloth_color.offset , slot.cloth_color.data   );
    write_u8(buf ,  SLOTS_OFFSET[slot_n] + slot.eye_color.offset   , slot.eye_color.data     );
    write_u8(buf ,  SLOTS_OFFSET[slot_n] + slot.feature_type.offset, slot.feature_type.data  );
    write_u16(buf,  SLOTS_OFFSET[slot_n] + slot.skin_tone.offset   , slot.skin_tone.data     );
}

pub fn buf_to_file(filepath: &String, buf: &mut Vec<u8>) -> Result<(), std::io::Error>
{
    let checksum: u32 = buf
        .iter()
        .skip(8)
        .map(|&x| x as u32)
        .sum();
    println!("Resulting checksum: 0x{:08X}", checksum);

    write_u32(buf, CHECKSUM_OFFSET, checksum);

    let mut file = File::create(filepath)?;
    file.write_all(buf.as_slice())?;
    Ok(())
}