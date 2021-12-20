use std::str::FromStr;

use color_eyre::eyre::{eyre, Report};

#[derive(Debug)]
pub struct LanternFish {
    age: u8,
}

impl LanternFish {
    const BIRTH_AGE: u8 = 8;
    const REBIRTH_AGE: u8 = 6;
    fn age(&self) -> u8 {
        self.age
    }

    pub fn step_day(&mut self) -> Option<LanternFish> {
        let (new_age, new_fish) = match self.age {
            0 => (
                Self::REBIRTH_AGE,
                Some(LanternFish {
                    age: Self::BIRTH_AGE,
                }),
            ),
            1..=Self::BIRTH_AGE => (self.age() - 1, None),
            _ => panic!("Lantern fish with age out of bounds: {}", self.age),
        };
        self.age = new_age;
        new_fish
    }
}

impl FromStr for LanternFish {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let age = s.parse::<u8>()?;
        (age <= Self::BIRTH_AGE)
            .then(|| LanternFish { age })
            .ok_or_else(|| {
                eyre!(
                    "Can't create lantern fish with age {}, greater than {}",
                    age,
                    Self::BIRTH_AGE
                )
            })
    }
}
