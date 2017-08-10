# Les fonctions

Les règles précédemment présentées s'appliquent également aux fonctions : un type `T` est générique lorsqu'il est précédé par la déclaration `<T>` (La lettre varie bien entendu selon le nom du type générique que vous donnez).

Lorsque vous utilisez des fonctions génériques il peut, parfois, être nécessaire d'expliciter le type des paramètres dans le cas où, par exemple, la fonction appelée possède un type de renvoi générique, ou encore si le compilateur ne dispose pas d'assez d'informations pour inférer le type des paramètres.

Une fonction dont le type des paramètres est explicité devrait ressembler à ceci : `fn::<A, B, ...>()`.

{{#playpen source/fonctionssource0.rs}}

## Voir aussi

[Les fonctions][fonctions], [les structures][struct].

[fonctions]: ../chapitre8/fonctions.html
[struct]: ../chapitre3/struct.html
