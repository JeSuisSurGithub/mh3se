use serde::{Serialize, Deserialize};
use serde_with::serde_as;

pub const CHECKSUM_OFFSET: usize        = 0x0004;
pub const SLOT_TOGGLE_START: usize      = 0x001A;

pub const SLOT_OFFSET: usize            = 0x0048;
pub const SLOT_SIZE: usize              = 0x6000;
pub const SAVE_SIZE: usize              = 0x14000;

pub const GENDER_OFFSET: usize          = 0x0000;
pub const FACE_TYPE_OFFSET: usize       = 0x0001;
pub const HAIR_TYPE_OFFSET: usize       = 0x0002;
pub const NAME_OFFSET: usize            = 0x0003;
pub const HAIR_COLOR_OFFSET: usize      = 0x0014;
pub const ZENNY_OFFSET: usize           = 0x0018;
pub const PLAYTIME_OFFSET: usize        = 0x001C;
pub const MELEE_POUCH_OFFSET: usize     = 0x00A0;
pub const CLOTH_TYPE_OFFSET: usize      = 0x009E;
pub const VOICE_TYPE_OFFSET: usize      = 0x009F;
pub const RANGE_POUCH_OFFSET: usize     = 0x0100;
pub const ITEM_BOX_OFFSET: usize        = 0x0180;
pub const EQUIP_BOX_OFFSET: usize       = 0x0E00;
pub const CLOTH_COLOR_OFFSET: usize     = 0x39BC;
pub const HRP_OFFSET: usize             = 0x3DE0;
pub const HR_OFFSET: usize              = 0x3DE4;
pub const EYE_COLOR_OFFSET: usize       = 0x3DE6;
pub const FEATURE_TYPE_OFFSET: usize    = 0x3DE7;
pub const SKIN_TONE_OFFSET: usize       = 0x39A0;

pub enum TypeID {
    None        = 0x0,
    Chest       = 0x1,
    Arms        = 0x2,
    Waist       = 0x3,
    Legs        = 0x4,
    Head        = 0x5,
    Talisman    = 0x6,
    GS          = 0x7,
    SNS         = 0x8,
    HA          = 0x9,
    LA          = 0xA,
    Frame       = 0xB,
    Barrel      = 0xC,
    Stock       = 0xD,
    LS          = 0xE,
    SA          = 0xF,
}

impl TypeID {
    pub fn from_u8(val: u8) -> TypeID {
        match val {
            0x1 => TypeID::Chest,
            0x2 => TypeID::Arms,
            0x3 => TypeID::Waist,
            0x4 => TypeID::Legs,
            0x5 => TypeID::Head,
            0x6 => TypeID::Talisman,
            0x7 => TypeID::GS,
            0x8 => TypeID::SNS,
            0x9 => TypeID::HA,
            0xA => TypeID::LA,
            0xB => TypeID::Frame,
            0xC => TypeID::Barrel,
            0xD => TypeID::Stock,
            0xE => TypeID::LS,
            0xF => TypeID::SA,
            _ => TypeID::None
        }
    }
}

/* ---------------------------------------- Save Data ---------------------------------------- */
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct ItemSlot {
    pub id: u16,
    pub qty: u16,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct BlankEquipSlot {
    pub buf: [u8; 12],
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct MeleeWeapon {
    pub type_id: u8,
    pub id: u16,
    pub deco1: u16,
    pub deco2: u16,
    pub deco3: u16,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RangedWeapon {
    pub type_id: u8,
    pub lvl: u8,
    pub id: u16,
    pub deco1: u16,
    pub deco2: u16,
    pub deco3: u16,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Armor {
    pub type_id: u8,
    pub lvl: u8,
    pub id: u16,
    pub deco1: u16,
    pub deco2: u16,
    pub deco3: u16,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Charm0Slot {
    pub type_id: u8,
    pub slot_count: u8,
    pub id: u16,
    pub skill1_pt: i8,
    pub skill2_pt: i8,
    pub skill1_id: u16,
    pub skill2_id: u16,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Charm1Slot {
    pub type_id: u8,
    pub slot_count: u8,
    pub id: u16,
    pub skill1_pt: i8,
    pub skill2_pt: i8,
    pub deco1: u16,
    pub skill1_id: u16,
    pub skill2_id: u16,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Charm2Slot {
    pub type_id: u8,
    pub slot_count: u8,
    pub id: u16,
    pub skill1_pt: i8,
    pub deco1: u16,
    pub deco2: u16,
    pub skill1_id: u16,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Charm3Slot {
    pub type_id: u8,
    pub slot_count: u8,
    pub id: u16,
    pub deco1: u16,
    pub deco2: u16,
    pub deco3: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EquipSlot {
    BlankEquipSlot(BlankEquipSlot),
    MeleeWeapon(MeleeWeapon),
    RangedWeapon(RangedWeapon),
    Armor(Armor),
    Charm0Slot(Charm0Slot),
    Charm1Slot(Charm1Slot),
    Charm2Slot(Charm2Slot),
    Charm3Slot(Charm3Slot),
}

/* ---------------------------------------- Character Slot ---------------------------------------- */
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct CharacterSlot {
    pub gender:         u8,
    pub name:           [u8; 8],
    pub zenny:          u32,
    pub playtime:       u32,
    #[serde_as(as = "[_; 24]")]
    pub melee_pouch:    [ItemSlot; 24],
    #[serde_as(as = "[_; 32]")]
    pub ranged_pouch:   [ItemSlot; 32],
    #[serde_as(as = "[_; 800]")]
    pub item_box:       [ItemSlot; 800],
    #[serde_as(as = "[_; 800]")]
    pub equip_box:      [EquipSlot; 800],
    pub hrp:            u32,
    pub hr:             u16,
    pub face_type:      u8,
    pub hair_type:      u8,
    pub hair_color:     [u8; 3],
    pub cloth_type:     u8,
    pub voice_type:     u8,
    pub cloth_color:    [u8; 3],
    pub eye_color:      u8,
    pub feature_type:   u8,
    pub skin_tone:      u16
}

impl CharacterSlot
{
    pub fn default() -> Self {
        CharacterSlot {
            gender: 0,
            name: [0; 8],
            zenny: 0,
            playtime: 0,
            melee_pouch: [ItemSlot::default(); 24],
            ranged_pouch: [ItemSlot::default(); 32],
            item_box: [ItemSlot::default(); 800],
            equip_box: [EquipSlot::BlankEquipSlot(BlankEquipSlot::default()); 800],
            hrp: 0,
            hr: 0,
            face_type: 0,
            hair_type: 0,
            hair_color: [0; 3],
            cloth_type: 0,
            voice_type: 0,
            cloth_color: [0; 3],
            eye_color: 0,
            feature_type: 0,
            skin_tone: 0,
        }
    }
}