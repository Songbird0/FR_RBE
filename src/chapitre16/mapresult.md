# La méthode `map` pour `Result`

Nous avons noté dans l'exemple précédent que le message d'erreur affiché lorsque le programme "panique" ne nous était pas d'une grande aide. Pour éviter cela, nous devons être plus précis concernant le type de renvoi. Ici, l'élément est de type `i32`.
Pour déterminer le type de `Err`, jetons un oeil à la documentation de la méthode [`parse()`](https://doc.rust-lang.org/std/primitive.str.html#method.parse), qui est implémentée avec le trait [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) pour le type [`i32`](https://doc.rust-lang.org/std/primitive.i32.html). Dans un résultat, le type de `Err` est [`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html).

Dans l'exemple ci-dessous, utiliser directement `match` mène à coder quelque chose de relativement lourd. Heureusement, la méthode `map` de `Option` est l'une de ces méthodes que de nombreux combinateurs ont également implémenté pour `Result`. [enum.Result](https://doc.rust-lang.org/std/result/enum.Result.html) en tient une liste complète.

{{#playpen source/mapresultsource0.rs}}
