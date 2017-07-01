fn main() {
    let pair = (0, -2);
    // TODO ^ Essayez de modifier les valeurs contenues par 
    // le tuple.

    println!("Dites m'en plus à propos de {:?}", pair);
    // match peut être utilisé pour déstructurer un tuple.
    match pair {
        // Déstructure `y`.
        (0, y) => println!("Le premier élément est égal à `0` 
        et `y` égal à `{:?}`", y),
        (x, 0) => println!("`x` est égal à `{:?}` et le dernier est égal à `0`", x),
        _      => println!("Peu importe ce qu'ils sont."),
        // L'underscore `_` siginifie que vous ne souhaitez pas 
        // assigner de valeurs à une variable, que vous souhaitez couvrir tous les 
        // autres cas.
    }
}