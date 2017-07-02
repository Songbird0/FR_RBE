fn main() {
    // Incrémentation avec les closures et fonctions.
    fn  function            (i: i32) -> i32 { i + 1 }

    // Les closures sont anonymes, ici nous assignons leurs références.
    // Les closures sont typées de la même manière qu'une fonction classique
    // mais le typage est optionnel. Chaque version (raccourcie et non-raccourcie)
    // est assignée à un identificateur approprié.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Appelle la fonction et les closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // Une closure qui ne prend aucun argument et renvoie un 
    // entier de type `i32`.
    // Le type de renvoi est inféré.
    let one = || 1;
    println!("closure returning one: {}", one());

}
