/// Crée une chaine dans le tas
/// et prend la possession
pub fn possession_chaine_tas() {
    let chaine = String::from("ENI");
    let chaine2 = chaine;

    println!("{}", chaine2);

    // Copie dans le tas : deux emplacements mémoire différents
    let chaine_clone = chaine2.clone();
    println!("{:p}", &chaine2);
    println!("{:p}", &chaine_clone);
}

/// Crée un entier dans la pile
/// et prend la possession
pub fn possession_entier_pile() {
    let annee_hector: i64 = 2011;
    let annee_hector_2: i64 = annee_hector; // equivalent a annee_hectoir.copy();

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

pub fn possession_retour_fonction() {
    let chaine = String::from("ENI");
    let chaine_retour = possession_retour_fonction_appel(chaine);
    println!("{}", chaine_retour);
}

/// Prend la possession pour obtenir la taille
/// Return Tuple {chaine, taille}
pub fn obtenir_taille(ch: String) -> (String, usize) {
    let taille = ch.len();
    (ch, taille)
}

/// Calcule la taille d'une chaine
/// sans prendre la possession
pub fn obtenir_taille_2(ch: &String) -> usize {
    ch.len()
}

/// Fonction qui ajoute un caractère a une chaine mutable
pub fn modifier_chaine(ch: &mut String) {
    ch.push_str(" !");
}

fn passage_fonction(texte: String, entier: i64) {
    println!("{}", texte);
    println!("{}", entier);
}
fn possession_retour_fonction_appel(chaine_param: String) -> String {
    chaine_param
}
