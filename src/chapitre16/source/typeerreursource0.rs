fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Génère la première erreur.
    2 * first.parse::<i32>().unwrap() // Génère la seconde erreur.
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(empty));
    // Erreur 1: Le vecteur passé en entrée est vide.

    println!("The first doubled is {}", double_first(strings));
    // Erreur 2: L'élément ne peut pas être converti en nombre.
}
