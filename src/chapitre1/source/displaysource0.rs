// On importe (via `use`) le module `fmt` pour le rendre accessible.
use std::fmt;

// Nous définissons une structure dans laquelle le trait `fmt::Display` 
// sera implémenté. Ce n'est qu'une simple tuple, nommée `Structure`, contenant un entier de type i32. 
struct Structure(i32);

// Pour pouvoir utiliser le marqueur `{}`, le trait `fmt::Display` doit être implémenté 
// manuellement pour le type.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // écrit le premier élément de la structure dans le flux de sortie
        // soumis: `f`. Renvoie une instance de `fmt::Result` qui témoigne du succès 
        // ou de l'échec de l'opération. Notez que `write!` possède une syntaxe très 
        // similaire à `println!`.
        write!(f, "{}", self.0)
    }
}