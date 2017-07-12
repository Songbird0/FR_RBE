fn used_function() {}

// L'attribut `#[allow(dead_code)]` désactive la lint `dead_code`.
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Ajoutez un attribut pour supprimer l'avertissement.

fn main() {
    used_function();
}
