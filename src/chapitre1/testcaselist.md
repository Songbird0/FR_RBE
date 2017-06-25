# Exemple d'utilisation : La structure List

Implémenter le `trait` `fmt::Display` pour une structure où les éléments doivent être gérés séquentiellement est assez délicat. Le problème réside dans le fait que chaque appel de la macro `write!` génère une instance de `fmt::Result`. Une bonne gestion de ces appels demande de tester chaque résultat. Rust vous permet de gérer les erreurs de deux manières :

1. En utilisant la macro `try!`;
2. En utilisant l'opérateur `?` (qui est l'équivalent de `try!` mais intégré directement au langage).

La macro `try!` vient envelopper la fonction (ou la macro) cible comme ceci :

```text
// On ‘test' write! Pour voir si une erreur survient. Si il y a une erreur, 
// elle sera renvoyée. Sinon, l'exécution continue. 
try!(write!(f, "{}", value));
```

L'opérateur `?`, bien qu'équivalent à la macro `try!`, vient se positionner devant l'appel de la fonction (ou macro).


```text
write!(f, "{}", value)?;
```

Avec l'opérateur `?`, l'implémentation du trait `fmt::Display` pour un `Vec` est simple et lisible.

```rust
use std::fmt; // On importe le module `fmt`.


// On déclare une structure nommée 'List' qui contient un 'Vec'.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // On extrait le premier champ de la structure. 
        // Nous créons une référence de 'vec'.
        let vec = &self.0;

        write!(f, "[")?;

        // On parcours 'vec' en stockant chacun de ses éléments et 
        // le nombre d'iterations.
        for (count, v) in vec.iter().enumerate() {
            // Pour tout élément, excepté le premier, on ajoute une virgule.
            // On utilise l'opérateur ?, ou la macro try!, pour renvoyer les erreurs.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // On ferme le crochet ouvert précédemment et on renvoie une instance 
        // de la structure fmt::Result.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

### Activité

Essayez de modifier le programme pour que l'index de chaque élément du vector soit également affiché durant l'exécution. Le résultat devrait ressembler à ceci :

```text
[0: 1, 1: 2, 2: 3]
```

### Voir aussi

[La boucle for](../chapitre7/forintervalles.html), [le pattern `ref`](../chapitre13/refpattern.html), [`Result`](../chapitre17/enumresult.html), [les structures](../chapitre3/struct.html), [`try!`](../chapitre17/trymacro.html), [`vec!`](../chapitre17/vecteurs.html)