// Un trait qui implémente le marqueur `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// Le type générique `T` doit implémenter le trait `Debug`.
// Qu'importe le type de `T`, cela fonctionnera.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` doit implémenter le trait `HasArea`. N'importe quelle 
// structure remplissant les conditions d'entrée peut accéder 
// à la méthode `area` du trait `HasArea`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // ^ TODO: Essayez de décommenter ces lignes.
    // | Erreur: N'implémente pas l'un de ces traits: `Debug` ou `HasArea`.
}
