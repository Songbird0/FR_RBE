// Nous utiliserons `String` en tant que type d'erreur.
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // On convertit l'`Option` en un `Result` s'il y a une valeur.
       // Autrement, on fournit une instance `Err` contenant la `String`.
       .ok_or("Please use a vector with at least one element.".to_owned())
       .and_then(|s| s.parse::<i32>()
                      // On convertit n'importe quelle erreur, générée par `parse`,
                      // en `String`.
                      .map_err(|e| e.to_string())
                      // `Result<T, String>` est le nouveau type de valeur,
                      // et nous pouvons doubler le nombre se trouvant dans le conteneur.
                      .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}
