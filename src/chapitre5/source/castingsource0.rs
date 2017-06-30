// Supprime tous les avertissements relatifs aux dépassements 
// de capacité (e.g. une variable de type u8 ne peut pas 
// contenir plus qu'une variable de type u16).
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Erreur! La conversion implicite n'est pas supportée.
    // let integer: u8 = decimal;
    // FIXME ^ Décommentez/Commentez cette ligne pour voir 
    // le message d'erreur apparaître/disparaître.

    // Conversion explicite.
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Lorsque vous convertissez une valeur vers un type 
    // non-signé T, std::T::MAX + 1 est incrémenté ou soustrait jusqu'à 
    // ce que la valeur respecte la capacité du nouveau type.

    // 1000 ne dépasse pas la capacité d'un entier non-signé codé sur 16 bits.
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // En réalité, les 8 premiers bits les plus faibles (LSB) sont conservés et les 
    // bits les plus forts (MSB) restants sont tronqués.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // Pour les nombres positifs, cette soustraction est équivalente à une 
    // division par 256.
    println!("1000 mod 256 is : {}", 1000 % 256);

    // Quand vous convertissez un type d'entiers signés, le résultat (bit à bit)
    // est équivalent à celui de la conversion vers un type d'entiers non-signés.
    // Si le bit de poids fort vaut 1, la valeur sera négative.

    // Sauf si il n'y a pas de dépassements, évidemment.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, complément à deux de 128 codé sur 8 bits:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // On répète l'exemple ci-dessus.
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // et le complément à deux de 232 est -24.
    println!(" 232 as a i8 is : {}", 232 as i8);


}