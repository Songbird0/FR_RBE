fn main(){
    // En général, le marqueur '{}' sera automatiquement remplacé par 
    // n'importe quel argument. Il sera transformé en chaîne de caractères.
    println!("{} jours", 31);
    
    // Sans suffixe, 31 est de type i32. Vous pouvez changez le type de 31 avec
    // un suffixe. (e.g. 31i64)
    
    // Différents modèles peuvent être utilisés. 
    // Les marqueurs de position peuvent être utilisés.
    println!("{0}, voici {1}. {1}, voici {0}", "Alice", "Bob");
    
    // Les marqueurs peuvent également être
    // nommés
    println!("{sujet} {verbe} {objet}", 
    objet="le chien paresseux",
    sujet="Rapide, le renard",
    verbe="saute par-dessus");
    
    // Un formatage spécial peut être spécifié après un ':'.
    println!("{} personne sur {:b} sait lire le binaire, l'autre moité non.", 1, 2);
    
    // Vous pouvez aligner vers la droite votre texte en spécifiant 
    // la largeur (en espace) entre le côté gauche de la console 
    // et votre chaîne. Cet exemple affichera: "     1", un "1" après 5 espaces.

    println!("{number:>width$}", number=1, width=6);
    
    // Vous pouvez également remplacer les white spaces par des '0'.
    // Affiche: "000001"
    
    println!("{number:>0width$}", number=1, width=6);
    
    // Le nombre d'arguments utilisé est vérifié par le compilateur.
    println!("Mon nom est {0}, {1} {0}", "Bond");
    // FIXME ^ Ajoutez l'argument manquant: "James".
    
    // On créé une structure nommé 'Structure' contenant un entier de type 'i32'.
    #[allow(dead_code)]
    struct Structure(i32);
    
    // Cependant, les types complexes tels que les structures demandent
    // une gestion de l'affichage plus complexe. Cela ne fonctionnera pas.
    println!("Cette structure '{}' ne sera pas affichée...", Structure(3));
    // FIXME ^ Commentez cette ligne pour voir l'erreur disparaître.
}