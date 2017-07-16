use std::fmt::{Debug, Formatter, Result};

/* Le type `S` n'est pas présent dans l'exemple original, vous pouvez 
consulter l'exemple ici: http://rustbyexample.com/generics/where.html
*/

struct S;


impl Debug for S{
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f, "S()")
    }
}

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
        println!("{:?}", Some(self)); // par exemple,
        // si vous retirez l'instance `Some()`, la condition 
        // ne sera pas respectée et le compilateur renverra une erreur
        // même si le type `T` implémente le trait `Debug`.
        // ^ Essayez de remplacer `println!("{:?}", Some(self));` 
        // par `println!("{:?}", self);`
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
    S.print_in_option();
}
