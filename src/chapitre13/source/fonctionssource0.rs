// La référence passée en paramètre possédant la lifetime `'a` 
// doit vivre au moins aussi longtemps que le fonction.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// L'utilisation des références mutables est également possible 
// avec les lifetimes.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Plusieurs entrées avec différentes lifetimes. Dans ce cas, 
// il serait plus simple, pour les deux paramètres, d'avoir 
// la même durée de vie (`'a`), mais dans des cas plus délicats, 
// plusieurs lifetimes peuvent être requises.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Renvoyer des références qui ont été passées en paramètre est légal.
// Toutefois, veillez à renvoyer la bonne lifetime.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a i32 { &7 }
// La déclaration ci-dessus est invalide: `'a` doit, au moins, 
// survivre à la fonction. Ici, `&7` créerait un entier et récupérerait 
// sa référence. La ressource serait alors perdue une fois l'exécution 
// de la fonction terminée, renvoyant une référence d'une ressource qui n'existe plus.

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
