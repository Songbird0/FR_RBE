# La macro `try!`

Chaîner les résultats en utilisant match peut être très chaotique; heureusement, la macro `try!` peut être utilisée pour soigner l'écriture. La macro `try!` étend une expression de match où la branche `Err(err)` étend un retour prématuré (`return Err(err)`) et la branche `Ok(ok)` étend une expression `ok` et fournit la ressource.

{{#playpen source/trymacrosource0.rs}}

N'hésitez pas à consulter la [documentation](https://doc.rust-lang.org/std/result/index.html "Documentation officielle du module result"), de nombreuses méthodes sont disponibles pour créer et gérer les `Result`.
