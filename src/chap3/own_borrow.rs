#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn possession_chaine_tas() {
    let chaine = String::from("ENI");
    let chaine2 = chaine;

    println!("{}", chaine2);
    //println!("{}", chaine); // provoque une erreur car la valeur
    // a été prêtée.
    let chaine_clone = chaine2.clone();
    println!("{:p}", &chaine2);
    println!("{:p}", &chaine_clone);
}

pub fn possession_entier_pile() {
    let annee_hector: i64 = 2011;
    let annee_hector_2: i64 = annee_hector; // equivalent a annee_hector.copy()

    println!("{}", annee_hector);
    println!("{:p}", &annee_hector);
    println!("{}", annee_hector_2);
    println!("{:p}", &annee_hector_2);
}

pub fn possession_fonction() {
    let chaine = String::from("ENI");
    let annee_arthur: i64 = 2007;

    println!("{}", chaine);
    println!("{}", annee_arthur);

    passage_fonction(chaine, annee_arthur);

    let valeur = annee_arthur;
    println!("{}", valeur);
    //println!("{}", chaine); // Déclenche une erreur de
    //  compilation, la valeur a été déplacée.
}

fn passage_fonction(texte: String, entier: i64) {
    println!("{}", texte);
    println!("{}", entier);
}
