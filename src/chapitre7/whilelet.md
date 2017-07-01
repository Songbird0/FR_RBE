# while let

Ayant un fonctionnement similaire à `if let`, `while let` peut alléger la syntaxe de `match` lorsqu'il n'est pas nécessaire de passer par le *pattern matching*. Voici une séquence qui incrémente `i` :

{{#playpen source/whileletsource0.rs}}

En utilisant `while let`, cela rend la séquence plus lisible :

{{#playpen source/whileletsource1.rs}}

## Voir aussi

[Les énumérations](../chapitre3/enum.html), [l'énumération `Option`](../chapitre17/enumoption.html) et [la RFC de while let](https://github.com/rust-lang/rfcs/pull/214).
