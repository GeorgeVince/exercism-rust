// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mana: Option<u32> = if self.level >= 10 { Some(100) } else { None };
            Some(Player {
                health: 100,
                mana: mana,
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(x) => {
                if x < mana_cost {
                    return 0;
                } else {
                    self.mana = Some(x - mana_cost);
                    return mana_cost * 2
                }
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                return 0;
            }
        };
    }
}
