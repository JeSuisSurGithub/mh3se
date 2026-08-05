use std::fs::File;
use std::io::Read;

use crate::common::*;

fn read_i8(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<i8, String> {
    if SAVE_SIZE < offset + 1 {
        return Err(format!("Cannot read u8 at {:06X}", offset));
    }
    Ok(buffer[offset] as i8)
}

fn read_u8(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<u8, String> {
    if SAVE_SIZE < offset + 1 {
        return Err(format!("Cannot read u8 at {:06X}", offset));
    }
    Ok(buffer[offset])
}
fn read_u16(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<u16, String> {
    if SAVE_SIZE < offset + 2 {
        return Err(format!("Cannot read u16 at {:06X}", offset));
    }
    Ok(u16::from_be_bytes([buffer[offset], buffer[offset + 1]]))
}
fn read_u32(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<u32, String> {
    if SAVE_SIZE < offset + 4 {
        return Err(format!("Cannot read u32 at {:06X}", offset));
    }
    Ok(u32::from_be_bytes([
        buffer[offset],
        buffer[offset + 1],
        buffer[offset + 2],
        buffer[offset + 3],]))
}
fn read_name(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<[u8; 8], String> {
    if SAVE_SIZE < offset + 8 {
        return Err(format!("Cannot read name at {:06X}", offset));
    }
    let mut array = [0; 8];
    array.copy_from_slice(&buffer[offset..offset + 8]);
    Ok(array)
}
fn read_rgb(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<[u8; 3], String> {
    if SAVE_SIZE < offset + 3 {
        return Err(format!("Cannot read rgb at {:06X}", offset));
    }
    let mut array = [0; 3];
    array.copy_from_slice(&buffer[offset..offset + 3]);
    Ok(array)
}

impl ItemSlot {
    pub fn from_buf(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<Self, String> {
        let id = read_u16(buffer, offset)?;
        let qty = read_u16(buffer, offset + 2)?;
        Ok(ItemSlot{id, qty})
    }
}

impl EquipSlot {
    pub fn from_buf(buffer: &[u8; SAVE_SIZE], offset: usize) -> Result<Self, String> {
        let type_id = read_u8(buffer, offset)?;
        match TypeID::from_u8(type_id) {
            TypeID::Head | TypeID::Chest | TypeID::Arms | TypeID::Waist | TypeID::Legs => {
                Ok(EquipSlot::Armor(Armor{
                    type_id: type_id,
                    lvl: read_u8(buffer, offset + 1)?,
                    id: read_u16(buffer, offset + 2)?,
                    deco1: read_u16(buffer, offset + 6)?,
                    deco2: read_u16(buffer, offset + 8)?,
                    deco3: read_u16(buffer, offset + 10)?,
                }))
            },
            TypeID::GS | TypeID::SNS | TypeID::HA | TypeID::LA | TypeID::LS | TypeID::SA => {
                Ok(EquipSlot::MeleeWeapon(MeleeWeapon{
                    type_id: type_id,
                    id: read_u16(buffer, offset + 2)?,
                    deco1: read_u16(buffer, offset + 6)?,
                    deco2: read_u16(buffer, offset + 8)?,
                    deco3: read_u16(buffer, offset + 10)?,
                }))
            },
            TypeID::Frame | TypeID::Barrel | TypeID::Stock  => {
                Ok(EquipSlot::RangedWeapon(RangedWeapon{
                    type_id: type_id,
                    lvl: read_u8(buffer, offset + 1)?,
                    id: read_u16(buffer, offset + 2)?,
                    deco1: read_u16(buffer, offset + 6)?,
                    deco2: read_u16(buffer, offset + 8)?,
                    deco3: read_u16(buffer, offset + 10)?,
                }))
            },
            TypeID::Talisman => {
                let slot_count = read_u8(buffer, offset + 1)?;
                match slot_count {
                    0 => {
                        Ok(EquipSlot::Charm0Slot(Charm0Slot{
                            type_id: type_id,
                            slot_count: read_u8(buffer, offset + 1)?,
                            id: read_u16(buffer, offset + 2)?,
                            skill1_pt: read_i8(buffer, offset + 5)?,
                            skill2_pt: read_i8(buffer, offset + 4)?,
                            skill1_id: read_u16(buffer, offset + 6)?,
                            skill2_id: read_u16(buffer, offset + 8)?,
                        }))
                    },
                    1 => {
                        Ok(EquipSlot::Charm1Slot(Charm1Slot{
                            type_id: type_id,
                            slot_count: read_u8(buffer, offset + 1)?,
                            id: read_u16(buffer, offset + 2)?,
                            skill1_pt: read_i8(buffer, offset + 5)?,
                            skill2_pt: read_i8(buffer, offset + 4)?,
                            deco1: read_u16(buffer, offset + 6)?,
                            skill1_id: read_u16(buffer, offset + 8)?,
                            skill2_id: read_u16(buffer, offset + 10)?,
                        }))
                    },
                    2 => {
                        Ok(EquipSlot::Charm2Slot(Charm2Slot{
                            type_id: type_id,
                            slot_count: read_u8(buffer, offset + 1)?,
                            id: read_u16(buffer, offset + 2)?,
                            skill1_pt: read_i8(buffer, offset + 5)?,
                            deco1: read_u16(buffer, offset + 6)?,
                            deco2: read_u16(buffer, offset + 8)?,
                            skill1_id: read_u16(buffer, offset + 10)?,
                        }))
                    },
                    3 => {
                        Ok(EquipSlot::Charm3Slot(Charm3Slot{
                            type_id: type_id,
                            slot_count: read_u8(buffer, offset + 1)?,
                            id: read_u16(buffer, offset + 2)?,
                            deco1: read_u16(buffer, offset + 6)?,
                            deco2: read_u16(buffer, offset + 8)?,
                            deco3: read_u16(buffer, offset + 10)?,
                        }))
                    }
                    _ => {
                        Err(format!("Unrecognized talisman slot at 0x{:06X}", offset))
                    }
                }
            }
            TypeID::None => {
                Ok(EquipSlot::BlankEquipSlot(BlankEquipSlot::default()))
            }
        }
    }
}

pub fn file_to_buf(filepath: &str) -> Result<[u8; SAVE_SIZE], std::io::Error>
{
    let mut buffer = [0; SAVE_SIZE];
    let mut file = File::open(filepath)?;
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

impl CharacterSlot {
    pub fn from_buf(buf: &[u8; SAVE_SIZE], nth_slot: usize) -> Result<Self, String>
    {
        let nth_slot_offset = SLOT_OFFSET + nth_slot * SLOT_SIZE;
        let mut slot = CharacterSlot::default();
        slot.gender     = read_u8(buf, nth_slot_offset + GENDER_OFFSET)?;
        slot.name       = read_name(buf, nth_slot_offset + NAME_OFFSET)?;
        slot.zenny      = read_u32(buf, nth_slot_offset + ZENNY_OFFSET)?;
        slot.playtime   = read_u32(buf, nth_slot_offset + PLAYTIME_OFFSET)?;
        for k in 0..slot.melee_pouch.len() {
            slot.melee_pouch[k] = ItemSlot::from_buf(buf,
                nth_slot_offset + MELEE_POUCH_OFFSET + k * std::mem::size_of::<ItemSlot>())?;
        }
        for k in 0..slot.ranged_pouch.len() {
            slot.ranged_pouch[k] = ItemSlot::from_buf(buf,
                nth_slot_offset + RANGE_POUCH_OFFSET + k * std::mem::size_of::<ItemSlot>())?;
        }
        for k in 0..slot.item_box.len() {
            slot.item_box[k] = ItemSlot::from_buf(buf,
                nth_slot_offset + ITEM_BOX_OFFSET + k * std::mem::size_of::<ItemSlot>())?;
        }
        for k in 0..slot.equip_box.len() {
            slot.equip_box[k] = EquipSlot::from_buf(buf, nth_slot_offset + EQUIP_BOX_OFFSET + k * std::mem::size_of::<BlankEquipSlot>())?;
        }
        slot.hrp = read_u32(buf, nth_slot_offset + HRP_OFFSET)?;
        slot.hr  = read_u16(buf, nth_slot_offset + HR_OFFSET)?;

        slot.face_type      = read_u8(buf, nth_slot_offset + FACE_TYPE_OFFSET)?;
        slot.hair_type      = read_u8(buf, nth_slot_offset + HAIR_TYPE_OFFSET)?;
        slot.hair_color     = read_rgb(buf, nth_slot_offset + HAIR_COLOR_OFFSET)?;
        slot.cloth_type     = read_u8(buf, nth_slot_offset + CLOTH_TYPE_OFFSET)?;
        slot.voice_type     = read_u8(buf, nth_slot_offset + VOICE_TYPE_OFFSET)?;
        slot.cloth_color    = read_rgb(buf, nth_slot_offset + CLOTH_COLOR_OFFSET)?;
        slot.eye_color      = read_u8(buf, nth_slot_offset + EYE_COLOR_OFFSET)?;
        slot.feature_type   = read_u8(buf, nth_slot_offset + FEATURE_TYPE_OFFSET)?;
        slot.skin_tone      = read_u16(buf, nth_slot_offset + SKIN_TONE_OFFSET)?;
        Ok(slot)
    }
}