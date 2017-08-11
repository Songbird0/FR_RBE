# Les alias de `Result`

Quid lorsque nous souhaitons réutiliser un type de `Result` bien précis ? Rappelez-vous que Rust nous permet de créer des [alias][alias]. Nous pouvons alors aisément en définir un pour le type de `Result` en question.

À l'échelle d'un module, la création d'alias peut être salvatrice. Les erreurs pouvant être trouvées dans un module pécis ont souvent le même type (wrappé par `Err`), donc un seul alias peut définir *l'intégralité* des `Result`s associés. C'est tellement utile que la bibliothèque standard en fournit un: `io::Result` !

Voici un petit exemple pour présenter la syntaxe:

{{#playpen source/aliasresultsource0.rs}}

## Voir aussi

[`Result`][result] et [`io::Result`][io_result].

[alias]: ../chapitre5/alias.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: https://doc.rust-lang.org/std/io/type.Result.html
