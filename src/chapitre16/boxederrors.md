# `Box`ing des erreurs

En implémentant `Display` et `Form` pour notre type d'erreur, nous avons usé de *presque* tous les outils dédiés à la gestion d'erreur de la bibliothèque standard. Nous avons cependant oublié quelque chose: la capacité à simplement `Box` notre type.

La bibliothèque standard convertit n'importe quel type qui implémente le trait `Error` et sera pris en charge par le type `Box<Error>`, via `From`. Pour l'utilisateur d'une bibliothèque, ceci permet aisément une manoeuvre de ce genre:

```rust,ignore
fn foo(...) -> Result<T, Box<Error>> { ... }
```

Un utilisateur peut utiliser nombre de bibliothèques externes, chacune fournissant leurs propres types d'erreur. Pour définir un type de `Result<T, E>` valide, l'utilisateur a plusieurs options:

* Définir un nouveau wrapper englobant les types d'erreur de la bibliothèque;
* Convertir les types d'erreur en `String` ou vers un autre type intermédiaire;
* `Box` les types dans `Box<Error>`.

Le "boxing" du type d'erreur est un choix plutôt habituel. Le problème est que le type de l'erreur sous-jacente est connu à l'exécution et n'est pas [déterminé statiquement][static_dispatch]. Comme mentionné plus haut, tout ce qu'il y a à faire c'est d'implémenter le trait `Error`:

```rust,ignore
trait Error: Debug + Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}
```

Avec cette implémentation, jetons un oeil à notre exemple récemment présenté. Notez qu'il est tout aussi fonctionnel avec le type `Box<Error>` qu'avec `DoubleError`:

{{#playpen source/boxederrorssource0.rs}}

## Voir aussi

[Distribution dynamique][static_dispatch] et 
[le trait `Error`][error_trait].

[static_dispatch]: https://doc.rust-lang.org/book/trait-objects.html#dynamic-dispatch
[error_trait]: https://doc.rust-lang.org/std/error/trait.Error.html
