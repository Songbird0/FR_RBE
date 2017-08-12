# Les combinateurs: `map`

`match` est une approche valable pour gérer les `Option`s. Cependant, vous pourriez trouver son utilisation fastidieuse, principalement avec des opérations qui ne sont valides qu'avec une entrée. Dans ces cas, les combinateurs peuvent être utilisés pour gérer le contrôle du flux de manière plus modulaire.

`Option` possède une méthode préfaite appelée `map()`, un combinateur pour la simple mise en corrélation de `Some -> Some` et `None -> None`. Les appels de la méthode `map()` peuvent être chaînés ensemble pour plus de flexibilité.

Dans l'exemple suivant, `process()` remplace toutes les fonctions précédentes tout en restant compacte.

{{#playpen source/mapsource0.rs}}
