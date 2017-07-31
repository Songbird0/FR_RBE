// Ré-implémentation de la division d'entiers (/).
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // La division par zéro fait planter le thread courant.
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}


fn main() {
    // Entier alloué dans le tas.
    let _x = Box::new(0i32);

    // Cette opération va déclencher la procédure d'abandon.
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` devrait être détruit à ce niveau.
}
