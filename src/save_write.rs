use std::fs::File;
use std::io::Write;
use std::ptr;

use crate::common::*;

fn write_i8(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: i8) -> Result<(), String> {
    if SAVE_SIZE < offset + 1 {
        return Err(format!("Cannot write u8 at {:06X}", offset));
    }
    buffer[offset] = value as u8;
    Ok(())
}

fn write_u8(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: u8) -> Result<(), String> {
    if SAVE_SIZE < offset + 1 {
        return Err(format!("Cannot write u8 at {:06X}", offset));
    }
    buffer[offset] = value;
    Ok(())
}
fn write_u16(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: u16) -> Result<(), String> {
    if SAVE_SIZE < offset + 2 {
        return Err(format!("Cannot write u16 at {:06X}", offset));
    }
    buffer[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
    Ok(())
}
fn write_u32(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: u32) -> Result<(), String> {
    if SAVE_SIZE < offset + 4 {
        return Err(format!("Cannot write u32 at {:06X}", offset));
    }
    buffer[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    Ok(())
}
fn write_name(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: [u8; 8]) -> Result<(), String> {
    if SAVE_SIZE < offset + 8 {
        return Err(format!("Cannot write name at {:06X}", offset));
    }
    buffer[offset..offset + 8].copy_from_slice(&value);
    Ok(())
}
fn write_rgb(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: [u8; 3]) -> Result<(), String> {
    if SAVE_SIZE < offset + 3 {
        return Err(format!("Cannot write rgb at {:06X}", offset));
    }
    buffer[offset..offset + 3].copy_from_slice(&value);
    Ok(())
}

fn write_item_slot(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: ItemSlot) -> Result<(), String> {
    write_u16(buffer, offset, value.id)?;
    write_u16(buffer, offset + 2, value.qty)?;
    Ok(())
}

fn write_equip_slot(buffer: &mut [u8; SAVE_SIZE], offset: usize, value: EquipSlot) -> Result<(), String> {
    unsafe {
        match value {
            EquipSlot::Armor(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u8(buffer, offset + 1, raw.lvl)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_u16(buffer, offset + 6, raw.deco1)?;
                write_u16(buffer, offset + 8, raw.deco2)?;
                write_u16(buffer, offset + 10, raw.deco3)?;
            },
            EquipSlot::MeleeWeapon(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_u16(buffer, offset + 6, raw.deco1)?;
                write_u16(buffer, offset + 8, raw.deco2)?;
                write_u16(buffer, offset + 10, raw.deco3)?;
            },
            EquipSlot::RangedWeapon(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u8(buffer, offset + 1, raw.lvl)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_u16(buffer, offset + 6, raw.deco1)?;
                write_u16(buffer, offset + 8, raw.deco2)?;
                write_u16(buffer, offset + 10, raw.deco3)?;
            },
            EquipSlot::Charm0Slot(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u8(buffer, offset + 1, raw.slot_count)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_i8(buffer, offset + 5, raw.skill1_pt)?;
                write_i8(buffer, offset + 4, raw.skill2_pt)?;
                write_u16(buffer, offset + 6, raw.skill1_id)?;
                write_u16(buffer, offset + 8, raw.skill2_id)?;
            },
            EquipSlot::Charm1Slot(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u8(buffer, offset + 1, raw.slot_count)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_i8(buffer, offset + 5, raw.skill1_pt)?;
                write_i8(buffer, offset + 4, raw.skill2_pt)?;
                write_u16(buffer, offset + 6, raw.deco1)?;
                write_u16(buffer, offset + 8, raw.skill1_id)?;
                write_u16(buffer, offset + 10, raw.skill2_id)?;
            },
            EquipSlot::Charm2Slot(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u8(buffer, offset + 1, raw.slot_count)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_i8(buffer, offset + 5, raw.skill1_pt)?;
                write_u16(buffer, offset + 6, raw.deco1)?;
                write_u16(buffer, offset + 8, raw.deco2)?;
                write_u16(buffer, offset + 10, raw.skill1_id)?;
            },
            EquipSlot::Charm3Slot(raw) => {
                write_u8(buffer, offset, raw.type_id)?;
                write_u8(buffer, offset + 1, raw.slot_count)?;
                write_u16(buffer, offset + 2, raw.id)?;
                write_u16(buffer, offset + 6, raw.deco1)?;
                write_u16(buffer, offset + 8, raw.deco2)?;
                write_u16(buffer, offset + 10, raw.deco3)?;
            },
            EquipSlot::BlankEquipSlot(raw) => {
                let raw_equip_slot: BlankEquipSlot = std::mem::transmute(raw);
                let ptr = buffer.as_mut_ptr().offset(offset as isize) as *mut BlankEquipSlot;
                ptr::write_unaligned(ptr, raw_equip_slot);
            },
        }
    }
    Ok(())
}

pub fn write_slot(buf: &mut [u8; SAVE_SIZE], slot: &CharacterSlot, nth_slot: usize) -> Result<(), String>
{
    let nth_slot_offset = SLOT_OFFSET + nth_slot * SLOT_SIZE;
    write_u8(buf, nth_slot_offset + GENDER_OFFSET, slot.gender)?;
    write_name(buf, nth_slot_offset + NAME_OFFSET, slot.name)?;
    write_u32(buf, nth_slot_offset + ZENNY_OFFSET, slot.zenny)?;
    write_u32(buf, nth_slot_offset + PLAYTIME_OFFSET, slot.playtime)?;

    for k in 0..slot.melee_pouch.len() {
        write_item_slot(buf, nth_slot_offset + MELEE_POUCH_OFFSET + k * std::mem::size_of::<ItemSlot>(),
        slot.melee_pouch[k])?;
    }
    for k in 0..slot.ranged_pouch.len() {
        write_item_slot(buf, nth_slot_offset + RANGE_POUCH_OFFSET + k * std::mem::size_of::<ItemSlot>(),
        slot.ranged_pouch[k])?;
    }
    for k in 0..slot.item_box.len() {
        write_item_slot(buf, nth_slot_offset + ITEM_BOX_OFFSET + k * std::mem::size_of::<ItemSlot>(),
        slot.item_box[k])?;
    }
    for k in 0..slot.equip_box.len() {
        write_equip_slot(buf, nth_slot_offset + EQUIP_BOX_OFFSET + k * std::mem::size_of::<BlankEquipSlot>(),
            slot.equip_box[k])?;
    }
    write_u32(buf, nth_slot_offset + HRP_OFFSET, slot.hrp)?;
    write_u16(buf, nth_slot_offset + HR_OFFSET, slot.hr)?;
    write_u8(buf, nth_slot_offset + FACE_TYPE_OFFSET, slot.face_type)?;
    write_u8(buf, nth_slot_offset + HAIR_TYPE_OFFSET, slot.hair_type)?;
    write_rgb(buf, nth_slot_offset + HAIR_COLOR_OFFSET, slot.hair_color)?;
    write_u8(buf, nth_slot_offset + CLOTH_TYPE_OFFSET, slot.cloth_type)?;
    write_u8(buf, nth_slot_offset + VOICE_TYPE_OFFSET, slot.voice_type)?;
    write_rgb(buf, nth_slot_offset + CLOTH_COLOR_OFFSET, slot.cloth_color)?;
    write_u8(buf, nth_slot_offset + EYE_COLOR_OFFSET, slot.eye_color)?;
    write_u8(buf, nth_slot_offset + FEATURE_TYPE_OFFSET, slot.feature_type)?;
    write_u16(buf, nth_slot_offset + SKIN_TONE_OFFSET, slot.skin_tone)?;
    Ok(())
}

pub fn buf_to_file(filepath: &str, buf: &mut [u8; SAVE_SIZE]) -> Result<(), std::io::Error>
{
    let checksum: u32 = buf
        .iter()
        .skip(8)
        .map(|&x| x as u32)
        .sum();
    println!("Save file checksum: 0x{:08X}", checksum);

    let _ = write_u32(buf, CHECKSUM_OFFSET, checksum);

    let mut file = File::create(filepath)?;
    file.write_all(buf.as_slice())?;
    Ok(())
}