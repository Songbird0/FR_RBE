# L'énumération `Result`

Nous avons vu que l'enum `Option` peut être utilisée en tant que valeur de retour depuis les fonctions pouvant échouer, où `None` peut être renvoyé pour indiquer un échec. Il est parfois important d'expliquer *pourquoi* une opération a échoué. Pour ce faire, nous pouvons utiliser `Result`.

`Result<T, E>` possède deux variantes:

1. `Ok(valeur)` qui signale que l'opération s'est correctement déroulée et enveloppe la `valeur` renvoyée par l'opération (`valeur` est de type `T`);
2. `Err(pourquoi)` qui signale que l'opération a échoué et enveloppe le `pourquoi`, qui (espérons-le) nous renseigne sur la cause de l'échec (`pourquoi` est de type `E`).

{{#playpen source/enumresultsource0.rs}}
