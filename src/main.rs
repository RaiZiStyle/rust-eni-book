// use Chap2::pointer::exempleBoites;

// #[path = "./Chap2/pointer.rs"]
// #[allow(non_snake_case)]
// mod Chap2;
// use Chap2::{exempleBoites, exemplePointeursBruts, exempleReferences};

mod chap2;
// use Chap2::{exempleBoites, exemplePointeursBruts, exempleReferences};
// use chap2::pointer::{exempleBoites};

mod chap3;

fn main() {

    chap2::pointer::exempleBoites();
    chap2::pointer::exempleReferences();
    chap2::pointer::exemplePointeursBruts();

    // chap2::pointer2::stuff_pointer(); // For test

    chap2::vec_array_slice::exempleSlice();
    chap2::string::exempleString();


    chap3::own_borrow::possession_chaine_tas();
    chap3::own_borrow::possession_entier_pile();
}
