# Le trait Drop

Le trait [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) ne possède qu'une seule méthode: `drop`; cette dernière est automatiquement appelée lorsqu'un objet sort du contexte. La fonction principale du trait `Drop` est de libérer les ressources que les instances, du type ayant implémenté le trait, possèdent.

`Box`, `Vec`, `String`, `File` et `Process` sont autant d'exemples de types qui implémentent le trait `Drop` pour libérer leurs ressources. Le trait `Drop` peut également être implémenté manuellement pour répondre aux besoins de vos propres types.

Dans l'exemple suivant, nous affichons quelque chose dans la console à partir de la méthode `drop` pour notifier chaque appel.

{{#playpen source/traitdropsource0.rs}}
