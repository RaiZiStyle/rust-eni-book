#![allow(dead_code)]

#[derive(Debug)]
/// Exemple de enum content des structs
///
/// Parametres :
/// * A : structure Unité
/// * B : Tuple d'option de string
/// * C : structure champs nommée
///
/// Note : Un mot enfin à propos de la visibilité :
/// si l’énumération est de visibilité publique, alors tous les champs qui la composent seront eux aussi publics.
pub enum Exemple {
    A,
    B(Option<String>),
    C {
        champ_nomme_1: String,
        champ_nomme_2: i64,
    },
}

/// Exemple d'enumeration générique
enum ExempleGeneric<T> {
    None,
    Some(T),
}

pub enum ArbreBinaire<T> {
    Terminaison,
    NonTerminaison(Box<Noeud<T>>),
}

pub struct Noeud<T> {
    pub valeur: T,
    pub gauche: ArbreBinaire<T>,
    pub droite: ArbreBinaire<T>,
}

/// Exemple plus concret d'enum content des structs
///
/// Parametres :
/// * Heros : structure Unité du Heros (en god mode)
/// * Fantome : Structure champs nommée
/// * Combattant : structure champs nommée
/// * Magicien : Structure de tuple
pub enum Personnage {
    Heros,
    Fantome {
        points_de_vie: u32,
        indice_invisibilite: u32,
    },
    Combattant {
        points_de_vie: u32,
    },
    Magicien(u32, u32),
}

pub fn filtrage_par_motif(entree: Personnage) -> String {
    match entree {
        Personnage::Heros => format!("Le héros va très bien."),
        Personnage::Fantome {
            points_de_vie,
            indice_invisibilite,
        } => format!(
            "Point de vie {} et indice d'invisibilite {} pour le fantôme.",
            points_de_vie, indice_invisibilite
        ),
        Personnage::Combattant { points_de_vie } => {
            format!("Point de vie {} pour le combattant.", points_de_vie)
        }
        Personnage::Magicien(a, b) => {
            format!("Indice de magie {} et âge {} pour le magicien.", a, b)
        }
    }
}
