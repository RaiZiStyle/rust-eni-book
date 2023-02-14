mod chap4;

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
}
