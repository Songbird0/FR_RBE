use std::path::{Path, PathBuf};
use std::ffi;

fn main() {
    // Création d'un objet `Path` à partir d'une `&'static str`.
    let path = Path::new(".");

    // La méthode `display` renvoie une structure présentable.
    let display = path.display();

    // La méthode `join()` fusionne un chemin avec toutes les ressources 
    // possédant une implémentation de la méthode `as_ref()` pour le type `Path` et 
    // renvoie un nouveau chemin avec le séparateur spécifique à l'OS.
    let new_path = path.join("a").join("b");

    // Convertit le chemin en une vue sur une string.
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
