// On utilise `String` comme type d'erreur.
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    // On convertit l'`Option` en un `Result` s'il y a une valeur.
    // Autrement, on fournit une instance `Err` contenant la `String`.
    let first = match vec.first() {
        Some(first) => first,
        None => return Err("Please use a vector with at least one element.".to_owned()),
    };

    // On double le nombre dans le conteneur si `parse` fonctionne
    // correctement.
    // Sinon, on convertit n'importe quelle erreur, générée par `parse`,
    // en `String`.
    match first.parse::<i32>() {
        Ok(i) => Ok(2 * i),
        Err(e) => Err(e.to_string()),
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}
