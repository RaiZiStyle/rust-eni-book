#![allow(unused_variables)]

// use crate::chap7::traits::mod_animal::Animal;

mod chap2;
mod chap3;
mod chap4;
mod chap5;
mod chap6;
mod chap7;

extern crate animal;
use crate::animal::mod_animal::Animal;



fn main() {
    chap2::pointer::exempleBoites();
    chap2::pointer::exempleReferences();
    chap2::pointer::exemplePointeursBruts();

    // chap2::pointer2::stuff_pointer(); // For test

    chap2::vec_array_slice::exempleSlice();
    chap2::string::exempleString();

    chap3::own_borrow::possession_chaine_tas();
    chap3::own_borrow::possession_entier_pile();

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

    static VALEUR_2: chap5::generic_struct::Exemple2 = chap5::generic_struct::Exemple2 { ii: 3 };

    let ref_static_valeur2: &'static chap5::generic_struct::Exemple2 = &VALEUR_2;
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

    // Chapitre 6 : Enumérations et motifs dans Rust
    let ain: i64 = chap6::enums::Departement::Ain as i64;
    println!("{:?}", ain);
    let aisne: i64 = chap6::enums::Departement::Ardeche as i64;
    println!("{:?}", aisne);

    let resultat_reste_1 = chap6::enums::reste_division(16, 9);
    println!("{:?}", resultat_reste_1);

    let resultat_reste_2 = chap6::enums::reste_division(13, 0);
    println!(
        "Valeur entière : {}",
        chap6::enums::obtenir_valeur_numerique(resultat_reste_1)
    );
    println!(
        "Valeur entière : {}",
        chap6::enums::obtenir_valeur_numerique(resultat_reste_2)
    );

    let departement_1: Option<chap6::enums::Departement> = chap6::enums::i64_vers_enum(5);
    println!(
        "Département correspondant au numéro 5 : {:?}",
        departement_1
    );
    let departement_135: Option<chap6::enums::Departement> = chap6::enums::i64_vers_enum(135);
    println!(
        "Département correspondant au numéro 135 : {:?}",
        departement_135
    );

    let alpesmaritimes: chap6::enums::Departement = chap6::enums::Departement::AlpesMaritimes;
    let alpesmaritimes_str = alpesmaritimes.conversion_str();
    println!(
        "Méthode d'énumération utilisant un match : {}",
        alpesmaritimes_str
    );

    let a: chap6::enum_struct::Exemple = chap6::enum_struct::Exemple::A;
    let b: chap6::enum_struct::Exemple = chap6::enum_struct::Exemple::B(Some("test B".to_string()));
    let c: chap6::enum_struct::Exemple = chap6::enum_struct::Exemple::C {
        champ_nomme_1: "test B".to_string(),
        champ_nomme_2: 23,
    };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let noeud1: chap6::enum_struct::Noeud<i64> = chap6::enum_struct::Noeud {
        valeur: 1,
        gauche: chap6::enum_struct::ArbreBinaire::Terminaison,
        droite: chap6::enum_struct::ArbreBinaire::Terminaison,
    };
    let arbre1: chap6::enum_struct::ArbreBinaire<i64> =
        chap6::enum_struct::ArbreBinaire::NonTerminaison(Box::new(noeud1));

    let noeud2: chap6::enum_struct::Noeud<i64> = chap6::enum_struct::Noeud {
        valeur: 2,
        gauche: chap6::enum_struct::ArbreBinaire::Terminaison,
        droite: chap6::enum_struct::ArbreBinaire::Terminaison,
    };
    let arbre2: chap6::enum_struct::ArbreBinaire<i64> =
        chap6::enum_struct::ArbreBinaire::NonTerminaison(Box::new(noeud2));

    let noeud3: chap6::enum_struct::Noeud<i64> = chap6::enum_struct::Noeud {
        valeur: 3,
        gauche: arbre1,
        droite: arbre2,
    };
    let arbre3: chap6::enum_struct::ArbreBinaire<i64> =
        chap6::enum_struct::ArbreBinaire::NonTerminaison(Box::new(noeud3));

    let h: chap6::enum_struct::Personnage = chap6::enum_struct::Personnage::Heros;
    let f = chap6::enum_struct::Personnage::Fantome {
        points_de_vie: 25,
        indice_invisibilite: 12,
    };
    let c = chap6::enum_struct::Personnage::Combattant { points_de_vie: 35 };
    let m = chap6::enum_struct::Personnage::Magicien(34, 78);

    println!("{}", chap6::enum_struct::filtrage_par_motif(h));
    println!("{}", chap6::enum_struct::filtrage_par_motif(f));
    println!("{}", chap6::enum_struct::filtrage_par_motif(c));
    println!("{}", chap6::enum_struct::filtrage_par_motif(m));

    let prenom = "Hector";
    match prenom {
        "Arthur" | "Sophie" | "Hector" => println!("Quel joli prénom."),
        autre => println!("Quel prénom joli."),
    };

    // Chapitre 7 : Les trais en Rust
    // animal::add(2, 2);

    let nom_chien: String = "Toto le cabot".to_string();
    let toto = animal::mod_animal::Chien::creer(nom_chien);
    println!("{}", toto.obtenir_nom());
    toto.afficher();

    let nom_chat : String = "Kiki le chaton".to_string(); 
    let kiki = animal::mod_animal::Chat::creer(nom_chat); 
    println!("{}", kiki.obtenir_nom()); 
    kiki.dormir(); 
    kiki.afficher(); 

    println!("Le plus long nom des deux : {}", 
    animal::mod_animal::getbiggestname(&kiki, &toto));

    // Une tortue (implémentation de deux traits). 
    println!("--- Deux traits, Tortue et Display ---"); 
    let nom_tortue : String = "Tutu la tortue".to_string(); 
    let tutu = chap7::traits_with_param::Tortue::creer(nom_tortue); 
    println!("{}", tutu); 
    chap7::traits_with_param::affichage_format(&tutu); 
    // Marche pas car, toto : Chat n'implémente pas la fonction display
    // chap7::traits_with_param::affichage_format(&toto);


    let petit_goujon = chap7::traits_with_param::Goujon::creer("Toto".to_string());
    petit_goujon.afficher();

    // let gros_poisson = chap7::traits_with_param::Poisson



}
