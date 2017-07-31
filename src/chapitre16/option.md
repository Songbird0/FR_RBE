# L'enum `Option` et la méthode `unwrap`

Dans la dernier exemple, nous avons montré qu'il était possible de mettre en échec le programme quand bon nous semble. Nous disions que notre programme pouvait "paniquer" si la princesse recevait un présent inapproprié - un serpent. Mais qu'en est-il du cas où la princesse attendait un cadeau mais n'en reçoit pas ? Ce cas figure serait tout bonnement irrecevable, donc inutile d'être géré.

Nous pourrions tester ce cas contre une chaîne de caractères vide (`""`), comme nous l'avons fait avec le serpent. Puisque nous utilisons Rust, laissons plutôt le compilateur gérer les cas où il n'y pas de cadeau.

Une `enum` nommée `Option<T>` dans la bibliothèque standard est utilisée lorsque "l'absence de" est une possibilité. Elle-même est représentée par deux "options":

* `Some(T)`: Un élément de type `T` a été trouvé;
* `None`: Aucun élément n'a été trouvé.

Ces cas peuvent être explicitement gérés par le biais de `match` ou implicitement avec `unwrap`. La gestion implicite renverra l'élément contenu en cas de succès, sinon un `panic` sera lancé.

Notez que si il est possible de personnaliser manuellement le message d'erreur de `panic`, ce n'est pas le cas pour `unwrap` qui nous laissera avec des informations moins intelligibles qu'une gestion explicite. Dans l'exemple suivant, la gestion explicite offre un plus grand contrôle sur le résultat tout en permettant l'utilisation de `panic`, si désiré.

{{#playpen source/optionsource0.rs}}
