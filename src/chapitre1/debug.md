# Debug

Tous les types qui utilisent le formatage des traits du module `std::fmt` doivent en posséder une implémentation pour être affichés.

Les implémentations ne sont fournies automatiquement que pour les types supportés par la bibliothèque standard. Les autres devront l'implémenter « manuellement ».

Pour le trait `fmt::Debug`, rien de plus simple. Tous les types peuvent hériter de son implémentation (i.e. la créer automatiquement, sans intervention de votre part). Ce n'est, en revanche, pas le cas pour le second trait : `fmt::Display`.

{{#playpen source/debugsource0.rs}}

Également, tous les types de la bibliothèque standard peuvent être automatiquement affichés avec le marqueur `{:?}` :

{{#playpen source/debugsource1.rs}}

Finalement, `fmt::Debug` permet de rendre un type personnalisé affichable en sacrifiant quelque peu « l'élégance » du résultat. Pour soigner cela, il faudra implémenter soit-même les services du traits `fmt::Display`.

## Voir aussi

[Les attributs](https://doc.rust-lang.org/reference.html#attributes), [lien interne vers l'attribut derive], [std::fmt](http://doc.rust-lang.org/std/fmt/), [les structures](../chapitre1/struct.html).
