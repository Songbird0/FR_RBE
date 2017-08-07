# Debug

Tous les types qui utilisent le formatage des traits du module `std::fmt` doivent en posséder une implémentation pour être affichés.

Les implémentations ne sont fournies automatiquement que pour les types supportés par la bibliothèque standard. Les autres devront l'implémenter « manuellement ».

Pour le trait `fmt::Debug`, rien de plus simple. Tous les types peuvent hériter de son implémentation (i.e. la créer automatiquement, sans intervention de votre part). Ce n'est, en revanche, pas le cas pour le second trait : `fmt::Display`.

```rust,ignore
// Cette structure ne peut être affichée par `fmt::Debug`, 
// ni par `fmt::Display`.
struct UnPrintable(i32);

// L'attribut `derive` créé automatiquement l'implémentation requise 
// pour permettre à cette structure d'être affichée avec `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);
```

Également, tous les types de la bibliothèque standard peuvent être automatiquement affichés avec le marqueur `{:?}` :

{{#playpen source/debugsource1.rs}}

Finalement, `fmt::Debug` permet de rendre un type personnalisé affichable en sacrifiant quelque peu « l'élégance » du résultat. Pour soigner cela, il faudra implémenter soit-même les services du traits `fmt::Display`.

## Voir aussi

[Les attributs][attributes],  [derive][derive], [std::fmt][fmt], [les structures][struct].

[attributes]: https://doc.rust-lang.org/reference.html#attributes
[derive]: ../chapitre14/derive.html
[fmt]: http://doc.rust-lang.org/std/fmt/
[struct]: ../chapitre1/struct.html
