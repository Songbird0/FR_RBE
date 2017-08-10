// `elided_input` et `annotated_input` ont fondamentalement la même signature, 
// sauf que la lifetime de l'entrée de la fonction `elided_input` a été omise 
// et ajoutée par le compilateur.
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x)
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x)
}

// Même combat, `elided_pass` et `annotated_pass` possèdent la même signature 
// sauf que la lifetime de `x`, pour la fonction `elided_pass`, a été omise 
// et ajoutée par le compilateur. Annotation implicite.
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
