use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// Le trait `std::ops::Add` est utilisé pour permettre la surcharge de `+`.
// Ici, nous spécifions `Add<Bar>` - cette implémentation sera appelée si l'opérande de droite est 
// de type `Bar`.
// Le bloc ci-dessous implémente l'opération: `Foo + Bar = FooBar`.
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// En inversant les types, nous nous retrouvons à implémenter une addition non-commutative.
// Ici, nous spécifions `Add<Foo>` - cette implémentation sera appelée si l'opérande de droite 
// est de type `Foo`.
// Le bloc ci-dessous implémente l'opération: `Bar + Foo = BarFoo`.
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
