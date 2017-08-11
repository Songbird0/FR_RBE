# L'enum `Result`

[Result][result] est une version plus élaborée de l'enum `Option` qui conçoit une potentielle erreur plutôt qu'une potentielle *absence*.

Autrement dit, `Result<T, E>` pourrait adopter l'un de ces deux états:

* `Ok<T>`: Un élément `T` a été trouvé;
* `Err<E>`: Une erreur a été trouvée avec l'élément `E`.

Par convention, la valeur de retour attendue est `Ok` jusqu'à la preuve du contraire (i.e. qu'une erreur (`Err`) est survenue).

Tout comme `Option`, `Result` possède de nombreuses méthodes associées à elle. `unwrap()`, par exemple, fournit l'élément `T` sinon déclenche `panic`. Pour la gestion des cas, `Result` possède nombre de combinateurs en commun avec `Option`.

En travaillant avec Rust, vous rencontrerez probablement des méthodes renvoyant le type `Result`, telles que la méthode [`parse()`][parse]. Il n'est pas toujours possible de convertir une chaîne de caractères dans un autre type, donc `parse()` renvoie une instance de `Result` indiquant les potentielles erreurs.

Voyons ce qu'il se passe lorsque nous parvenons à convertir une chaîne de caractères et lorsque ce n'est pas le cas:

{{#playpen source/resultsource0.rs}}

Dans le cas où nous ne parvenons pas à la convertir, `parse()` nous laisse avec l'erreur sur laquelle `unwrap` a paniqué. Vous noterez également que le message d'erreur affiché par `panic` est assez désagréable.

Pour améliorer la qualité de notre message d'erreur, nous devrions être plus rigoureux quant à la valeur de retour et envisager de gérer explicitement l'erreur.

[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
