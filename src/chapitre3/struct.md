# Les structures

Il y a trois types de structures pouvant être créé en utilisant le mot-clé `struct`:


1. Les « tuple structs », aussi appelées simplement tuples;
2. Les [structures classiques](https://en.wikipedia.org/wiki/Struct_(C_programming_language)#Declaration) issues du langage C;
3. Les structures unitaires. Ne possèdant aucun champ, elles sont utiles pour la généricité.

{{#playpen source/structsource0.rs}}

### Activité

1. Ajoutez une fonction `rect_area` qui calcule l'air d'un rectangle (essayez d'utiliser la déstructuration);
2. Ajoutez une fonction `square` qui prend en paramètre une instance de la structure `Point` et un réel de type `f32` puis renvoie une instance de la structure `Rectangle` contenant le point du coin inférieur gauche du rectangle ainsi qu'une largeur et une hauteur correspondant au réel passé en paramètre à la fonction `square`.

### Voir aussi

[Les attributs](../chapitre11/attributes.html) et [la déstructuration](../chapitre7/destruct.html).