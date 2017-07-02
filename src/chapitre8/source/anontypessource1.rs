// `F` doit implémenter `Fn` pour une closure qui ne prend aucun 
// argument et ne renvoie rien - exactement ce qui est nécessaire 
// pour `print`.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Capture la variable `x` dans une structure anonyme 
    // et implémente `Fn` pour cette dernière. On stocke dans `print`.
    let print = || println!("{}", x);

    apply(print);
}
