use crate::chap5::generic_struct::Exemple2;

mod chap4;
mod chap5;

fn main() {
    // println!("Hello, world!");

    // Possession et référence
    chap4::own_borrow::possession_chaine_tas();
    chap4::own_borrow::possession_entier_pile();
    chap4::own_borrow::possession_fonction();
    chap4::own_borrow::possession_retour_fonction();

    // Solution 1 : Transfert de propriété de valeur
    let chaine = String::from("ENI");
    let (chaine_retour, taille) = chap4::own_borrow::obtenir_taille(chaine);
    println!("{}", chaine_retour);
    println!("{}", taille);

    // Solution 2 : prêt de valeur
    let chaine2 = String::from("ENI");
    let taille2 = chap4::own_borrow::obtenir_taille_2(&chaine2);
    println!("{}", chaine2);
    println!("{}", taille2);

    let mut chaine3 = String::from("ENI");
    chap4::own_borrow::modifier_chaine(&mut chaine3);
    println!("{}", chaine3);

    // # Chapitre 5 : Structure
    let cercle_1 = chap5::exemple_struct::Cercle {
        coord_centre_x: 5,
        coord_centre_y: 5,
        rayon_cercle: 2,
        nom_cercle: String::from("cercle_1"),
    };
    println!("{}", cercle_1.coord_centre_x);
    println!("{}", cercle_1.coord_centre_y);
    println!("{}", cercle_1.rayon_cercle);
    println!("{}", cercle_1.nom_cercle);

    let point_a = chap5::exemple_struct::Point(1, 2, String::from("A"));
    println!("{}", point_a.0);
    println!("{}", point_a.1);
    println!("{}", point_a.2);

    let rect = chap5::exemple_struct::Rectangle::new(5_f64, 10_f64);
    println!("Périmètre : {}", rect.perimetre());
    println!("Aire : {}", rect.aire());
    println!("Valeur entière largeur : {}", rect.obtenir_largeur_int());
    println!("Valeur entière longueur : {}", rect.obtenir_longueur_int());

    println!("--- Exemple vecteur d'entiers ---");
    let mut mon_vecteur = chap5::generic_struct::VecteurEntiers::creer();

    println!(
        "Taille courante du vecteur : {}",
        mon_vecteur.obtenir_longueur()
    );

    mon_vecteur.ajouter(1);
    mon_vecteur.ajouter(2);
    mon_vecteur.ajouter(3);

    println!(
        "Taille courante du vecteur : {}",
        mon_vecteur.obtenir_longueur()
    );
    println!(
        "Quel est l'élément d'indice 2 : {}",
        mon_vecteur.obtenir_element(2)
    );
    println!(
        "On demande un indice au-delà de la taille 
du vecteur : {}",
        mon_vecteur.obtenir_element(42)
    );

    let mut mon_vecteur_generique = chap5::generic_struct::VecteurGenerique::<f64>::creer();
    mon_vecteur_generique.ajouter(5.0);
    mon_vecteur_generique.ajouter(4.9);
    mon_vecteur_generique.ajouter(4.8);
    mon_vecteur_generique.ajouter(4.7);

    let mut mon_vecteur_generique_2 = chap5::generic_struct::VecteurGenerique::creer();
    mon_vecteur_generique_2.ajouter("Hector");
    mon_vecteur_generique_2.ajouter("Arthur");
    mon_vecteur_generique_2.ajouter("Sophie");
    // mon_vecteur_generique_2.ajouter(2); # Ne marche pas

    let valeur: i64 = 5;
    let exemple = chap5::generic_struct::Exemple { ii: &valeur };
    println!("Affichage de ii : {}", exemple.ii);

    static VALEUR_2: Exemple2 = chap5::generic_struct::Exemple2 { ii: 3 };

    let ref_static_valeur2: &'static Exemple2 = &VALEUR_2;
    println!("Affichage de ii - exemple 2 : {}", ref_static_valeur2.ii);

    static VALEUR_3: i64 = 2;
    let exemple_3 = chap5::generic_struct::Exemple3 { ii: &VALEUR_3 };

    println!("Affichage de ii - exemple 3 : {}", exemple_3.ii);

    let p1 = chap5::generic_struct::Point { xx: 2, yy: 3 };

    // Grâce au trait Clone
    let p2 = p1.clone();
    println!("xx après clonage : {}", p2.xx);
    println!("yy après clonage : {}", p2.yy);

    // Grâce au trait Copy
    let p3 = p2;
    println!("xx après copie : {}", p3.xx);
    println!("yy après copie : {}", p3.yy);

    // Grâce au trait Debug
    println!("p3 : {:?}", p3);

    // Grâce au trait PartialEq
    if p3 == p1 {
        println!("PartialEq : {}", p3 == p1);
    }
}
