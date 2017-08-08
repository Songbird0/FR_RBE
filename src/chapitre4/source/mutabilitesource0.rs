fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Avant modification: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("Après modification: {}", mutable_binding);

    // Erreur!
    //_immutable_binding += 1;
    // FIXME ^ Décommentez cette ligne pour voir le message d'erreur
}
