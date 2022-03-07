use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use ring::{
    rand,
    signature::{self, KeyPair},
};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let input = File::open("/Users/kaichaosun/github/holo/ad4m-host/dist/ad4m-macos-x64")
        .map_err(|e| e.to_string())?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    let encoded_digest = HEXLOWER.encode(digest.as_ref());
    println!("SHA-256 digest is {}", encoded_digest);

    // Generate a key pair in PKCS#8 (v2) format.
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).map_err(|e| e.to_string())?;

    // Normally the application would store the PKCS#8 file persistently. Later
    // it would read the PKCS#8 file from persistent storage to use it.

    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).map_err(|e| e.to_string())?;

    // Sign the message.
    let sig = key_pair.sign(encoded_digest.as_bytes());
    println!("Signature of digest is {}", HEXLOWER.encode(sig.as_ref()));

    // Normally an application would extract the bytes of the signature and
    // send them in a protocol message to the peer(s). Here we just get the
    // public key key directly from the key pair.
    let peer_public_key_bytes = key_pair.public_key().as_ref();

    // Verify the signature of the message using the public key. Normally the
    // verifier of the message would parse the inputs to this code out of the
    // protocol message(s) sent by the signer.
    let peer_public_key =
        signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
    peer_public_key.verify(encoded_digest.as_bytes(), sig.as_ref()).map_err(|e| e.to_string())?;
    println!("verify success");

    Ok(())
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, String> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}
