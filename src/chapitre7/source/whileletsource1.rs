fn main() {
    // Crée une valeur optionnelle de type `Option<i32>`.
    let mut optional = Some(0);

    // Fonctionnement: "`while let` déstructure `optional` pour assigner sa valeur 
    // à Some(i) puis exécute le bloc (`{}`). Sinon, on sort de la boucle."
    while let Some(i) = optional {
        if i > 9 {
            println!("Plus grand que 9, on quitte!");
            optional = None;
        } else {
            println!("`i` est égal à `{:?}`. On réitère.", i);
            optional = Some(i + 1);
        }
        // Moins explicite, il n'est plus nécessaire de gérer 
        // le cas où la déstructuration échoue.
    }
    // ^ `if let` permet d'ajouter des branches `else`/`else if` 
    // optionnelles. `while let` ne le permet pas, en revanche.
}