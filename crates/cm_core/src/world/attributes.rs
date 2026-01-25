//! Player attributes.

use serde::{Deserialize, Serialize};

/// Complete player attributes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attributes {
    pub technical: TechnicalAttributes,
    pub mental: MentalAttributes,
    pub physical: PhysicalAttributes,
    pub goalkeeper: GoalkeeperAttributes,
}

impl Attributes {
    /// Calculate overall attacking ability.
    pub fn attack_rating(&self) -> u8 {
        let sum = self.technical.finishing as u16
            + self.technical.dribbling as u16
            + self.technical.passing as u16
            + self.mental.off_the_ball as u16;
        (sum / 4) as u8
    }

    /// Calculate overall defensive ability.
    pub fn defense_rating(&self) -> u8 {
        let sum = self.technical.tackling as u16
            + self.technical.marking as u16
            + self.mental.positioning as u16
            + self.physical.strength as u16;
        (sum / 4) as u8
    }

    /// Calculate overall midfield ability.
    pub fn midfield_rating(&self) -> u8 {
        let sum = self.technical.passing as u16
            + self.technical.first_touch as u16
            + self.mental.vision as u16
            + self.physical.stamina as u16;
        (sum / 4) as u8
    }

    /// Calculate overall keeper ability.
    pub fn keeper_rating(&self) -> u8 {
        let sum = self.goalkeeper.handling as u16
            + self.goalkeeper.reflexes as u16
            + self.goalkeeper.positioning as u16
            + self.goalkeeper.one_on_ones as u16;
        (sum / 4) as u8
    }
}

/// Technical skills.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechnicalAttributes {
    pub crossing: u8,
    pub dribbling: u8,
    pub finishing: u8,
    pub first_touch: u8,
    pub free_kick: u8,
    pub heading: u8,
    pub long_shots: u8,
    pub marking: u8,
    pub passing: u8,
    pub penalties: u8,
    pub tackling: u8,
    pub technique: u8,
}

/// Mental attributes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MentalAttributes {
    pub aggression: u8,
    pub anticipation: u8,
    pub bravery: u8,
    pub composure: u8,
    pub concentration: u8,
    pub decisions: u8,
    pub determination: u8,
    pub flair: u8,
    pub leadership: u8,
    pub off_the_ball: u8,
    pub positioning: u8,
    pub teamwork: u8,
    pub vision: u8,
    pub work_rate: u8,
}

/// Physical attributes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhysicalAttributes {
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub jumping: u8,
    pub natural_fitness: u8,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
}

/// Goalkeeper-specific attributes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoalkeeperAttributes {
    pub aerial_ability: u8,
    pub command_of_area: u8,
    pub communication: u8,
    pub handling: u8,
    pub kicking: u8,
    pub one_on_ones: u8,
    pub positioning: u8,
    pub reflexes: u8,
    pub throwing: u8,
}
