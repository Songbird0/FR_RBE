# Mutabilité

Une ressource mutable peut être modifiée, tout en étant empruntée, en utilisant `&mut T`. Nous passons alors une référence mutable de la ressource, donnant un accès en lecture et en écriture à l'emprunteur. En revanche, une référence dont la mutabilité n'est pas précisée (i.e. que le mot-clé `mut` n'est pas présent) empêche l'emprunteur d'accéder en écriture à la ressource.

{{#playpen source/borrowingmutsource0.rs}}

## Voir aussi

[La lifetime `'static`][static].

[static]: ../chapitre13/static.html
