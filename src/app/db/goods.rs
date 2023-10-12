use crate::app::db::fixed_str::Fixed;
use serde::{Deserialize, Serialize};

pub use super::person::{choose, Person};
use super::Random;
use rand::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Crate {
    pub sender: Person,
    pub receiver: Person,
    pub goods_name: Fixed,
    pub producer: Fixed,
    pub goods_id: u64,
}

#[allow(dead_code)]
impl Crate {
    pub fn new(
        sender: Person,
        receiver: Person,
        goods_name: Fixed,
        producer: Fixed,
        goods_id: u64,
    ) -> Self {
        Crate {
            goods_name,
            producer,
            sender,
            receiver,
            goods_id,
        }
    }
}

impl Random for Crate {
    fn random() -> Self {
        let qualities: Vec<&str> = vec![
            "Peruna ",
            "Yarilova ",
            "Yarila ",
            "Svaroga ",
            "Dazhboga ",
            "Striboga ",
            "Chernoboga ",
            "Velesa ",
            "Gamayun ",
            "Svyativita ",
            "Triglava ",
            "Horosho-",
            "Pacarapano-",
            "Vognuto-",
            "Vygnuto-",
            "Polirovanno-",
            "Almazno-",
            "Zhelezno-",
            "Kozhano-",
            "Zoloto-",
            "Kolchuzhno-",
            "Diryavo-",
            "Plotno-",
            "Tselo-",
            "Drobno-",
            "Veshestvenno-",
            "Slozhno-",
            "Prosto-",
            "Red ",
            "Blue ",
            "White ",
            "Black ",
            "Brown ",
            "Yellow ",
            "iz Zapada ",
            "iz Vostoka ",
            "iz Severa ",
            "iz Yuga ",
            "iz Afriki ",
            "ot Rusov ",
            "ot Yasherov ",
            "Poly",
            "Kiber",
            "Giga",
            "Mega",
            "iz Baikala ",
        ];
        let goods: Vec<&str> = vec![
            "Mech", "Shit", "Kopyie", "Topor", "Kolchuga", "Shlem", "Kamen", "Palka", "Kniga",
            "Gramota", "Korona", "Zhena", "Stol", "Lodka", "Voda", "Eda", "Zhelezo", "Zoloto",
            "Almaz", "Serebro", "Med", "Kalash", "Medved", "Bumaga", "Kost", "Shar", "Slon",
            "Yasher", "Laba",
        ];
        let producers: Vec<&str> = vec![
            "Gora",
            "More",
            "Cheburek",
            "Loshad",
            "Medved",
            "Cifra",
            "Shar",
            "Kost",
            "Slon",
            "Shit i mech ",
            "Rus",
            "Drevo",
            "Zherebets Yasher ",
            "Polychel",
            "Gigaslav",
            "Baikal",
            "Varyag",
        ];
        let places: Vec<&str> = vec![
            "prodzavod",
            "zavod",
            "remeslo",
            "fabrika",
            "delo",
            "nachalo",
            "otrok",
            "imenie",
        ];

        let quality_good = rand::thread_rng().gen::<bool>();

        Crate {
            sender: Person::random(),
            receiver: Person::random(),
            goods_name: if quality_good {
                format!("{}{}", choose(&qualities), choose(&goods).to_lowercase()).into()
            } else {
                format!("{}", choose(&goods)).into()
            },
            producer: format!("{}{}", choose(&producers), choose(&places).to_lowercase()).into(),
            goods_id: rand::thread_rng().gen_range(0..5000),
        }
    }
}
