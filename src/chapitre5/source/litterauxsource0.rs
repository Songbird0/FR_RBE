fn main() {
    // Ces littéraux sont suffixés, leurs types sont connus à l'initialisation.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Ces littéraux ne sont pas suffixés, leurs types dépendent du contexte.
    let i = 1;
    let f = 1.0;

    // La fonction `size_of_val` renvoie la taille d'une variable en octets.
    println!("La taille de `x` en octets: {}", std::mem::size_of_val(&x));
    println!("La taille de `y` en octets: {}", std::mem::size_of_val(&y));
    println!("La taille de `z` en octets: {}", std::mem::size_of_val(&z));
    println!("La taille de `i` en octets: {}", std::mem::size_of_val(&i));
    println!("La taille de `f` en octets: {}", std::mem::size_of_val(&f));
}