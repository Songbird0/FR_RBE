fn main() {
    // Le type des variables peut être spécifié, annoté.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // typage classique
    let an_integer   = 5i32; // Typage par suffixe

    // Le type par défaut peut également être conservé.
    // typage implicite
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // Entier signé codé sur 4 octets (i32).

    // Erreur! Le type d'une variable ne peut être modifié en cours de route.
    // mutable = true;
}