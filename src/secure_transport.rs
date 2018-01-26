// =========================
// Ppprzlink Secure Transport
// =========================
use super::rusthacl::*;
use std::mem;
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian, WriteBytesExt, ReadBytesExt};

const PPRZ_MSG_TYPE_PLAINTEXT: u8 = 0xaa;
const PPRZ_MSG_TYPE_ENCRYPTED: u8 = 0x55;

// ENCRYPTED MSG:
// 0 crypto/plaintext byte
// 1 counter MSB 1 {big-endian}
// 2 counter MSB 2 {big-endian}
// 3 counter MSB 3 {big-endian}
// 4 counter MSB 4 {big-endian}
// 5 	source_ID {auth}
// 6 	dest_ID {auth}
// 7..N:    	... encrypted payload {encrypted}
// end-15..end: tag (16 bytes) {tag}
//
// PLAINTEXT MSG:
// 0 crypto/plaintext byte
// 1 source_ID
// 2 dest_ID
// 3 class/component byte
// 4 MSG_ID
// .. msg payload {optional}

/// index of encrypted/payload byte
const PPRZ_GEC_IDX: usize = 0;
/// index of the beginning of the counter
const PPRZ_CNTR_IDX: usize = 1;
/// index of the beginning of the authenticated bytes
const PPRZ_AUTH_IDX: usize = 5;
/// index of the beginning of the ciphertex
const PPRZ_CIPH_IDX: usize = 7;

/// length of the encryption keys
const PPRZ_KEY_LEN: usize = 32;
/// length of the authenticated data
const PPRZ_AUTH_LEN: usize = 2;
/// length of the message authentication tag
const PPRZ_MAC_LEN: usize = 16;
/// length of the message nonce
const PPRZ_NONCE_LEN: usize = 12;
/// length of the counter
const PPRZ_COUNTER_LEN: usize = 4;

/// index of the message ID for plaintext messages
const PPRZ_PLAINTEXT_MSG_ID_IDX: usize = 4;

/// 4 bytes of MSG info (source_ID, dest_ID, class_byte, msg_ID) + 1 GEC byte
const PPRZ_PLAINTEXT_MSG_MIN_LEN: usize = 5;

/// 20 bytes crypto overhead + 4 bytes MSG info + 1 GEC byte
const PPRZ_ENCRYPTED_MSG_MIN_LEN: usize = 25;

/// length of the crypto overhead (4 bytes of counter + 16 bytes of tag)
const PPRZ_CRYPTO_OVERHEAD: usize = 20;


#[derive(Debug)]
struct GecSymKey {
    key: [u8; PPRZ_KEY_LEN],
    nonce: [u8; PPRZ_NONCE_LEN],
    ctr: u32,
}

impl GecSymKey {
    pub fn new() -> GecSymKey {
        let k = [0; PPRZ_KEY_LEN];
        let n = [0; PPRZ_NONCE_LEN];
        let c = 0;
        GecSymKey {
            key: k,
            nonce: n,
            ctr: c,
        }
    }

    pub fn update(&mut self, k: &[u8], n: &[u8], c: u32) -> Result<(), String> {
        if k.len() != PPRZ_KEY_LEN {
            return Err(String::from(
                "GecSymKey update Error: k.len() != PPRZ_KEY_LEN",
            ));
        }
        if n.len() != PPRZ_NONCE_LEN {
            return Err(String::from(
                "GecSymKey update Error: n.len() != PPRZ_NONCE_LEN",
            ));
        }

        self.ctr = c;
        self.key.clone_from_slice(k);
        self.nonce.clone_from_slice(n);

        Ok(())
    }
}

/// Convert bytes to a counter
/// Network byte order is big endian
pub fn pprzlink_bytes_to_counter(bytes: &[u8]) -> Result<u32, String> {
    let size = mem::size_of::<u32>();
    if bytes.len() != size {
        return Err(String::from("Error, size of slice doesn't match the size of u32"));
    }
    let mut rdr = Cursor::new(bytes);
    match rdr.read_u32::<BigEndian>() {
        Ok(v) => return Ok(v),
        Err(_) => return Err(String::from("Error during reading bytes")),
    };
}

pub fn pprzlink_counter_to_bytes(c: u32) -> Vec<u8> {
	let mut buf = vec![];
	buf.write_u32::<LittleEndian>(c).expect("Error - converting to bytes not succesful");
	buf
}

/// can be used for for tx and rx
pub struct SecurePprzTransport {
    pub length: u8,
    pub buf: Vec<u8>,
    pub allowed_msg_ids: Vec<u8>,
    rx_sym_key: GecSymKey,
}

impl SecurePprzTransport {
    pub fn new() -> SecurePprzTransport {
        SecurePprzTransport {
            length: 0,
            buf: vec![],
            allowed_msg_ids: vec![],
            rx_sym_key: GecSymKey::new(),
        }
    }

