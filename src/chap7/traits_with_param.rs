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

fn obtenir_animal(b: bool) -> impl Animal {
    let nom: String = "Mimi l'animal".to_string();
    animal::mod_animal::Chien::creer(nom)
}

// Ne compile pas.
// fn obtenir_animal2(b: bool) -> impl Animal {
//     let nom: String = "Mimi l'animal".to_string();
//     if b == true {
//         animal::mod_animal::Chien::creer(nom)
//     } else {
//         animal::mod_animal::Chat::creer(nom)
//     }
// }

// Course d'un jour.
trait CourseEtape {}

// Course par étapes.
struct StructCourseEtapes<J: CourseEtape> {
    etapes: Vec<J>,
}
// Course par étapes 2. Ne compile pas, car au moment de la compilation, il ne connaît pas la taille de chaque élément du vecteur...
// struct CourseEtapes2 {
//     etapes: Vec<CourseEtape>,
// }

// Course par étapes 3.
struct StructCourseEtapes3<J: CourseEtape> {
    etapes: Vec<Box<J>>,
}

/// Heritage :
/// ```mermaid
/// classDiagram
///     Animal <|-- Poisson : Hérite
///     Goujon --> Poisson
///
///
///     class Animal
///     <<interface>> Animal
///     Animal : creer(nom String) -> Self
///     Animal : emettre_son(&self) -> String
///     Animal : obtenir_nom(&self) -> String
///
///     class Poisson
///     <<interface>> Poisson
///     Poisson : nager(&self)
///
///     Class Goujon
/// ```
pub trait Poisson: Animal {
    fn nager(&self) -> String {
        // self.afficher();
        "Nage dans les eaux".to_string()
    }
}
pub struct Goujon {
    nom: String,
}

impl Animal for Goujon {
    fn creer(nom: String) -> Self {
        Goujon { nom: nom }
    }

    fn afficher(&self) {
        println!(
            "{} le goujon cri : '{}' quand {}",
            self.obtenir_nom(),
            self.emettre_son(),
            self.nager()
        );
    }
    fn emettre_son(&self) -> String {
        "Blublublu".to_string()
    }
    fn obtenir_nom(&self) -> String {
        let copie = self.nom.clone();
        copie
    }
}

impl Poisson for Goujon {
    fn nager(&self) -> String {
        "Nage dans les eaux profonde".to_string()
    }
}
