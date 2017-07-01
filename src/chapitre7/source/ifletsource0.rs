// Crée une valeur optionnelle de type `Option<i32>`.
let optional = Some(7);

match optional {
    Some(i) => {
        println!("Ceci est une très longue chaîne de caractères contenant un 
        `{:?}`", i);
        // ^ Deux niveaux d'indentations sont nécessaires alors 
        // que nous aurions pu simplement déstructurer `i`.
    },
    _ => {},
    // ^ Nécessaire parce que `match` est exhaustif. Cette branche vous 
    // paraît-elle utile?
};