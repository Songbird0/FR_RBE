// Un attribut qui masque les avertissements du compilateur
// concernant le code mort.
#![allow(dead_code)]

// Énumération avec un identifiant implicite (partant de 0).
enum Number {
    Zero, // 0
    One, // 1 
    Two, // 2
}

// Énumération avec un identifiant explicite.
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // Les variantes d'une énumération peuvent être converties en entiers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}