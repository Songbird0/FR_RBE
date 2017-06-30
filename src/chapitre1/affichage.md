# Affichage formaté

L'affichage est pris en charge par une série de macros déclarées dans le module `std::fmt` qui inclut :


* `format!`: Construit la chaîne de caractères du texte à afficher;
* `print!`: Fait exactement la même chose que `format!`, mais le texte est affiché dans la console;
* `println!`: Fait exactement la même chose que `print!`, mais un retour à la ligne est ajouté.

Toutes formatent le texte de la même manière.

**Note**: La validité du formatage (i.e. Si la chaîne de caractères que vous soumettez peut être formatée comme vous le désirez) est vérifiée au moment de la compilation.

{{#playpen source/affichagesource0.rs}}

`std::fmt` contient plusieurs traits qui structurent l'affichage du texte. Les deux plus « importants » sont listés ci-dessous :


1. [fmt::Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) : Utilise le marqueur `{:?}`. Applique un formatage dédié au débogage.
2. [fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) : Utilise le marqueur `{}`. Formate le texte de manière plus élégante, plus « user friendly ».

Dans cet exemple, `fmt::Display` était utilisé parce que la bibliothèque standard fournit les implémentations pour ces types. Pour afficher du texte à partir de types complexes/personnalisés, d'autres étapes sont requises.

## Activité

Réglez les deux problèmes dans le code ci-dessus (cf. FIXME) pour qu'il s'exécute sans erreurs.

Ajoutez une macro `println!` qui affiche : « Pi est, à peu près, égal à 3,142 » en contrôlant le nombre affiché de chiffres après la virgule. Dans le cadre de l'exercice, vous utiliserez `let pi = 3.141592` comme estimation de Pi (**Note **:Vous pourriez avoir besoin de consulter la documentation du module [std::fmt](https://doc.rust-lang.org/std/fmt/index.html "Documentation du module") pour configurer le nombre de décimaux à afficher).

## Voir aussi

[std::fmt](https://doc.rust-lang.org/std/fmt/index.html), [lien interne vers les macros], [les structures](../chapitre3/struct.html), [lien interne vers les traits]
