# Les itérateurs

Le trait `Iterator` est utilisé pour implémenter les itérateurs sur les collections tels que les tableaux.

Le trait nécessite seulement la définition d'une méthode pour l'élément `next`. Elle peut être implémentée manuellement en utilisant un bloc `impl` ou automatiquement (comme pour les tableaux et intervalles).

Pour les utilisations les plus communes, la construction `for` peut convertir certaines collections en itérateurs en utilisant la méthode [`.into_iterator()`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).

Les méthodes pouvant être appelées en utilisant le trait `Iterator`, en plus de celles présentées dans l'exemple ci-dessous, sont listées [ici](https://doc.rust-lang.org/core/iter/trait.Iterator.html).

{{#playpen source/iteratorssource0.rs}}
