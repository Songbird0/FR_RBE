// Une division entre deux entiers qui ne plante pas.
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // L'échec est représenté par la variante `None`.
        None
    } else {
        // Le résultat est enveloppé dans une instance `Some`.
        Some(dividend / divisor)
    }
}

// Cette fonction gère une division qui peut ne pas fonctionner.
fn try_division(dividend: i32, divisor: i32) {
    // Les valeurs d'`Option` peuvent être matchées, tout comme les autres enums.
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // L'assignation de `None` à une variable nécessite de typer cette dernière.
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // "dé-wrapper" une instance de `Some` va extraire la valeur contenue.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Tenter d'utiliser `unwrap` sur la variante `None` fera planter le programme.
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
