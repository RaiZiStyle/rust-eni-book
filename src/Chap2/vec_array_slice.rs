#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn exempleArray() {
    let mut tableauEntiers: [u32; 8] = [2, 1, 3, 5, 6, 4, 7, 8];
    println!("4e case du tableau d'entiers : {}", tableauEntiers[3]);
    tableauEntiers.sort();
    for i in 0..tableauEntiers.len() {
        println!("Chaque case après tri : {}", tableauEntiers[i]);
    }

    println!("----------");

    let tableauFlottants = [3.4, 2.0, 5.1];
    println!(
        "Taille du tableau de flottants : {}",
        tableauFlottants.len()
    );
    println!("3e case du tableau de flottants : {}", tableauFlottants[2]);
}

pub fn exempleVecteur() {
    let mut vecteur1 = vec![1, 2, 3, 4];
    vecteur1.push(5);
    println!("Longueur : {}", vecteur1.len());
    vecteur1.pop();
    println!("Longueur : {}", vecteur1.len());

    let vecteur1_iter = vecteur1.iter();
    for valeur in vecteur1_iter {
        println!("Valeur : {}", valeur);
    }
}

pub fn exempleSlice() {
    let tableau = [1, 2, 3, 4, 5];
    let tranche = &tableau[0..2];

    println!("Longueur tranche : {:?}", tranche.len());
    println!("Tranche : {:?}", tranche);
    println!("Tableau : {:?}", tableau);

    println!("----------");

    let mut vecteur = Vec::new();
    vecteur.push("A");
    vecteur.push("B");
    vecteur.push("C");
    let trancheVecteur = &vecteur[0..2];

    println!("Longueur trancheVecteur : {:?}", trancheVecteur.len());
    println!("Tranche : {:?}", trancheVecteur);
    println!("Vecteur : {:?}", vecteur);
}
