fn main() {
    // Addition d'entiers
    println!("1 + 2 = {}", 1u32 + 2);

    // Soustraction d'entiers
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Essayez de changer `1i32` par `1u32` pour prendre conscience de l'importance du type

    // Logique booléenne
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Opérations bit-à-bit
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Utilisez des underscores pour améliorer la lisibilité !
    println!("One million is written as {}", 1_000_000u32);
}