# L'imbrication et les labels

Il est possible de sortir (i.e. `break`) ou de relancer (i.e. `continue`) l'itération d'une boucle à partir d'une autre boucle interne à cette dernière. Pour ce faire, les boucles concernées doivent être annotées avec un `‘label` et il devra être passé aux instructions `break` et/ou `continue`.

{{#playpen source/nestinglabelssource0.rs}}