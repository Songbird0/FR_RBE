# Retour prématuré

Dans l'exemple précédent, nous gérions explicitement les erreurs en utilisant les combinateurs. Une autre manière de répondre à cette analyse est d'utiliser une série de `match` et des *retours prématurés*.

Autrement dit, nous pouvons simplement mettre fin à l'exécution de la fonction et renvoyer l'erreur, s'il y en a une. 
Pour certains, cette manière de faire est plus simple à lire et écrire. Voici une nouvelle version de l'exemple précédent, réécrit en utilisant les retours prématurés:

{{#playpen source/retourpremsource0.rs}}

À ce niveau, nous avons appris à gérer explicitement les erreurs en utilisant les combinateurs et les retours prématurés. Bien que, généralement, nous souhaitons éviter un plantage, la gestion explicite de toutes nos erreurs peut s'avérer relativement lourde.

Dans la section suivante, nous introduirons la macro `try!` pour couvrir les cas où nous souhaitons simplement utiliser `unwrap()` sans faire planter le programme.
