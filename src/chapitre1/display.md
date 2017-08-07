# Display

`fmt::Debug` propose un formatage rudimentaire, et il peut être de bon ton de soigner ce que nous affichons. Pour ce faire, il faudra implémenter `fmt::Display` (qui utilise le marqueur `{}`).

Voici un exemple d'implémentation du trait :

```rust,ignore
// On importe (via `use`) le module `fmt` pour le rendre accessible.
use std::fmt;

// Nous définissons une structure dans laquelle le trait `fmt::Display` 
// sera implémenté. Ce n'est qu'un simple tuple, nommée `Structure`, contenant un entier de type i32. 
struct Structure(i32);

// Pour pouvoir utiliser le marqueur `{}`, le trait `fmt::Display` doit être implémenté 
// manuellement pour le type.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // écrit le premier élément de la structure dans le flux en sortie
        // soumis: `f`. Renvoie une instance de `fmt::Result` qui témoigne du succès 
        // ou de l'échec de l'opération. Notez que `write!` possède une syntaxe très 
        // similaire à `println!`.
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display` pourrait être plus lisible que `fmt::Debug` mais il présente un problème pour la bibliothèque standard. Comment les types ambiguës devraient être affichés ? Par exemple, si la bibliothèque standard devait implémenter un seul formatage pour toutes les « variantes » de `Vec<T>`, quel style devrait être choisi ? N'importe lequel ?

1. `Vec<Path> : /:/etc:/home/username:/bin` (séparé par des « : »);
2. `Vec<i32> : 1,2,3` (séparé par des « , »).

Bien sûr que non, puisqu'il n'y a pas de mise en forme idéale pour tous les types et la bibliothèque standard n'en impose pas.

`fmt::Display` n'est pas implémenté pour la structure `Vec<T>` ni pour aucun autre conteneur générique. `fmt::Debug`  doit alors être utilisé pour ces ressources.

Ce n'est en revanche pas un problème pour les conteneurs(e.g. structures) qui ne sont pas génériques, `fmt::Display` peut être implémenté et utilisé.

{{#playpen source/displaysource1.rs}}

Donc `fmt::Display` a été implémenté mais ce n'est pas le cas de `fmt::Binary`, il ne peut alors pas être utilisé.

`std::fmt` possède de nombreux [traits][traits] et chacun doit posséder sa propre implémentation. Pour plus d'informations, nous vous invitons à consulter [la documentation du module][fmt].

## Activité

Après avoir constaté le résultat de l'exemple ci-dessus, aidez-vous de la structure `Point2D` pour ajouter à l'exemple une nouvelle structure nommée `Complex`. Voici le résultat attendu lorsqu'une instance de la structure `Complex` sera affichée :

```text
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

## Voir aussi

[L'attribut derive][derive], [std::fmt][fmt], [les macros][macros], [les structures][struct], [les traits][traits], [le mot-clé use][use].

[derive]: ../chapitre14/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[macros]: ../chapitre15/systememacro.html
[struct]: ../chapitre3/struct.html
[traits]: ../chapitre14/traits.html
[use]: ../chapitre9/usedeclaration.html
