use std::num::ParseIntError;

// Maintenant que le type de renvoi a été réécrit, nous utilisons le pattern matching 
// sans la méthode `unwrap()`.
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n)  => Ok(2 * n),
        Err(e) => Err(e),
    }
}


// Tout comme avec `Option`, nous pouvons utiliser les combinateurs tels que `map()`.
// Cette fonction possède le même fonctionnement que celle ci-dessus et se lit comme suit:
// Modifie n si la valeur est valide, sinon renvoie une erreur.
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Ceci fournit toujours une réponse valable.
    let twenty = double_number("10");
    print(twenty);

    // Ce qui suit fournit désormais un message d'erreur plus intelligible.
    let tt = double_number_map("t");
    print(tt);
}
