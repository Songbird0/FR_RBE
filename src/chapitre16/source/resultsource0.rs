fn double_number(number_str: &str) -> i32 {
    // Essayons d'utiliser la méthode `unwrap` pour récupérer le nombre.
    // Va-t-elle nous mordre ?
    2 * number_str.parse::<i32>().unwrap()
}

fn main() {
    let twenty = double_number("10");
    println!("double is {}", twenty);

    let tt = double_number("t");
    println!("double is {}", tt);
}
