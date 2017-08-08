fn main() {
    let number = 13;
    // TODO ^ Assignez différentes valeurs à `number`.

    println!("Nature de number {}", number);
    match number {
        // Teste une seule valeur.
        1 => println!("Un!"),
        // Teste plusieurs valeurs.
        2 | 3 | 5 | 7 | 11 => println!("C'est un nombre premier."),
        // Teste l'intervalle [13;19].
        13...19 => println!("A teen"),
        // Couvre tous les autres cas.
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // L'analyse est également une expression.
    let binary = match boolean {
        // Les branches du match doivent couvrir tous les cas possibles.
        false => 0,
        true => 1,
        // TODO ^ Essayez de commenter l'une de ses branches.
    };

    println!("{} -> {}", boolean, binary);
}
