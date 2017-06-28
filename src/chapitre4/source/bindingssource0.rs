fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copie `an_integer` dans `copied_integer`.
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer); // un entier
    println!("A boolean: {:?}", a_boolean); // un booléen
    println!("Meet the unit value: {:?}", unit); // rien

    // Le compilateur vous alertera lorsqu'il détecte une variable inutilisée; 
    // Vous pouvez faire taire ces avertissements en préfixant l'identificateur 
    // de la variable avec un underscore (i.e. _).
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Préfixez cet identificateur avec un underscore pour supprimer 
    // l'avertissement.
}
