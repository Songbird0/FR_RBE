// `min!` va calculer le minimum entre chaque argument passé, peu importe le nombre.
macro_rules! find_min {
    // Expression de base:
    ($x:expr) => ($x);
    // `$x` suivi par au moins un `$y,`.
    ($x:expr, $($y:expr),+) => (
        // On appelle `find_min` sur les $y suivants`.
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
