// raii.rs
fn create_box() {
    // Allocation d'un entier dans le tas.
    let _box1 = Box::new(3i32);

    // `_box1` est détruit ici, et la mémoire est libérée.
}

fn main() {
    // Allocation d'un entier dans le tas.
    let _box2 = Box::new(5i32);

    // Contexte imbriqué:
    {
        // Allocation d'un entier dans le tas.
        let _box3 = Box::new(4i32);

        // `_box3` est détruit ici, et la mémoire est libérée.
    }

    // On créé ici un grand nombre de "box" juste pour l'exemple.
    // Il n'y a pas besoin de libérer manuellement la mémoire !
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` est détruit ici, et la mémoire est libérée.
}
