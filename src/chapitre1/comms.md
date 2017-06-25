# Les commentaires

N'importe quel programme a besoin de commentaires, c'est pour cela que Rust supporte différentes syntaxes :

Les commentaires basiques ignorés par le compilateur :

*  `// Les commentaires mono-lignes.`;
*  `/* Les blocs de commentaires régis par leurs délimiteurs. */`.

Les commentaires dédiés à la documentation qui seront converti au format HTML :

*  `/// Génère de la documentation pour ce qui suit ce commentaire.`;
*  `//! Génère la documentation pour un conteneur (e.g. un module)`;
*  `/*! Permet de rédiger un bloc entier de documentation.*/`.