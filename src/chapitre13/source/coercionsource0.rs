// Ici, Rust va inférer une durée de vie aussi courte que possible. 
// Les deux références sont alors assignées à cette durée de vie.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` se lit comme suit: La lifetime `'a` est au moins aussi longue 
// que `'b`. Ici, nous prenons en entrée un entier soumis à la lifetime 
// `&'a i32` et renvoyons un entier (le même entier, en fait) soumis à la lifetime 
// `&'b i32` comme résultat de la coercition.
// Note: Nous pouvons renvoyer l'entier taggé avec la lifetime `'b` car `'a` et `'b`
// ont une durée de vie aussi longue, pour le temps de l'exécution de la fonction, 
// tout du moins.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longue lifetime.

    {
        let second = 3; // Courte lifetime.

        // Note: Ici `first` et `second` possèdent la même durée de vie.
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
