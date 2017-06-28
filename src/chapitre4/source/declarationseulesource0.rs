fn main() {
    // On déclare une variable.
    let a_binding;

    {
        let x = 2;

        // On initialise la variable.
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Erreur! Utilisation d'une variable non-initialisée.
    // println!("another binding: {}", another_binding);
    // FIXME ^ Commentez cette ligne pour voir l'erreur disparaître.
    another_binding = 1;

    println!("another binding: {}", another_binding);
}