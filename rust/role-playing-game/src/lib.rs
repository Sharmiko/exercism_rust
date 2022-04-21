// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;


pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level
            }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level < 10 {
            self.health -= min(self.health, mana_cost);
            return 0;
        } else if self.mana.unwrap() < mana_cost {
            return 0;
        } else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            return mana_cost * 2;
        }
    }
}
