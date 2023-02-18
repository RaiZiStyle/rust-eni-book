#![allow(dead_code)]

pub struct Cercle {
    pub coord_centre_x: i64,
    pub coord_centre_y: i64,
    pub rayon_cercle: i64,
    pub nom_cercle: String,
}

pub struct Point(pub i64, pub i64, pub String);

pub struct Unite;

pub struct Rectangle {
    largeur: f64,
    longueur: f64,
}

impl Rectangle {
    pub fn new(longueur: f64, largeur: f64) -> Rectangle {
        Rectangle { largeur, longueur }
    }

    pub fn perimetre(&self) -> f64 {
        2.0 * (self.largeur + self.longueur)
    }

    pub fn aire(&self) -> f64 {
        self.largeur * self.longueur
    }

    pub fn obtenir_largeur_int(&self) -> i64 {
        self.largeur.floor() as i64
    }

    pub fn obtenir_longueur_int(&self) -> i64 {
        self.longueur.ceil() as i64
    }
}
