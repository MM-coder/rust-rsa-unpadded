use rand::Rng;
use std::io::{self, Write};

fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    for i in (5..).step_by(6).take_while(|i| i * i <= n) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    true
}

fn get_random_prime() -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        let num = rng.gen_range(0..100);
        if is_prime(num) {
            return num;
        }
    }
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    let mut lcm = 0;
    let mut i = 1;
    while i <= a * b {
        if i % a == 0 && i % b == 0 {
            // Check if i is a multiple of both a and b
            lcm = i;
            break;
        }
        i += 1;
    }
    lcm
}

fn modular_multiplicative_inverse(a: u64, m: u64) -> Option<u64> {
    for i in 1..m {
        // The condition '(a % m) * (i % m) % m == 1' checks if the product of 'a' and 'i',
        // under modulo 'm', equals 1. If this condition is true, 'i' is the modular
        // multiplicative inverse of 'a' modulo 'm'.
        if (a % m) * (i % m) % m == 1 {
            return Some(i);
        }
    }
    None
}

fn modular_exponentiation(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        // If the modulus is 1, the result is always 0
        return 0;
    }
    let mut result = 1;
    base = base % modulus; // Reduce the base to a value between 0 and modulus - 1
    while exponent > 0 {
        if exponent % 2 == 1 {
            // Is the exponent odd?
            result = (result * base) % modulus; // Multiply the result with the base
        }
        exponent = exponent >> 1; // Divide the exponent by 2
        base = (base * base) % modulus; // Square the base
    }
    result
}

struct KeyPair {
    public_key: (u64, u64),
    private_key: (u64, u64),
}

fn generate_keypair() -> KeyPair {
    let p = get_random_prime();
    let q = get_random_prime();
    let n = p * q;
    let totient = least_common_multiple(p - 1, q - 1); // Calculate the totient as the mleast
                                                       // common multiple of p-1 and q-1
    let e = 2_u64.pow(16) + 1; // 65537 is a commonly used value for e
    let d = modular_multiplicative_inverse(e, totient).unwrap();
    KeyPair {
        public_key: (e, n),
        private_key: (d, n),
    }
}

fn encrypt_plaintext(plaintext: String, public_key: (u64, u64)) -> Vec<u64> {
    let mut ciphertext = Vec::new();
    for c in plaintext.chars() {
        let m = c as u64;
        let (e, n) = public_key;
        let c = modular_exponentiation(m, e, n);
        ciphertext.push(c);
    }
    ciphertext
}

fn decrypt_ciphertext(ciphertext: Vec<u64>, private_key: (u64, u64)) -> String {
    let mut plaintext = String::new();
    let (d, n) = private_key;
    for &c in ciphertext.iter() {
        let m = modular_exponentiation(c, d, n);
        if let Some(ch) = std::char::from_u32(m as u32) {
            plaintext.push(ch);
        } else {
            println!("Warning: Decryption produced an invalid character.");
        }
    }
    plaintext
}

fn main() {
    let keypair = generate_keypair();

    // Request plaintext input from user
    print!("Enter plaintext to encrypt: ");
    io::stdout().flush().unwrap(); // Flush to make sure the print! macro output is displayed immediately

    let mut plaintext = String::new();
    io::stdin()
        .read_line(&mut plaintext)
        .expect("Failed to read line");
    plaintext = plaintext.trim().to_string(); // Trim newline character

    // Encrypt the plaintext
    let ciphertext = encrypt_plaintext(plaintext.clone(), keypair.public_key);
    println!("Encrypted: {:?}", ciphertext);

    // Decrypt the ciphertext
    let decrypted_plaintext = decrypt_ciphertext(ciphertext, keypair.private_key);
    println!("Decrypted: {}", decrypted_plaintext);

    // Check if decryption is successful
    assert_eq!(plaintext, decrypted_plaintext);
}
