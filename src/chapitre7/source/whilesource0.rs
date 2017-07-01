fn main() {
    // Un compteur.
    let mut n = 1;

    // Itère sur `n` tant que sa valeur est strictement inférieure 
    // à 101.
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Incrémente le compteur.
        n += 1;
    }
}