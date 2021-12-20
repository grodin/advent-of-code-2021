use std::str::FromStr;

use color_eyre::eyre::{eyre, Context, Report};

#[derive(Debug)]
pub struct LanternFish {
    pub age: usize,
}

impl LanternFish {
    const BIRTH_AGE: usize = 8;
}

impl FromStr for LanternFish {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let age = s
            .parse::<usize>()
            .wrap_err_with(|| format!("Error parsing {} as u8", s))?;
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
