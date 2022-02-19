use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use anyhow::Error;
use num_bigint::BigUint;
use num_prime::RandPrime;
use rand::Rng;

fn main() -> Result<(), Error> {
    let key = match prompt("User (1/2): ")? {
        1 => user1(),
        2 => user2(),
        _ => exit(1),
    }?;
    println!("key = {:x}", key);
    let key = key.to_be_bytes();

    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(bytes) if bytes > 0 => {}
            _ => break,
        }
        match try_decode(line.trim(), key) {
            Ok(plain) => println!("{}", plain),
            Err(_) => {
                let cry = crypt(line.trim_end().as_bytes().to_vec(), key);
                println!("{}", base64::encode(cry))
            }
        }
    }
    Ok(())
}

fn user1() -> Result<u128, Error> {
    let mut random = rand::thread_rng();
    let p: BigUint = random.gen_safe_prime(127);
    let a = BigUint::try_from(random.gen::<u8>())?;
    let n1 = BigUint::try_from(random.gen::<u32>())?;
    let x1 = a.modpow(&n1, &p);
    println!("Diffie-Hellman: p = {}, a = {}, x1 = {}", p, a, x1);
    let x2 = BigUint::try_from(prompt("x2 = ")?)?;
    Ok(x2.modpow(&n1, &p).try_into()?)
}

fn user2() -> Result<u128, Error> {
    let mut random = rand::thread_rng();
    let p = BigUint::try_from(prompt("p = ")?)?;
    let a = BigUint::try_from(prompt("a = ")?)?;
    let x1 = BigUint::try_from(prompt("x1 = ")?)?;
    let n2 = BigUint::try_from(random.gen::<u32>())?;
    let x2 = a.modpow(&n2, &p);
    println!("Diffie-Hellman: x2 = {}", x2);
    Ok(x1.modpow(&n2, &p).try_into()?)
}

fn prompt(print: &str) -> Result<u128, Error> {
    print!("{}", print);
    stdout().flush()?;
    let mut n = String::new();
    stdin().read_line(&mut n)?;
    Ok(n.trim().parse()?)
}

fn try_decode(b64: &str, key: [u8; 16]) -> Result<String, Error> {
    let decode = base64::decode(b64)?;
    Ok(String::from_utf8(crypt(decode, key))?)
}

fn crypt(mut plain: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
    for i in 0..plain.len() {
        plain[i] ^= key[i & 0xf];
    }
    plain
}
