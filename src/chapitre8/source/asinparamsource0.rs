// Une fonction qui prend une closure en paramètre et appelle cette dernière.
fn apply<F>(f: F) where
    // La closure ne prend rien et ne renvoie rien.
    F: FnOnce() {
    // ^ TODO: Essayez de remplacer ce trait par `Fn` ou `FnMut`.

    f();
}

// Une fonction qui prend une closure en paramètre et renvoie un entier
// de type `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // La closure prend en paramètre un `i32` et renvoie 
    // un `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // Un type qui ne peut pas être copié.
    // `to_owned` crée une ressource dont 
    // l'assignation `farewell` sera responsable, à partir d'une ressource empruntée.
    let mut farewell = "goodbye".to_owned();

    // Capture deux variables: `greeting` par référence et 
    // `farewell` par valeur.
    let diary = || {
        // `greeting` est capturé par référence: requiert `Fn`.
        println!("I said {}.", greeting);

        // Le fait de modifier `farewell` rend obligatoire 
        // la capture par référence mutable, le compilateur choisira 
        // donc `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Appeler manuellement la fonction `drop` nécessite 
        // désormais de capturer par valeur `farewell`, le compilateur 
        // choisira alors `FnOnce`.
        mem::drop(farewell);
    };

    // On appelle la fonction qui prend en paramètre la closure.
    apply(diary);

    // `double` satisfait les conditions du trait soumis à `apply_to_3`.
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
