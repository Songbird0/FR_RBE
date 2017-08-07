use std::fmt; // On importe le module `fmt`.


// On déclare une structure nommée 'List' qui contient un 'Vec'.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // On extrait le premier champ de la structure. 
        // Nous créons une référence de 'vec'.
        let vec = &self.0;

        write!(f, "[")?;

        // On parcourt 'vec' en stockant chacun de ses éléments et 
        // le nombre d'itérations.
        for (count, v) in vec.iter().enumerate() {
            // Pour tout élément, excepté le premier, on ajoute une virgule.
            // On utilise l'opérateur ?, ou la macro try!, pour renvoyer les erreurs.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // On ferme le crochet ouvert précédemment et on renvoie une instance 
        // de la structure `fmt::Result`.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
