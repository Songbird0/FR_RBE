// Une simple macro nommée `say_hello`.
macro_rules! say_hello {
    // `()` indique que la macro ne prend aucun argument.
    () => (
        // La macro étendra le contenu de ce bloc.
        println!("Hello!");
    )
}

fn main() {
    // Cet appel va étendre `println!("Hello");`.
    say_hello!()
}
