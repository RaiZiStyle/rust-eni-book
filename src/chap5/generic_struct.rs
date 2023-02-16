#![allow(dead_code)]

/// Exemple de structure qui contient un i64
/// En effet, afin d’encapsuler un vecteur de flottants dans une structure, ou un vecteur de chaînes de caractères,
/// il va falloir dupliquer le code déjà écrit en l’adaptant légèrement.
pub struct VecteurEntiers {
    vec: Vec<i64>,
}

impl VecteurEntiers {
    pub fn creer() -> VecteurEntiers {
        VecteurEntiers { vec: Vec::new() }
    }

    pub fn ajouter(&mut self, element: i64) {
        self.vec.push(element);
    }

    pub fn obtenir_longueur(&mut self) -> usize {
        self.vec.len()
    }

    pub fn obtenir_element(&mut self, indice: usize) -> i64 {
        if indice < self.vec.len() {
            self.vec[indice]
        } else {
            -1
        }
    }
}

/// Structure générique qui peut contenir
/// des floats, des int, string etc..
pub struct VecteurGenerique<T> {
    vec: Vec<T>,
}
impl<T> VecteurGenerique<T> {
    pub fn creer() -> VecteurGenerique<T> {
        VecteurGenerique { vec: Vec::new() }
    }

    pub fn ajouter(&mut self, element: T) {
        self.vec.push(element);
    }

    pub fn obtenir_longeur(&mut self) -> usize {
        self.vec.len()
    }
}
