// Les variables globales sont déclarées en dehors de tous contextes.
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Accès à la constante dans une fonction.
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Accès à la constante dans le fil d'exécution principal.
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Erreur! Vous ne pouvez pas modifier une constante.
    // THRESHOLD = 5;
    // FIXME ^ Commentez cette ligne pour voir disparaître
    // le message d'erreur.
}
