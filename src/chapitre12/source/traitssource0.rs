// Types non-copiables.
struct Empty;
struct Null;

// Un trait générique qui reçoit un type générique `T`.
trait DoubleDrop<T> {
    // On déclare une méthode qui sera implémentée par la structure 
    // appelante (caller) et prendra un paramètre `T` en entrée mais n'en fera rien (juste le libérer). 
    fn double_drop(self, _: T);
}

// On implémente `DoubleDrop<T>` pour n'importe quel paramètre `T` et 
// n'importe quelle structure appelante (`U`).
impl<T, U> DoubleDrop<T> for U {
    // Cette méthode prend "possession" des deux paramètres (`U` et `T`),
    // et sont donc tous deux libérés.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;
    
    // `empty` et `null` sont désalloués.
    empty.double_drop(null);

    // empty;
    // null;
    // ^ TODO: Essayez de décommenter ces lignes.
}
