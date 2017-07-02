# Les fonctions d'ordre supérieur

Rust supporte les [fonctions d'ordre supérieur](https://fr.wikipedia.org/wiki/Fonction_d'ordre_supérieur) (HOF). Ces fonctions prennent en paramètre une ou plusieurs fonctions et renvoie une autre fonction. Ce sont les HOF ainsi que les « [itérateurs fainéants](http://stackoverflow.com/questions/2155788/most-of-the-iterators-and-iterables-methods-are-lazy-what-does-this-mean) » qui donnent cet aspect fonctionnel à Rust.

{{#playpen source/hofsource0.rs}}

L'énumération [Option](http://doc.rust-lang.org/core/option/enum.Option.html) et le trait [Iterator](http://doc.rust-lang.org/core/iter/trait.Iterator.html) implémentent leur lot d'HOF.
