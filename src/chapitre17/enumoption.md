# L'énumération `Option`

Il est parfois désirable de rattraper les erreurs provenants de différentes parties du programme plutôt que d'appeler `panic!`; pour ce faire, l'enum `Option` prend le relais.

L'enum `Option<T>` possède deux variantes:

1. `None`, pour signaler une erreur ou l'absence d'une valeur, et
2. `Some(value)`, un tuple qui enveloppe, contient une valeur de type `T`.

{{#playpen source/enumoptionsource0.rs}}
