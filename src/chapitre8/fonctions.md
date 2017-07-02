# Les fonctions

Les fonctions sont déclarées à l'aide du mot-clé `fn`. Leurs arguments sont typés, tout comme les variables, et, si la fonction renvoie une valeur, le type renvoyé doit être spécifié à la suite d'une flèche `->`.

La dernière expression se trouvant dans le corps de la fonction sera utilisée pour inférer le type de renvoi. Également, il est possible d'utiliser l'instruction `return` pour effectuer un renvoi prématuré dans la fonction (peut être utilisé dans les boucles et les structures conditionnelles).

Réécrivons les règles de FizzBuzz en utilisant les fonctions !

{{#playpen source/fonctionssource0.rs}}
