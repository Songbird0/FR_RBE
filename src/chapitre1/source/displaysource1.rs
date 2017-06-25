use std::fmt; // On importe le module `fmt`

// Une structure qui contient deux nombres. `Debug` va être hérité pour que les résultats
// puissent être comparés avec `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implémentation du trait `Display` pour la structure `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // On utilise `self.nombre` pour faire référence à la donnée se trouvant
        // à cette position.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Définissons une structure où les champs sont nommés pour comparer.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// On implémente également le trait fmt::Display pour la structure Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // On désigne les champs de notre choix. (en l'occurrence `x` et `y`)
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Comparaison des structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("Le grand intervalle est {big} et le petit est {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Comparaison des points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    // Erreur. Les traits `Debug` and `Display` étaient implémentés mais 
    // le marqueur `{:b}` requiert l'implémentation du trait `fmt::Binary`.
    // Cela ne fonctionnera pas.
    // println!("A quoi ressemble Point2D formaté en binaire: {:b} ?", point);
}