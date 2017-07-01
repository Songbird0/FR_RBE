fn main() {
    let n = 5;

    if n < 0 {
        print!("{} est négatif.", n);
    } else if n > 0 {
        print!("{} est positif.", n);
    } else {
        print!("{} est nul.", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(" et est un petit nombre, multiplions-le par dix");

            // Cette expression renvoie un entier de type `i32`.
            10 * n
        } else {
            println!(" est un grand nombre, divisons-le par deux");

            // Cette expression doit également renvoyer un entier de type `i32`.
            n / 2
            // TODO ^ Essayez de supprimer cette expression en ajoutant un point-virgule.
        };
    //   ^ Ne pas oubliez de mettre un point-virgule ici! Toutes les 
    // assignations (`let`) doivent se terminer par un point-virgule.

    println!("{} -> {}", n, big_n);
}