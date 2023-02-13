// use Chap2::pointer::exempleBoites;

// #[path = "./Chap2/pointer.rs"]
// #[allow(non_snake_case)]
// mod Chap2;
// use Chap2::{exempleBoites, exemplePointeursBruts, exempleReferences};

mod chap2;
// use Chap2::{exempleBoites, exemplePointeursBruts, exempleReferences};
// use chap2::pointer::{exempleBoites};

fn main() {

    chap2::pointer::exempleBoites();
    chap2::pointer::exempleReferences();
    chap2::pointer::exemplePointeursBruts();
}
