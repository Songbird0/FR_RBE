 fn main() {
    // Assigne une référence de type `i32`. Le `&` signifie qu'une 
    // référence est assignée.
    // L'équivalent non-raccourci de cette assignation pourrait ressembler à ceci:
    // ```rust
    // let _reference: i32 = 4;
    // let reference: &i32 = &_reference;
    // ```
    let reference: &i32 = &4;

    match reference {
        // Lorsque `reference` est comparé à `&val`, la comparaison ressemble à ceci:
        // `&i32`
        // `&val` <- `val` est plus ou moins une représentation de `reference`.
        // ^ Nous remarquons que si le `&` est omis, la valeur devrait être 
        // assignée à `val`.
        &val => println!("On récupère une valeur via déstructuration: {:?}", val),
    }

    // Pour éviter d'utiliser la référence, vous pouvez déréférencer `reference` 
    // avant analyse (vous permettant d'opérer sur la valeur, si elle est mutable).
    match *reference {
        val => println!("On récupère la valeur déréférencée: {:?}", val),
    }

    // Que se passe-t-il si vous ne créez pas une référence ? `reference` 
    // était une référence parce que la r-value était une référence. Cette 
    // variable n'en est pas une parce que la r-value n'en est pas une.
    let _not_a_reference = 3;

    // Rust fournit le mot-clé `ref` dans ce but. Il modifie l'assignation 
    // de manière à créer une référence pour l'élément; cette référence est assignée.
    let ref _is_a_reference = 3;

    // Bien entendu, en assignant deux valeurs sans références, ces dernières
    // peuvent être récupérées à l'aide du mot-clé `ref` et `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // On utilise le mot-clé `ref` pour créer une référence.
    match value {
        ref r => println!("On récupère une référence de la valeur: {:?}", r),
    }

    // `ref mut` s'utilise de la même manière.
    match mut_value {
        ref mut m => {
            // On obtient une référence. Nous allons déréférencer `m` avant 
            // de pouvoir opérer.
            *m += 10;
            println!("Nous incrémentons de 10. `mut_value`: {:?}", m);
        },
    }
}
