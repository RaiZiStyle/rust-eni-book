// use Chap2::pointer::exempleBoites;

#[path = "./Chap2/pointer.rs"]
#[allow(non_snake_case)]
mod Chap2;
// use Chap2::{exempleBoites, exemplePointeursBruts, exempleReferences};


fn main() {
    Chap2::exempleBoites();
    Chap2::exempleReferences();
    Chap2::exemplePointeursBruts();
}
