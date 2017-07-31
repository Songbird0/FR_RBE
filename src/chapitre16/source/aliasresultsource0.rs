use std::num::ParseIntError;

// On définit un alias générique pour un type de `Result` avec le type d'erreur 
// `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// On utilise l'alias ci-dessus pour faire référence à notre 
// `Result`.
fn double_number(number_str: &str) -> AliasedResult<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// Ici, l'alias nous permet encore d'épargner de l'espace.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}
