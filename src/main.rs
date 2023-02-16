mod chap4;
mod chap5;

fn main() {
    // println!("Hello, world!");

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

    
}
