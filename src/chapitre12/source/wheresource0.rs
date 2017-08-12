use std::fmt::{Debug, Formatter, Result};

trait PrintInOption {
    fn print_in_option(self);
}

// Parce que nous pourrions modifier la restriction sans spécifier le 
// conteneur `T: Debug` ou adopter une autre approche,
// il est nécessaire d'utiliser la condition `where`: 
impl<T> PrintInOption for T where
    Option<T>: Debug{
    // Nous spécifions la restriction de type `Option<T>: Debug` parce que 
    // c'est ce que nous souhaitons afficher. Faire autrement pourrait nous 
    // induire en erreur quant au type de restriction à spécifier.
    fn print_in_option(self) {
        println!("{:?}", Some(self)); 
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
