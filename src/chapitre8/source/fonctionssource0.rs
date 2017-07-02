// Contrairement à C ou C++, l'ordre de déclaration des fonctions 
// n'est pas important.
fn main() {
    // Nous pouvons appeler cette fonction ici et l'implémenter ailleurs.
    fizzbuzz_to(100);
}

// Une fonction qui renvoie une valeur booléenne.
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Comportement imprévisible, renvoi prématuré.
    if rhs == 0 {
        return false;
    }

    // C'est une expression, le mot-clé `return` n'est pas nécessaire ici.
    lhs % rhs == 0
}
// Les fonctions qui n'ont pas de type de renvoi défini renvoient par défaut 
// un tuple vide `()`.
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// Quand une fonction ne possède pas de type de renvoi, le tuple vide `()` 
// peut être omis.
fn fizzbuzz_to(n: u32) /* type de renvoi omis */ {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}