# Les combinateurs: `and_then`

`map()` a été décrite comme étant un moyen de chaîner les directives pour simplifier les déclarations `match`. Cependant, utiliser `map()` sur une fonction qui renvoie déjà une instance de `Option<T>` risque d'imbriquer le résultat dans une autre instance `Option<Option<T>>`. Chaîner des appels peut alors prêter à confusion. C'est là où un autre combinateur nommé `and_then()`, connu dans d'autres langages sous le nom de flatmap, entre en jeu.

`and_then()` appelle la fonction passée en entrée avec la valeur imbriquée et renvoie le résultat. Si le conteneur `Option` vaut `None`, alors elle renvoie `None` à la place.

Dans l'exemple suivant, `cookable_v2()` renvoie une instance de `Option<Food>`. Utiliser la méthode `map()` au lieu de `and_then()` donnerait une instance imbriquée `Option<Option<Food>>`, qui est un type invalide pour la fonction `eat()`.

{{#playpen source/andthensource0.rs}}

## Voir aussi

[Les closures][closures], [Option][option] et [Option::and_then()][andthen].

[closures]: ../chapitre8/closures.html
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[andthen]: https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
