use std::fmt::{Debug, Formatter, Result};

struct S;

impl Debug for S{
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f, "S()")
    }
}

fn print_in_option<T: Debug>(opt: T) -> (){
    println!("{:?}", Some(opt)); // pas de vérifications, on peut 
    // très bien retirer le conteneur, aucune erreur ne sera renvoyée 
    // par le compilateur.
}

fn main() {
    let vec = vec![1, 2, 3];

    print_in_option(vec);
    print_in_option(S)
}
