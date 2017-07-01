// On fait taire les avertissements (puisqu'on utilise 
// qu'une seule variante).
#[allow(dead_code)]
enum Color {
    // Identification implicite.
    Red,
    Blue,
    Green,
    // Ces variantes assignent plusieurs tuples sous différents noms: les modèles 
    // de couleur.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Essayez de modifier les valeurs du tuple.
    println!("De quelle couleur s'agit-il?");
    // Une énumération peut être déstructurée en utilisant le pattern matching.
    match color {
        Color::Red   => println!("La couleur rouge!"),
        Color::Blue  => println!("La couleur bleu!"),
        Color::Green => println!("La couleur vert!"),
        Color::RGB(r, g, b) =>
            println!("Rouge: {}, Vert: {}, et Bleu: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Teinte: {}, Saturation: {}, Valeur: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Teinte: {}, Saturation: {}, Lumière: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, Magenta: {}, Jaune: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, Magenta: {}, Jaune: {}, Noir: {}!",
                c, m, y, k),
        // Inutile d'ajouter une branche "par défaut" car tous les 
        // cas ont été couverts.
    }
}