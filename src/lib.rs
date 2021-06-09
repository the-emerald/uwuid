// As far as I can see, the API here won't change. Will fix if this ever gets stabilised!
#![allow(unstable_name_collisions)]

use itertools::Itertools;
use uuid::Uuid;

pub const ZERO: &str = "uwu";
pub const ONE: &str = "owo";
pub const HYPHEN: &str = "rawr xd";

#[derive(Copy, Clone, Eq, Hash, PartialOrd, PartialEq)]
pub struct UwuId {
    uuid: Uuid,
}

impl From<Uuid> for UwuId {
    fn from(val: Uuid) -> Self {
        Self { uuid: val }
    }
}

impl UwuId {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl ToString for UwuId {
    fn to_string(&self) -> String {
        let f1 = (0..32_u32)
            .rev()
            .map(|n| match (self.uuid.as_fields().0 >> n) & 1 {
                0 => ZERO,
                1 => ONE,
                _ => unreachable!(),
            })
            .intersperse(" ")
            .collect::<String>();

        let f2 = (0..16_u16)
            .rev()
            .map(|n| match (self.uuid.as_fields().1 >> n) & 1 {
                0 => ZERO,
                1 => ONE,
                _ => unreachable!(),
            })
            .intersperse(" ")
            .collect::<String>();

        let f3 = (0..16_u16)
            .rev()
            .map(|n| match (self.uuid.as_fields().2 >> n) & 1 {
                0 => ZERO,
                1 => ONE,
                _ => unreachable!(),
            })
            .intersperse(" ")
            .collect::<String>();

        let f45 = self
            .uuid
            .as_fields()
            .3
            .to_vec() // Aw. array map is unstable.
            .into_iter()
            .map(|byte| {
                (0..8_u8)
                    .rev()
                    .map(|n| match (byte >> n) & 1 {
                        0 => ZERO,
                        1 => ONE,
                        _ => unreachable!(),
                    })
                    .intersperse(" ")
                    .collect::<String>()
            })
            .intersperse(" ".to_string())
            .collect::<Vec<String>>();

        let (f4, f5) = f45.split_at(3);

        format!(
            "{} {} {} {} {} {} {} {} {}",
            f1,
            HYPHEN,
            f2,
            HYPHEN,
            f3,
            HYPHEN,
            f4.join(""),
            HYPHEN,
            f5[1..].join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::UwuId;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn nil_uuid() {
        let nil_uuid = UwuId::from(Uuid::nil());
        println!("{}", nil_uuid.to_string())
    }

    #[test]
    fn uuid_v4() {
        let v4_uuid = UwuId::from(Uuid::from_str("4f38bdb9-c368-4630-bb80-8b3eb918a9c8").unwrap());
        println!("{}", v4_uuid.to_string())
    }
}
