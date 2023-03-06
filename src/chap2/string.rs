#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn exempleString() {
    let mut chaine1 = String::new();
    chaine1.push('a');
    chaine1.push('b');
    chaine1.push('c');
    println!("{}", chaine1);

    let chaine2 = String::from("def");
    println!("{}", chaine2);

    let chaine3 = "ghi".to_string();
    println!("{}", chaine3);

    let trancheChaine3 = &chaine3[1..];
    println!("{}", trancheChaine3);

    println!("Inclut 'hi' ? : {}", chaine3.contains("hi"));

    let chaine4 = chaine3.replace("i", "L");
    println!("Après remplacement : {}", chaine4);

    for lettre in chaine4.split("h") {
        println!("Lettre : {}", lettre);
    }
}
