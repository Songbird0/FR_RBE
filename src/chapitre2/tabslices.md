# Les tableaux et les "slices"

Un tableau est une collection d'objets appartenant au même type `T`, contenu dans un bloc de mémoire défragmenté. Vous pouvez créer un tableau en utilisant les crochets `[]` et leur taille, connue à la compilation, fait partie intégrante de la signature du type `[T; taille]`.

**Note**: Le terme « slice », en français, pourrait être traduit par « morceau », « tranche », « fragment ». Pour la suite du chapitre, nous utiliserons le terme « slice ».

Les slices sont similaires aux tableaux, à l'exception de leur taille qui n'est pas connue à la compilation. Une slice est un objet composé de deux « mots », le premier étant un pointeur vers la ressource initiale et le second étant la taille de la slice. La taille en mémoire de la slice est déterminée par l'architecture du processeur (e.g. **64 bits** pour une architecture **x86-64**). Les slices peuvent être utilisées pour isoler une partie d'un tableau et héritent de la signature de ce dernier `&[T]`.

{{#playpen source/tabslicessource0.rs}}

