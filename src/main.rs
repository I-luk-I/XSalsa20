use rand::{RngCore};

fn main() {
/*
    let key = generateKey();
    println!("Key: {:?}",hex::encode(key));

    let nonce = generate_nonce();
    println!("Nonce: {:?}",hex::encode(nonce));

    let message = "Hello world";

    let encrypt = encrypt(&key,&nonce,message);
    println!("Encrypt data: {:?}",hex::encode(&*encrypt));

    let decrypted = decrypt(key.as_slice(),nonce.as_slice(),encrypt);
    println!("Decrypt data: {:?}",String::from_utf8(decrypted).unwrap());
*/

}
fn flow(key:&[u8],nonce:&[u8])->[u8;64]{

    let mut count:u64 = 0;

    let mut state = [0u32;16];

    let copystate:[u32;16];

    state[0] = 0x61707865;

    let mut countstate = 0;

    (1..=4).for_each(|i|{

        let mut count = 0;

        state[i] = littleendian(key[countstate],key[countstate+1],key[countstate+2],key[countstate+3]);
        countstate+=4;
    });

    state[5] = 0x3320646e;

    countstate = 0;

    (6..=9).for_each(|i|{
        state[i] = littleendian(nonce[countstate],nonce[countstate+1],nonce[countstate+2],nonce[countstate+3]);
        countstate+=4;
    });

    state[10] = 0x79622d32;

    countstate = 16;

    (11..=14).for_each(|i|{
        state[i] = littleendian(key[countstate],key[countstate+1],key[countstate+2],key[countstate+3]);
        countstate+=4;
    });

    state[15] = 0x6b206574;

    copystate = state;

    (0..10).for_each(|_|{
        (state[0], state[4], state[8], state[12]) = quarter_round(state[0], state[4], state[8], state[12]);
        (state[5], state[9], state[13], state[1]) = quarter_round(state[5], state[9], state[13], state[1]);
        (state[10], state[14], state[2], state[6]) = quarter_round(state[10], state[14], state[2], state[6]);
        (state[15], state[3], state[7], state[11]) = quarter_round(state[15], state[3], state[7], state[11]);
        (state[0], state[1], state[2], state[3]) = quarter_round(state[0], state[1], state[2], state[3]);
        (state[5], state[6], state[7], state[4]) = quarter_round(state[5], state[6], state[7], state[4]);
        (state[10], state[11], state[8], state[9]) = quarter_round(state[10], state[11], state[8], state[9]);
        (state[15], state[12], state[13], state[14]) = quarter_round(state[15], state[12], state[13], state[14]);

    });
    let derivedKey = [state[0],state[5],state[10],state[15]];

    let derivedNonce = &nonce[16..];

    let mut salsaState = [0u32;16];

    salsaState[0] = 0x61707865;

    salsaState[5] = 0x3320646e;

    salsaState[10] = 0x79622d32;

    salsaState[15] = 0x6b206574;

    salsaState[1] = derivedKey[0];
    salsaState[2] = derivedKey[1];
    salsaState[3] = derivedKey[2];
    salsaState[4] = derivedKey[3];

    salsaState[11] = derivedKey[0];
    salsaState[12] = derivedKey[1];
    salsaState[13] = derivedKey[2];
    salsaState[14] = derivedKey[3];
    salsaState[6] = (count as u32);
    salsaState[7] = (count >> 32) as u32;
    salsaState[8] = littleendian(derivedNonce[0], derivedNonce[1], derivedNonce[2], derivedNonce[3]);
    salsaState[9] = littleendian(derivedNonce[4], derivedNonce[5], derivedNonce[6], derivedNonce[7]);

    let salsaStatecopy = salsaState;

    (0..20).for_each(|_|{
        (salsaState[0], salsaState[4], salsaState[8], salsaState[12]) = quarter_round(salsaState[0], salsaState[4], salsaState[8], salsaState[12]);
        (salsaState[5], salsaState[9], salsaState[13], salsaState[1]) = quarter_round(salsaState[5], salsaState[9], salsaState[13], salsaState[1]);
        (salsaState[15], salsaState[3], salsaState[7], salsaState[11]) = quarter_round(salsaState[15], salsaState[3], salsaState[7], salsaState[11]);
        (salsaState[0], salsaState[1], salsaState[2], salsaState[3]) = quarter_round(salsaState[0], salsaState[1], salsaState[2], salsaState[3]);
        (salsaState[5], salsaState[6], salsaState[7], salsaState[4]) = quarter_round(salsaState[5], salsaState[6], salsaState[7], salsaState[4]);
        (salsaState[10], salsaState[11], salsaState[8], salsaState[9]) = quarter_round(salsaState[10], salsaState[11], salsaState[8], salsaState[9]);
        (salsaState[15], salsaState[12], salsaState[13], salsaState[14]) = quarter_round(salsaState[15], salsaState[12], salsaState[13], salsaState[14]);
        (salsaState[10], salsaState[14], salsaState[2], salsaState[6]) = quarter_round(salsaState[10], salsaState[14], salsaState[2], salsaState[6]);

    });
    (0..16).for_each(|i|{
        salsaState[i] = salsaState[i].wrapping_add(salsaStatecopy[i]);
    });

    let mut keystream = [0u8;64];

    (0..16).for_each(|i|{
        [keystream[i * 4],keystream[i * 4 +1],keystream[i * 4 + 2],keystream[i * 4 + 3]] = word_to_bytes(salsaState[i]);

    });
    keystream
}
fn addbin(num1:u32,num2:u32)->u32{
    num1.wrapping_add(num2)
}

