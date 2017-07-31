// Cette fonction prend possession d'une box et la détruit.
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// Cette fonction emprunte un entier signé
// codé sur 32 bits.
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // On créé entier alloué dans le tas 
    // et un autre alloué dans la pile.
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // `borrow_i32()` emprunte le contenu de la box.
    // La fonction ne prend pas possession des ressources, 
    // donc le contenu peut être de nouveau emprunté.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Récupère une référence de l'entier contenu dans la box.
        let _ref_to_i32: &i32 = &boxed_i32;

        // Erreur!
        // Vous ne pouvez pas détruire `boxed_i32` tant que la valeur 
        // qu'il contient est empruntée.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Essayez de décommenter cette ligne.

        // La durée de vie de `_ref_to_i32` prend fin ici,
        // l'entier n'est donc plus emprunté.
    }

    // `boxed_i32` peut désormais être possédé par `eat_box()` et peut donc 
    // être détruit.
    eat_box_i32(boxed_i32);
}
