# Introduction à `try!`

Parfois nous voulons la simplicité de `unwrap()` sans la possibilité de faire planter le programme. Jusqu'ici, `unwrap()` nous a dévié de ce que nous voulions vraiment: *récupérer la variable*. C'est exactement le but de `try!` que de régler ce souci.

Une fois l'instance `Err` trouvée, il y a deux actions possibles:

1. `panic!` que nous avons choisi d'éviter, si possible, avec `try!`;
2. `return` parce qu'une `Err` ne peut être traitée.

La macro `try!` est *presque*<sup>[1](#note0)</sup> équivalent à la méthode `unwrap()` mais effectue un renvoi prématuré au lieu de planter lorsqu'un conteneur `Err` est récupéré. Voyons comment nous pouvons simplifier l'exemple précédent qui utilisait les combinateurs: 

{{#playpen source/introtrysource0.rs}}

Notez que, jusqu'ici, nous avons utilisé les `String`s pour les erreurs. Cependant, elles sont quelque peu limitées en tant que type d'erreur. Dans la prochaine section, nous apprendrons à créer des erreurs plus structurées, plus riches, en définissant leur propre type.

<blockquote id="note0">
	<sup>1</sup>. Consultez <a href="../chapitre16/othersuses.html">cette section</a> pour plus de détails.
</blockquote>

## Voir aussi

[`Result`][result] et 
[`io::Result`][io_result].

[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[io_result]: https://doc.rust-lang.org/std/io/type.Result.html
