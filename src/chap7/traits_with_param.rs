#![allow(dead_code)]

use std::fmt;

extern crate animal;
use crate::animal::mod_animal::Animal;

pub struct Tortue {
    nom: String,
}

impl Animal for Tortue {
    fn creer(nom: String) -> Tortue {
        Tortue { nom: nom }
    }

    fn emettre_son(&self) -> String {
        "stridulation".to_string()
    }

    fn obtenir_nom(&self) -> String {
        let copie = self.nom.clone();
        copie
    }
}

impl fmt::Display for Tortue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ::: {})", self.obtenir_nom(), self.emettre_son())
    }
}

pub fn affichage_format(element: &(impl fmt::Display + Animal)) {
    println!("Affichage formaté : {}", format!("{}", element));
}

fn fonction_exemple<T, U>(t: &T, u: &U) -> String
where
    T: Animal + fmt::Display,
    U: Animal + fmt::Debug,
{
    String::new()
}