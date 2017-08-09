# Les fonctions d'ordre supérieur

Rust supporte les [fonctions d'ordre supérieur][hof] (HOF). Ces fonctions prennent en paramètre une ou plusieurs fonctions et renvoie une autre fonction. Ce sont les HOF ainsi que les « [itérateurs laxistes][lazy] » qui donnent cet aspect fonctionnel à Rust.

{{#playpen source/hofsource0.rs}}

L'énumération [Option][option] et le trait [Iterator][iterator] implémentent leur lot d'HOF.

[hof]: https://fr.wikipedia.org/wiki/Fonction_d'ordre_supérieur
[lazy]: https://stackoverflow.com/questions/2155788/most-of-the-iterators-and-iterables-methods-are-lazy-what-does-this-mean/2155801#2155801
[option]: http://doc.rust-lang.org/core/option/enum.Option.html
[iterator]: http://doc.rust-lang.org/core/iter/trait.Iterator.html
