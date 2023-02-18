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
