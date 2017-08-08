# while let

Ayant un fonctionnement similaire à `if let`, `while let` peut alléger la syntaxe de `match` lorsqu'il n'est pas nécessaire de passer par le *pattern matching*. Voici une séquence qui incrémente `i` :

{{#playpen source/whileletsource0.rs}}

En utilisant `while let`, cela rend la séquence plus lisible :

{{#playpen source/whileletsource1.rs}}

## Voir aussi

[Les énumérations][enums], [l'énumération `Option`][option] et [la RFC de while let][rfc].

[enums]: ../chapitre3/enum.html
[option]: ../chapitre17/enumoption.html
[rfc]: https://github.com/rust-lang/rfcs/pull/214
