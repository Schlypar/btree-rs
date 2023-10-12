use crate::app::db::fixed_str::Fixed;
use serde::{Deserialize, Serialize};

use super::Random;
use rand::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Person {
    pub name: Fixed,
    pub surname: Fixed,
    pub patronymic: Fixed,
    pub post_index: u32,
}

#[allow(dead_code)]
impl Person {
    pub fn new(name: Fixed, surname: Fixed, patronymic: Fixed, post_index: u32) -> Self {
        Person {
            name,
            surname,
            patronymic,
            post_index,
        }
    }
}

pub fn choose<T>(vec: &Vec<T>) -> &T {
    let i = (0..vec.len()).choose(&mut rand::thread_rng()).unwrap();
    vec.get(i).unwrap()
}

impl Random for Person {
    fn random() -> Self {
        let prenames: Vec<&str> = vec![
            "Smerdo", "Yarostno", "Krasno", "Belo", "Hramo", "Kiber", "Slav", "Vse", "Kruto",
            "Ploho", "Bole", "Malo", "Sredne", "Metro", "Ruka", "Pika", "Nano", "Versto", "Lokot",
            "Malo", "Mnogo", "Uni", "Chasto", "Redko", "Nechasto", "Izvestno", "Slavno", "Chudo",
            "Divano", "Viso", "Dlino", "Nizo", "Greko", "Cherno", "Golubo", "Zhelezo", "Zoloto",
            "Serebro", "Medo", "Zeleno",
        ];
        let names: Vec<&str> = vec![
            "Slav", "Yasher", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav",
            "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav",
            "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav",
            "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav", "Slav",
            "Slav", "Yasher", "Yasher", "Yasher", "Yasher", "Yasher", "Yasher", "Yasher", "Yasher",
            "Yasher",
        ];
        let surnames: Vec<&str> = vec![
            "Pitonov",
            "Zmiyov",
            "Smerdov",
            "Gorbatov",
            "Yasherov",
            "Zeleniyov",
            "Hramov",
            "Koverov",
            "Kiberov",
            "Krutov",
            "Plohov",
            "Malov",
            "Srednov",
            "Metrov",
            "Rukov",
            "Pikov",
            "Nanov",
            "Verstov",
            "Lokotov",
            "Malov",
            "Mnogov",
            "Chastov",
            "Obichnov",
            "Nechastov",
            "Redkov",
            "Izvestov",
            "Slavnov",
            "Divanov",
            "Visov",
            "Dlinov",
            "Nizov",
            "Grekov",
            "Krasnov",
            "Chernov",
            "Belov",
            "Golubov",
            "Zhelezov",
            "Zolotov",
            "Serebrov",
            "Medov",
            "Kozlov",
            "Skodinov",
            "Petrov",
            "Ivanov",
            "Dobrynov",
            "Nikitov",
            "Vladov",
            "Slavynov",
        ];
        let patronimycs: Vec<&str> = vec![
            "Pitonich",
            "Zmiyich",
            "Smerdich",
            "Gorbatich",
            "Yasherich",
            "Zeleniyich",
            "Hramich",
            "Koverich",
            "Kiberich",
            "Krutich",
            "Plohich",
            "Malich",
            "Srednich",
            "Metrich",
            "Rukich",
            "Pikich",
            "Nanich",
            "Verstich",
            "Lokotich",
            "Malich",
            "Mnogich",
            "Univerich",
            "Chastich",
            "Obichnich",
            "Nechastich",
            "Redkich",
            "Izvestich",
            "Slavnich",
            "Divanich",
            "Visich",
            "Dlinich",
            "Nizich",
            "Grekich",
            "Krasnich",
            "Chernich",
            "Belich",
            "Golubich",
            "Zhelezich",
            "Zolotich",
            "Serebrich",
            "Medich",
            "Kozlich",
            "Skodinich",
            "Petrich",
            "Ivanich",
            "Dobrynich",
            "Nikitich",
            "Vladich",
            "Slavynich",
        ];
        let qualities: Vec<&str> = vec![
            "Pacarapano",
            "Vognuto",
            "Vygnuto",
            "Polirovanno",
            "Almazno",
            "Zhelezno",
            "Kozhano",
            "Zoloto",
            "Kolchuzhno",
            "Diryavo",
            "Plotno",
            "Tselo",
            "Veshestvenno",
            "Krasno",
            "Sine",
            "Belo",
            "Cherno",
            "Zhelto",
            "Zapadno",
            "Vostochno",
            "Severno",
            "Yuzhno",
            "Afrikansko",
            "Rusovsko",
            "Yashersko",
            "Polu",
            "Kiber",
            "Giga",
            "Mega",
            "Baikalsko",
        ];
        let double_name = rand::thread_rng().gen::<bool>();
        let double_surname = rand::thread_rng().gen::<bool>();

        Person {
            name: if double_name {
                format!("{}{}", choose(&prenames), choose(&names).to_lowercase()).into()
            } else {
                format!("{}", choose(&names)).into()
            },
            surname: if double_surname {
                format!("{}{}", choose(&qualities), choose(&surnames).to_lowercase()).into()
            } else {
                format!("{}", choose(&surnames)).into()
            },
            patronymic: format!("{}", choose(&patronimycs)).into(),
            post_index: rand::thread_rng().gen_range(1000..3000),
        }
    }
}