    pub fn process_message(&mut self, payload: &[u8]) -> Result<Vec<u8>, String> {
        let mut data = vec![];
        match payload[PPRZ_GEC_IDX] {
            PPRZ_MSG_TYPE_PLAINTEXT => {
                // check if we have enough data in the buffer
                if payload.len() < PPRZ_PLAINTEXT_MSG_MIN_LEN {
                    return Err(String::from("Plaintext msg payload too short"));
                }
                // check if the message ID is whitelisted
                if self.allowed_msg_ids.contains(
                    &payload[PPRZ_PLAINTEXT_MSG_ID_IDX],
                )
                {
                    data.extend_from_slice(&payload[1..]);
                    return Ok(data);
                }
                return Err(String::from("Plaintext msg not in the whitelist"));
            }
            PPRZ_MSG_TYPE_ENCRYPTED => {
                // check if we have enough data in the buffer
                if payload.len() < PPRZ_ENCRYPTED_MSG_MIN_LEN {
                    return Err(String::from("Encrypted msg payload too short"));
                }

				// first check the message counter
                let counter: &[u8] = &payload[PPRZ_CNTR_IDX..PPRZ_CNTR_IDX + PPRZ_COUNTER_LEN];
                let counter_as_u32 = pprzlink_bytes_to_counter(counter).expect("Error converting counter");
                
                // check against the saved counter
                if counter_as_u32 <= self.rx_sym_key.ctr {
                	return Err(String::from("Decryption error: received counter is not larger than the saved counter"));
                }
                
                // update nonce
                self.rx_sym_key.nonce[0..4].clone_from_slice(
                    &payload
                        [PPRZ_CNTR_IDX..PPRZ_CNTR_IDX + PPRZ_COUNTER_LEN],
                );
                
                // update intermediate fields
                let auth: &[u8] = &payload[PPRZ_AUTH_IDX..PPRZ_AUTH_IDX + PPRZ_AUTH_LEN];
                let tag: &[u8] = &payload[payload.len() - PPRZ_MAC_LEN..];
                let ciphertext: &[u8] = &payload[PPRZ_CIPH_IDX..payload.len() - PPRZ_MAC_LEN];
                let mut plaintext = Vec::with_capacity(auth.len() + ciphertext.len());
                plaintext.extend_from_slice(auth);
                plaintext.extend_from_slice(ciphertext);

                println!("counter = {:?}", counter);
                println!("nonce = {:?}", self.rx_sym_key.nonce);
                println!("auth = {:?}", auth);
                println!("ciphertext = {:?}", ciphertext);
                println!("plaintext ={:?}", plaintext);
                println!("tag = {:?}", tag);
                match chacha20poly1305_aead_decrypt(
                    &mut plaintext[auth.len()..],
                    tag,
                    ciphertext,
                    auth,
                    &self.rx_sym_key.key,
                    &self.rx_sym_key.nonce,
                ) {
                    Ok(val) => {
                        if val {
                        	self.rx_sym_key.ctr = counter_as_u32;
                            return Ok(plaintext);
                        }
                    }
                    Err(msg) => return Err(msg),
                };

                // attempt decryption
                return Err(String::from("Decryption not sucessful"));
            }
            _ => {
                return Err(String::from("Unknown byte value"));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // TODO: add tests for encryption and decryption
    // TODO: figure how to nicely update the Gec sym key struct
    static KEY: [u8; 32] = [
        0x70,
        0x03,
        0xAA,
        0x0A,
        0x8E,
        0xE9,
        0xA8,
        0xFF,
        0xD5,
        0x46,
        0x1E,
        0xEC,
        0x7C,
        0xC1,
        0xC1,
        0xA1,
        0x6A,
        0x43,
        0xC9,
        0xD4,
        0xB3,
        0x2B,
        0x94,
        0x7E,
        0x76,
        0xF9,
        0xD8,
        0xE8,
        0x1A,
        0x31,
        0x5D,
        0xA8,
    ];

    static CIPHERTEXT: [u8; 2] = [0xa1, 0x7b];
    static PLAINTEXT: [u8; 2] = [4, 3];
    static AUTH: [u8; 2] = [1, 2];
    static MAC: [u8; 16] = [
        0x83,
        0xf6,
        0x95,
        0x66,
        0x4a,
        0xa4,
        0x82,
        0x82,
        0x12,
        0xf0,
        0x7f,
        0xa1,
        0xf,
        0x92,
        0x86,
        0xea,
    ];
    static NONCE: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc];


    #[test]
    fn trivial() {    	
        let mut trans = SecurePprzTransport::new();
        trans.rx_sym_key.update(&KEY, &NONCE, 0);

        let mut buf = vec![PPRZ_MSG_TYPE_ENCRYPTED];
        buf.extend_from_slice(&NONCE[0..4]); // counter
        buf.extend_from_slice(&AUTH); // auth data
        buf.extend_from_slice(&CIPHERTEXT); // encrypted payload
        buf.extend_from_slice(&MAC); // tag

        println!("b.len={}", buf.len());
        match trans.process_message(&buf) {
            Ok(x) => println!("ok: plaintext: {:?}",x),
            Err(e) => println!("error {}", e),
        }
    }
}
