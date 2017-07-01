// Crée une valeur optionnelle de type `Option<i32>`.
let mut optional = Some(0);

// On répète le test.
loop {
    match optional {
        // Si il est possible de déstructurer `optional`, 
        // le bloc sera exécuté.
        Some(i) => {
            if i > 9 {
                println!("Plus grand que 9, on quitte!");
                optional = None;
            } else {
                println!("`i` est égal à `{:?}`. On réitère.", i);
                optional = Some(i + 1);
            }
            // ^ Nécessite trois niveaux d'indentations.
        },
        // On quitte la boucle si la déstructuration 
        // a échoué:
        _ => { break; }
        // ^ Pourquoi cette instruction devrait être nécessaire ?
        // Il doit y avoir une solution plus adaptée!
    }
}