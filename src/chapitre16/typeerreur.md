# Multiples types d'erreur

Les exemples précédents ont toujours été très basiques; Les (instances de) `Result` interagissent avec d'autres (instances de) `Result` et les `Option`s interagissent avec d'autres `Option`s.

Parfois une instance d'`Option` a besoin d'interagir avec un `Result` ou encore un `Result<T, Error1>` devant interagir avec un `Result<T, Error2>`. Dans ces cas, nous souhaitons gérer nos différents types de manière à pouvoir interagir simplement avec eux.

Dans le code suivant, deux instances de la méthode `unwrap()` génèrent deux types d'erreur différents. `Vec::first` renvoie une instance de `Option`, alors que `parse::<i32>` renvoie une instance de `Result<i32, ParseIntError>`:

{{#playpen source/typeerreursource0.rs}}

En utilisant notre connaissance des combinateurs, nous pouvons réécrire ce qu'il y a au-dessus pour gérer explicitement les erreurs. Puisque deux types différents peuvent être rencontrés, nous nous devons de les convertir en un type commun tel que `String`.

Pour ce faire, nous convertissons les instances d'`Option` et de `Result` en `Result`s puis nous convertissons leurs erreurs respectives sous le même type:

{{#playpen source/typeerreursource1.rs}}

Dans la prochaine section, nous allons voir une méthode pour la gestion explicite de ces erreurs.

## Voir aussi

[`Option::ok_or`][ok_or], 
[`Result::map_err`][map_err].

[ok_or]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[map_err]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
