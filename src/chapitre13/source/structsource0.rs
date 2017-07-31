// On créé un type nommé `Borrowed` qui a pour attribut 
// une référence d'un entier codé sur 32 bits. La référence 
// doit survivre à l'instance de la structure `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Même combat, ces deux références doivent survivre à l'instance
// (ou les instances)  de la structure `NamedBorrowed`.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// On créé une énumération qui contient deux variantes:
// 1. Un tuple qui prend en entrée un entier codé sur 32 bits;
// 2. Un tuple qui prend en entrée une référence d'un `i32`.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
