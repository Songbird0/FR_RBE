// On fait hériter l'implémentation de `fmt::Debug` pour `Structure`.
// `Structure` est une structure qui contient un simple entier de type `i32`.
#[derive(Debug)]
struct Structure(i32);

// On créé une structure nommée `Deep`, que l'on rend également affichable,
// contenant un champ de type `Structure`,
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // L'affichage avec le marqueur `{:?}` est similaire à `{}`,
    // pour des types standards comme les entiers et les chaînes de caractères.
    println!("{:?} mois dans une année.", 12);
    println!("{1:?} {0:?} est le nom de {actor:?}.",
             "Slater",
             "Christian",
             actor="l'acteur");

    // `Structure` peut être affichée !
    println!("{:?} peut désormais être affichée!", Structure(3));

    // Le problème avec `derive` est que vous n'avez aucun contrôle quant au résultat
    // affiché. Comment faire si je souhaite seulement afficher `7` ?
    println!("{:?} peut désormais être affichée!", Deep(Structure(7)));
}