fn xorr(num1:u32,num2:u32)->u32{
    num1 ^ num2
}
fn rotat(num:u32,posit:u32)->u32{

    num << posit | num >> (32 - posit)
}

fn littleendian(a:u8,b:u8,c:u8,d:u8)->u32{
    (d as u32) << 24 | (c as u32) << 16 | (b as u32) << 8 | (a as u32)

}
fn quarter_round(a: u32, b: u32, c: u32, d: u32) -> (u32, u32, u32, u32) {
    let mut b = b;
    let mut c = c;
    let mut a = a;
    let mut d = d;

    b = xorr(b, rotat(addbin(a, d), 7));
    c = xorr(c, rotat(addbin(b, a), 9));
    a = xorr(a, rotat(addbin(c, b), 13));
    d = xorr(d, rotat(addbin(a, c), 18));

    (a, b, c, d)
}

fn word_to_bytes(num: u32) -> [u8; 4] {
    let b0 = (num & 0xFF) as u8;
    let b1 = ((num >> 8) & 0xFF) as u8;
    let b2 = ((num >> 16) & 0xFF) as u8;
    let b3 = ((num >> 24) & 0xFF) as u8;
    [b0, b1, b2, b3]
}

fn generate_nonce() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    nonce[0..8].copy_from_slice(&time.to_le_bytes()[0..8]);

    rand::thread_rng().fill_bytes(&mut nonce[8..24]);

    nonce
}
fn generateKey()->[u8;32]{
    let mut key = [0u8;32];
    rand::thread_rng().fill_bytes(&mut key);
    key

}
fn encrypt(key:&[u8],nonce:&[u8],data:&str)->Vec<u8>{
    let data = data.as_bytes();

    let keystream = flow(key,nonce);

    let mut encrypt = Vec::new();

    (0..data.len()).for_each(|i|{
        encrypt.push(data[i] ^ keystream[i])
    });
    encrypt
}

fn decrypt(key:&[u8],nonce:&[u8],data:Vec<u8>)->Vec<u8>{
    let data = data;

    let stream = flow(key,nonce);

    let mut decrypt = Vec::new();

    (0..data.len()).for_each(|i|{
        decrypt.push(data[i] ^ stream[i])
    });
    decrypt
}
