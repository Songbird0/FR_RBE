# La méthode `map` pour `Result`

Nous avons noté dans l'exemple précédent que le message d'erreur affiché lorsque le programme "panique" ne nous était pas d'une grande aide. Pour éviter cela, nous devons être plus précis concernant le type de renvoi. Ici, l'élément est de type `i32`.
Pour déterminer le type de `Err`, jetons un oeil à la documentation de la méthode [`parse()`][parse], qui est implémentée avec le trait [`FromStr`][from_str] pour le type [`i32`][i32]. Dans un résultat, le type de `Err` est [`ParseIntError`][parse_int_error].

Dans l'exemple ci-dessous, utiliser directement `match` mène à coder quelque chose de relativement lourd. Heureusement, la méthode `map` de `Option` est l'un de ces nombreux combinateurs ont également été implémentés pour `Result`. [enum.Result][enum_result] en tient une liste complète.

{{#playpen source/mapresultsource0.rs}}

[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[from_str]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[i32]: https://doc.rust-lang.org/std/primitive.i32.html
[parse_int_error]: https://doc.rust-lang.org/std/num/struct.ParseIntError.html
[enum_result]: https://doc.rust-lang.org/std/result/enum.Result.html
