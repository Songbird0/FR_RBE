fn main() {
    let mut count = 0u32;

    println!("Comptons jusqu'à l'infini!");

    // Boucle infinie.
    loop {
        count += 1;

        if count == 3 {
            println!("trois");

            // Ignore le reste de l'itération.
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, ça suffit!");

            // Sort de la boucle.
            break;
        }
    }
}