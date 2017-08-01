# La structure `File`

La structure `File` représente un fichier qui a été ouvert (contient un descripteur de fichier), et donne les accès lecture et/ou écriture sur le fichier sous-jacent.

Puisque de nombreuses choses peuvent mal se passer lorsqu'une opération est effectuée, toutes les méthodes de `File` renvoie le type `io::Result<T>`, lequel étant un alias pour `Result<T, io::Error>`.

Ceci couvre les potentielles erreurs de toutes les opérations d'entrée/sortie explicites. Grâce à cela, le programmeur peut visualiser toutes les erreurs possibles et est encouragé à les anticiper.
