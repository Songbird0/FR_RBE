// `print_refs` prend deux références d'entiers possédant une durée de 
// vie différente chacune (i.e. `'a` et `'b`). Ces deux lifetimes doivent être 
// au moins aussi longues que celle de la fonction `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Une fonction qui ne prend aucun argument, mais qui déclare quand même 
// une lifetime `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERREUR: `_x` ne vit pas assez longtemps.
    // let y: &'a i32 = &_x;
    // Tenter d'utiliser la lifetime `'a` sur une ressource au sein de la fonction
    // ne fonctionnera pas car la durée de vie de `&_x` est plus courte que celle 
    // de `y` .
    // Une courte durée de vie ne peut être rallongée en cours de route.
}

fn main() {
    // On crée ces variables pour les emprunter 
    // un peu plus bas.
    let (four, nine) = (4, 9);

    // `print_refs` emprunte (`&`) les deux ressources précédemment 
    // créées.
    print_refs(&four, &nine);
    // Une ressource empruntée doit survivre à la fonction qui l'emprunte.
    // Autrement dit, la durée de vie de `four` et `nine` doit être 
    // plus longue que celle de `print_refs`.

    failed_borrow();

    // `failed_borrow` ne contient aucune référence qui force la lifetime `'a` à 
    // être plus longue que celle de la fonction, mais `'a` reste plus longue.
    // Parce que la durée de vie d'une ressource (empruntée) n'est jamais imposée, 
    // la durée de vie par défaut est `'static`.
}
