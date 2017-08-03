# Documentation

Les commentaires de documentation sont très utiles pour d'importants projets nécessitant une documentation. Lorsque vous lancez [Rustdoc][rd], ce sont ces commentaires qui seront compilés dans la documentation. Ils sont préfixés par la séquence `///` et supportent [Markdown][md].

{{#playpen source/docsource0.rs}}

> **Note**: Si vous souhaitez compiler ce programme vous-même, n'oubliez pas de décommenter la ligne `// #![crate_name = "doc"]`.

Pour lancer les tests, vous devez tout d'abord compiler le code en mode bibliothèque puis renseigner la position de la bibliothèque à rustdoc pour qu'il puisse la lier à chaque programme de la documentation:

```bash
rustc doc.rs --crate-type lib
rustdoc --test --extern doc="libdoc.rs"
```

Lorsque vous exécutez la commande `cargo test` sur une crate, Cargo générera et exécutera automatiquement les commandes respectives de rustc et rustdoc.

[rd]: https://doc.rust-lang.org/book/first-edition/documentation.html#about-rustdoc
[md]:  https://en.wikipedia.org/wiki/Markdown
