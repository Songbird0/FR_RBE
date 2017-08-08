// Une fonction `age` qui renvoie un entier de type `u32`.
fn age() -> u32 {
    15
}

fn main() {
    println!("Dis-moi quel type de personne es-tu.");

    match age() {
        0             => println!("Je ne suis pas encore né, je suppose."),
        // Nous aurions pu `match` 1 ... 12 directement mais il n'aurait pas 
        // été possible de connaître l'âge de l'enfant 
        // À la place, nous assignons la valeur à `n`
        // pour la séquence de 1 ... 12. L'âge peut désormais être affiché.
        n @ 1  ... 12 => println!("Je suis un enfant de {:?} ans!", n),
        n @ 13 ... 19 => println!("Je suis un adolescent de {:?} ans!", n),
        // Pas de limite. On renvoie le résultat.
        n             => println!("Je suis un adulte de {:?} ans!", n),
    }
}
