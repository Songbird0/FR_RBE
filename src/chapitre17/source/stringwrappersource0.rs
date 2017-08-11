fn main() {
    // (Le typage est optionnel)
    // Une référence d'une chaîne de caractères immuable.
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // On itère sur les mots dans le sens inverse, aucune nouvelle instance 
    // n'est créée.
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // On copie les caractères dans un vecteur, les trie et supprime 
    // les occurrences multiples.
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // On créé une instance de `String` vide et mutable.
    let mut string = String::new();
    for c in chars {
        // On ajoute un caractère à la fin de la chaîne.
        string.push(c);
        // On ajoute une nouvelle chaîne à la fin de la chaîne initiale.
        string.push_str(", ");
    }

    // La chaîne tronquée est une slice de la chaîne originale, il n'y a 
    // pas de nouvelle allocation.
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Chaîne allouée dans le tas.
    let alice = String::from("I like dogs");
    // Nouvelle allocation mémoire et stockage de la chaîne modifiée 
    // à cet endroit.
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
