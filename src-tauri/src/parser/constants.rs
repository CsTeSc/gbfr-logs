use std::fmt::{Display, Formatter};

use enum_display::EnumDisplay;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, EnumDisplay)]
pub enum CharacterType {
    Pl0000,
    Pl0100,
    Pl0200,
    Pl0300,
    Pl0400,
    Pl0500,
    Pl0600,
    Pl0700,
    Pl0800,
    Pl0900,
    Pl1000,
    Pl1100,
    Pl1200,
    Pl1300,
    Pl1400,
    Pl1500,
    Pl1600,
    Pl1700,
    Pl1800,
    Pl1900,
    Pl2000,
    Unknown(u32),
}

impl CharacterType {
    pub fn from_hash(hash: u32) -> Self {
        match hash {
            0x26A4848A => CharacterType::Pl0000,
            0x9498420D => CharacterType::Pl0100,
            0x34D4FD8F => CharacterType::Pl0200,
            0xF8D73D33 => CharacterType::Pl0300,
            0x7B5934AD => CharacterType::Pl0400,
            0x443D46BB => CharacterType::Pl0500,
            0xA9D6569E => CharacterType::Pl0600,
            0xFBA6615D => CharacterType::Pl0700,
            0x63A7C3F0 => CharacterType::Pl0800,
            0xF96A90C2 => CharacterType::Pl0900,
            0x28AC1108 => CharacterType::Pl1000,
            0x94E2514E => CharacterType::Pl1100,
            0x2B4AA114 => CharacterType::Pl1200,
            0xC97F3365 => CharacterType::Pl1300,
            0x601AA977 => CharacterType::Pl1400,
            0xBCC238DE => CharacterType::Pl1500,
            0xC3155079 => CharacterType::Pl1600,
            0xD16CFBDE => CharacterType::Pl1700,
            0x6FDD6932 => CharacterType::Pl1800,
            0x8056ABCD => CharacterType::Pl1900,
            0xF5755C0E => CharacterType::Pl2000,
            _ => CharacterType::Unknown(hash),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub enum EnemyType {
    Em0000,
    Em0001,
    Em0002,
    Em0003,
    Em0004,
    Em0005,
    Em0100,
    Em0101,
    Em0102,
    Em0103,
    Em0200,
    Em0201,
    Em0300,
    Em0400,
    Em0500,
    Em0501,
    Em0502,
    Em0600,
    Em0601,
    Em0602,
    Em0603,
    Em0604,
    Em0605,
    Em0701,
    Em0706,
    Em0800,
    Em0801,
    Em0802,
    Em0803,
    Em0804,
    Em0805,
    Em0900,
    Em0901,
    Em0902,
    Em0903,
    Em1000,
    Em1001,
    Em1002,
    Em1003,
    Em1004,
    Em1005,
    Em1006,
    Em1100,
    Em1200,
    Em1300,
    Em1301,
    Em1303,
    Em1500,
    Em1600,
    Em1800,
    Em1801,
    Em1802,
    Em1803,
    Em1804,
    Em1805,
    Em1806,
    Em1900,
    Em2000,
    Em2100,
    Em2400,
    Em2500,
    Em2600,
    Em2700,
    Em2800,
    Em3100,
    Em7000,
    Em7100,
    Em7110,
    Em7200,
    Em7210,
    Em7300,
    Em7310,
    Em7400,
    Em7401,
    Em7404,
    Em7406,
    Em7407,
    Em7408,
    Em7409,
    Em7500,
    Em7520,
    Em7530,
    Em7600,
    Em7603,
    Em7610,
    Unknown(u32),
}

impl EnemyType {
    pub fn from_hash(hash: u32) -> Self {
        match hash {
            0x427E32FE => Self::Em0000,
            0x44A72D4B => Self::Em0001,
            0x3D05EB17 => Self::Em0002,
            0x00012BCD => Self::Em0003,
            0xCA4091C8 => Self::Em0004,
            0x84FDD7B5 => Self::Em0005,
            0x0E0A8D33 => Self::Em0100,
            0xA81F6DCA => Self::Em0101,
            0x3A0EA858 => Self::Em0102,
            0xB69DD953 => Self::Em0103,
            0x777ABF6D => Self::Em0200,
            0x4698F258 => Self::Em0201,
            0xA63DE428 => Self::Em0300,
            0x119D4D01 => Self::Em0400,
            0xB71014C5 => Self::Em0500,
            0x5FB90EC9 => Self::Em0501,
            0xBA320EEB => Self::Em0502,
            0x6782B537 => Self::Em0600,
            0xF83B508A => Self::Em0601,
            0x80B7EEFF => Self::Em0602,
            0x0178BC98 => Self::Em0603,
            0x2D1E3CC6 => Self::Em0604,
            0xB624C0BE => Self::Em0605,
            0x6D5BA989 => Self::Em0701,
            0xB31EE30A => Self::Em0706,
            0xD622ED34 => Self::Em0800,
            0xB2EFC09D => Self::Em0801,
            0xE777F247 => Self::Em0802,
            0xC4D2BB4B => Self::Em0803,
            0x8EAF20FB => Self::Em0804,
            0xE7767BA1 => Self::Em0805,
            0x095C118F => Self::Em0900,
            0x97DFA1F2 => Self::Em0901,
            0x0EE3E67F => Self::Em0902,
            0xE93FACA2 => Self::Em0903,
            0x48EBACE1 => Self::Em1000,
            0x3165AABA => Self::Em1001,
            0x9AC78D6F => Self::Em1002,
            0x363F9B5C => Self::Em1003,
            0xFD6D4BAB => Self::Em1004,
            0x3A1C16F4 => Self::Em1005,
            0x03EC1106 => Self::Em1006,
            0xBCEF1EE2 => Self::Em1100,
            0x9338ABC9 => Self::Em1200,
            0x58A85756 => Self::Em1300,
            0xF45A0B6D => Self::Em1301,
            0xBDC484A5 => Self::Em1303,
            0x3F7F9A35 => Self::Em1500,
            0x7D71C604 => Self::Em1600,
            0x044BBC73 => Self::Em1800,
            0x4A299E62 => Self::Em1801,
            0xE170F036 => Self::Em1802,
            0xAFEB2D7E => Self::Em1803,
            0x181B3A5B => Self::Em1804,
            0x0E3D1703 => Self::Em1805,
            0x65C26889 => Self::Em1806,
            0x6409B2A8 => Self::Em1900,
            0x17576F75 => Self::Em2000,
            0xE4C12A94 => Self::Em2100,
            0xB39EEAB5 => Self::Em2400,
            0xC3089918 => Self::Em2500,
            0x6056B80A => Self::Em2600,
            0xA379AC65 => Self::Em2700,
            0x8B964BE7 => Self::Em2800,
            0x4FDB94BB => Self::Em3100,
            0xDBCA3857 => Self::Em7000,
            0x8C3ADB57 => Self::Em7100,
            0x96316442 => Self::Em7110,
            0xC3C58C76 => Self::Em7200,
            0xE556D1BF => Self::Em7210,
            0x75A3867C => Self::Em7300,
            0x938A4BDF => Self::Em7310,
            0xB6E8888A => Self::Em7400,
            0x0DBFC18A => Self::Em7401,
            0x4D2202AE => Self::Em7404,
            0x22E188A1 => Self::Em7406,
            0xA51BA796 => Self::Em7407,
            0xF946D055 => Self::Em7408,
            0x677F73CD => Self::Em7409,
            0xE7576708 => Self::Em7500,
            0x6F92396E => Self::Em7520,
            0x1943ED28 => Self::Em7530,
            0xB4AED02D => Self::Em7600,
            0x5E316D49 => Self::Em7603,
            0xA86498C5 => Self::Em7610,
            _ => EnemyType::Unknown(hash),
        }
    }
}

impl Display for EnemyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(hash) => return write!(f, "Unknown({:x})", hash),
            _ => write!(f, "{}", self.to_string()),
        }
    }
}
