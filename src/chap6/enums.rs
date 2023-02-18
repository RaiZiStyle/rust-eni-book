#![allow(dead_code)]

#[derive(Debug)]
pub enum Departement {
    Ain = 1,
    Aisne,
    Allier,
    AlpesDeHauteProvence,
    HautesAlpes,
    AlpesMaritimes,
    Ardeche,
    Ardennes,
    Ariege,
}

impl Departement {
    fn conversion_i64(self) -> i64 {
        let ret = self as i64;
        ret
    }

    pub fn conversion_str(self) -> &'static str {
        match self {
            Departement::Ain => "Ain",
            Departement::Aisne => "Aisne",
            Departement::Allier => "Allier",
            Departement::AlpesDeHauteProvence => "Alpes-de-Haute-Provence",
            Departement::HautesAlpes => "Hautes-Alpes",
            Departement::AlpesMaritimes => "Alpes-Maritimes",
            Departement::Ardeche => "Ardèche",
            Departement::Ardennes => "Ardennes",
            Departement::Ariege => "Ariège",
        }
    }
}

pub fn reste_division(numerateur: i64, denominateur: i64) -> Option<i64> {
    if denominateur == 0 {
        None
    } else {
        Some(numerateur % denominateur)
    }
}

pub fn obtenir_valeur_numerique(valeur: Option<i64>) -> i64 {
    let mut ret: i64 = -1;
    match valeur {
        Some(p) => ret = p,
        None => println!(
            "obtenir_valeur_numerique : 
pas de valeur (division par zéro)"
        ),
    }
    ret
}

pub fn i64_vers_enum(valeur: i64) -> Option<Departement> {
    match valeur {
        1 => Some(Departement::Ain),
        2 => Some(Departement::Aisne),
        3 => Some(Departement::Allier),
        4 => Some(Departement::AlpesDeHauteProvence),
        5 => Some(Departement::HautesAlpes),
        6 => Some(Departement::AlpesMaritimes),
        7 => Some(Departement::Ardeche),
        8 => Some(Departement::Ardennes),
        9 => Some(Departement::Ariege),
        _ => None,
    }
}
