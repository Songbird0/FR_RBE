# La surcharge des opérateurs

Avec Rust, nombre d'opérateurs peuvent être surchargés via les traits. En d'autres termes, des opérateurs peuvent être utilisés pour accomplir différentes tâches en fonction des arguments passés en entrée. Cette manipulation est possible parce que les opérateurs sont des sucres syntaxes visant à masquer l'appel des méthodes liées à ces derniers. Par exemple, l'opérateur `+` dans l'expression `a + b` appelle la méthode `add` (`a.add(b)`). La méthode `add` appartient au trait `Add`, d'où l'utilisation de l'opérateur `+` par tous les types implémentant le trait.

Vous pouvez retrouver la liste des traits surchargeant des opérateurs [ici](https://doc.rust-lang.org/book/first-edition/syntax-index.html#operators-and-symbols).

{{#playpen source/opoverloadingsource0.rs}}

## Voir aussi

[Add](https://doc.rust-lang.org/core/ops/trait.Add.html), [index de la syntaxe](https://doc.rust-lang.org/book/first-edition/syntax-index.html).
