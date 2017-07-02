// On déclare une fonction qui prend l'argument générique `F`
// délimité par le trait `Fn` et appelle la fonction (ou closure).
fn call_me<F: Fn()>(f: F) {
    f();
}

// On déclare une fonction qui satisfait la délimitation (hériter de `Fn`).
fn function() {
    println!("I'm a function!");
}

fn main() {
    // On déclare une closure qui satisfait la délimitation (hériter de `Fn`).
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
