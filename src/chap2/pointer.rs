#![allow(non_snake_case)]

#[path = "./pointer2.rs"]
mod pointer2;

fn obtenir_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn exempleReferences() {
    let abc = std::f64::consts::PI;
    let refNonMutable = &abc;
    println!("{}", refNonMutable);

    let mut fgh = 3.14 * 2.0;
    let refMutable = &mut fgh;
    println!("{}", refMutable);

    println!("abc : {}, adresse : {:p}", abc, &abc);
    println!(
        "refNonMutable : {}, adresse : {:p}",
        refNonMutable, refNonMutable
    );
    println!("adresse refNonMutable : {:p}", &refNonMutable);
}

pub fn exempleBoites() {
    println!("---------");

    let valeur = Box::new(123);
    obtenir_type(&valeur);
    println!("valeur = {}", valeur);
    println!("adresse pointée = {:p}", &valeur);

    println!("---------");

    let t = (3.4, 12, "test");
    let valeur2 = Box::new(t);
    obtenir_type(&valeur2);
    println!("valeur2.0 = {}", valeur2.0);
    println!("valeur2.1 = {}", valeur2.1);
    println!("valeur2.2 = {}", valeur2.2);
    println!("adresse pointée = {:p}", &valeur2);

    println!("Called pointer2.rs from pointer.rs");
    pointer2::stuff_pointer();
}

pub fn exemplePointeursBruts() {
    let mut valeur = 5;

    let ptr_brut = &valeur as *const i32;
    let ptr_brut_mut = &mut valeur as *mut i32;

    unsafe {
        println!("ptr_brut : {}", *ptr_brut);
        println!("ptr_brut_mut : {}", *ptr_brut_mut);
    }
}
