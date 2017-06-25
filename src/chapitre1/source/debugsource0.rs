// Cette structure ne peut être affichée par `fmt::Debug`, 
// ni par `fmt::Display`.
struct UnPrintable(i32);

// L'attribut `derive` créé automatiquement l'implémentation requise 
// pour permettre à cette structure d'être affichée avec `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);