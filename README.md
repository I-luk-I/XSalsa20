# XSalsa20 Rust Implementation

Rust implementation of the XSalsa20 symmetric encryption algorithm.

## ⚠️ Important Warning

**This implementation SHOULD NOT be used in production code**, as it has not undergone sufficient testing and security audit.

## Usage

```rust
use xsalsa20_rust::{encrypt, decrypt, generate_key, generate_nonce};

    let key = generateKey();
    println!("Key: {:?}",hex::encode(key));

    let nonce = generate_nonce();
    println!("Nonce: {:?}",hex::encode(nonce));

    let message = "Hello world";

    let encrypt = encrypt(&key,&nonce,message);
    println!("Encrypt data: {:?}",hex::encode(&*encrypt));

    let decrypted = decrypt(key.as_slice(),nonce.as_slice(),encrypt);
    println!("Decrypt data: {:?}",String::from_utf8(decrypted).unwrap());

```
