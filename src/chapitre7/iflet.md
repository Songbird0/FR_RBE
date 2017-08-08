# if let

Pour certains cas, `match` peut être « lourd ». Par exemple :

{{#playpen source/ifletsource0.rs}}

`if let` est plus adapté à ce genre de cas et permet la création de plusieurs branches en cas d'erreur :

{{#playpen source/ifletsource1.rs}}

## Voir aussi 

[Les énumérations][enums], [l'énumération `Option`][option] et [la RFC de if let][rfc].

[enums]: ../chapitre3/enum.html
[option]: ../chapitre17/enumoption.html
[rfc]: https://github.com/rust-lang/rfcs/pull/160
