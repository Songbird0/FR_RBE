# Limitations des accès lecture/écriture

Il est possible de faire autant « d'emprunts immuables » (i.e. récupérer une référence immuable) qu'on le souhaite. Toutefois, tant qu'il y a des accès en lecture par référence, l'objet original ne peut pas être modifié. Par contre, un seul accès en écriture, par ressource, est permis tant que le dernier emprunt par référence mutable n'est pas terminé (i.e. tant que la référence mutable est toujours dans le contexte).

{{#playpen source/aliasingsource0.rs}}
