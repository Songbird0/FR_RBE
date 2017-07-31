use std::error;
use std::fmt;
use std::num::ParseIntError;

// On modifie l'alias pour ajouter `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            // Courte description de l'erreur.
            // Vous n'êtes pas obligé de renseigner la même description que 
            // pour `Display`.
            DoubleError::EmptyVec => "empty vectors not allowed",
            // Ceci implémente déjà `Error`, on se reporte à sa propose implémentation.
            DoubleError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // Pas de cause (i.e. pas d'autre erreur) 
            // sous-jacente au déclenchement de cette erreur, donc on renvoie `None`.
            DoubleError::EmptyVec => None,
            // La cause est l'implémentation sous-jacente du type d'erreur. 
            // Il (le type) est implicitement casté en `&error::Error`. 
            // Ca fonctionne parce que le type en question a déjà implémenté le trait `Error`.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
