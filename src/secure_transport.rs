// =========================
// Ppprzlink Secure Transport
// =========================
use super::rusthacl::*;
use super::parser::{PprzDictionary, PprzMsgBaseType, PprzMsgClassID, PprzMessage,
                    PprzProtocolVersion};
use super::transport::PprzTransport;
use std::sync::Arc;
use std::mem;
use std::io::Cursor;
use super::rand::Rng;
use super::rand::os::OsRng;
use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};

use parser::{V1_MSG_ID, V2_MSG_ID};

/// Two possible values of the crypto byte
const PPRZ_MSG_TYPE_PLAINTEXT: u8 = 0xaa;
const PPRZ_MSG_TYPE_ENCRYPTED: u8 = 0x55;

/// MSG_ID for key exchange messages. Update if IDs change in messages.xml
const KEY_EXCHANGE_MSG_ID_UAV: u8 = 239;
const KEY_EXCHANGE_MSG_ID_GCS: u8 = 159;

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
/// legth of the message signature
const PPRZ_SIGN_LEN: usize = 64;
/// lenght of a hash for key derivation
const PPRZ_HASH_LEN: usize = 64;
/// length of the encryption keys
const PPRZ_KEY_LEN: usize = 32;
/// length of the message authentication tag
const PPRZ_MAC_LEN: usize = 16;
/// length of the message nonce
const PPRZ_NONCE_LEN: usize = 12;
/// length of the counter
const PPRZ_COUNTER_LEN: usize = 4;
/// length of the crypto overhead (4 bytes of counter + 16 bytes of tag)
const PPRZ_CRYPTO_OVERHEAD: usize = 20;
/// basepoint value for the scalar curve multiplication
const PPRZ_CURVE_BASEPOINT: u8 = 9;

/// Pprzlink 2.0: index of the beginning of the ciphertex
const V2_PPRZ_CIPH_IDX: usize = 7;
/// Pprzlink 2.0: index of the message ID for plaintext messages
const V2_PPRZ_PLAINTEXT_MSG_ID_IDX: usize = 4;
/// Pprzlink 2.0: 4 bytes of MSG info (source_ID, dest_ID, class_byte, msg_ID) + 1 GEC byte
const V2_PPRZ_PLAINTEXT_MSG_MIN_LEN: usize = 5;
/// Pprzlink 2.0: 20 bytes crypto overhead + 4 bytes MSG info + 1 GEC byte
const V2_PPRZ_ENCRYPTED_MSG_MIN_LEN: usize = 25;
/// Pprzlink 2.0: length of the authenticated data (SENDER ID, DEST ID)
const V2_PPRZ_AUTH_LEN: usize = 2;

/// Pprzlink 1.0: index of the beginning of the ciphertex
const V1_PPRZ_CIPH_IDX: usize = 6;
/// Pprzlink 1.0: index of the message ID for plaintext messages
const V1_PPRZ_PLAINTEXT_MSG_ID_IDX: usize = 2;
/// Pprzlink 1.0: 2 bytes of MSG info (source_ID, msg_ID) + 1 GEC byte
const V1_PPRZ_PLAINTEXT_MSG_MIN_LEN: usize = 3;
/// Pprzlink 1.0: 20 bytes crypto overhead + 2 bytes MSG info + 1 GEC byte
const V1_PPRZ_ENCRYPTED_MSG_MIN_LEN: usize = 23;
/// Pprzlink 1.0: length of the authenticated data (SENDER ID)
const V1_PPRZ_AUTH_LEN: usize = 1;

/// Container for a symmetric key
#[derive(Debug)]
pub struct GecSymKey {
    key: [u8; PPRZ_KEY_LEN],
    nonce: [u8; PPRZ_NONCE_LEN],
    ctr: u32,
    ready: bool,
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
            ready: false,
        }
    }

    pub fn is_ready(&self) -> bool {
        return self.ready;
    }

    #[allow(dead_code)]
    pub fn set(&mut self, k: &[u8], n: &[u8], c: u32) -> Result<(), String> {
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
        self.ready = true;

        Ok(())
    }
}

/// Convert bytes to a 32 bit counter
/// From NetworkEndian -> NativeEndian
pub fn pprzlink_bytes_to_counter(bytes: &[u8]) -> Result<u32, String> {
    let size = mem::size_of::<u32>();
    if bytes.len() != size {
        return Err(String::from(
            "Error, size of slice doesn't match the size of u32",
        ));
    }
    let mut rdr = Cursor::new(bytes);
    match rdr.read_u32::<NetworkEndian>() {
        Ok(v) => return Ok(v),
        Err(_) => return Err(String::from("Error during reading bytes")),
    };
}

/// Convert a 32 bit counter to network endianness
/// From NativeEndian -> NetworkEndian
pub fn pprzlink_counter_to_bytes(c: u32) -> Vec<u8> {
    let mut buf = vec![];
    buf.write_u32::<NetworkEndian>(c)
        .expect("Error - converting to bytes not succesful");
    buf
}

/// can be used for for tx and rx
pub struct SecurePprzTransport {
    /// protocol version
    protocol: PprzProtocolVersion,
    /// pprz transport for parsing incoming messages
    rx: PprzTransport,
    /// pprz transport for constructing outgoing messages
    tx: PprzTransport,
    /// list of whitelisted message IDs
    pub allowed_msg_ids: Vec<u8>,
    /// symmetric session key for incoming messages
    pub rx_sym_key: GecSymKey,
    /// symmetric session key for outgoing messages
    pub tx_sym_key: GecSymKey,
    /// set STS party
    party: StsParty,
    /// set STS stage
    stage: StsStage,
    /// mark last STS error
    pub last_error: StsError,
    /// last error message
    pub last_error_message: String,
    /// remote party's public key
    pub their_public_key: GecPubKey,
    /// local private key
    pub my_private_key: GecPrivKey,
    /// remote party's ephemeral key for STS
    pub their_public_ephemeral: GecPubKey,
    /// local private key for STS
    pub my_private_ephemeral: GecPrivKey,
    /// pprz dictionary for sending KEY_EXCHANGE and other messages
    pub dictionary: Option<Arc<PprzDictionary>>,
    /// destination ID for the messages
    destination_id: u8,
    /// sender ID for the messages
    sender_id: u8,
    /// a message class we are interested in receiving
    rx_msg_class: PprzMsgClassID,
    /// msg1 of the Sts protocol
    msg1: Option<Vec<u8>>,
    /// msg2 of the Sts protocol
    msg2: Option<Vec<u8>>,
    /// msg 3 of the Sts protocol
    msg3: Option<Vec<u8>>,
}

impl SecurePprzTransport {
    /// Create a new transport. Required fields are:
    /// party - which StS party are we
    /// destination_id - the AC_ID of the destination
    pub fn new(party: StsParty, destination_id: u8) -> SecurePprzTransport {
        SecurePprzTransport {
            protocol: PprzProtocolVersion::ProtocolV2,
            rx: PprzTransport::new(),
            tx: PprzTransport::new(),
            allowed_msg_ids: vec![KEY_EXCHANGE_MSG_ID_UAV, KEY_EXCHANGE_MSG_ID_GCS],
            rx_sym_key: GecSymKey::new(),
            tx_sym_key: GecSymKey::new(),
            party: party,
            stage: StsStage::Init,
            last_error: StsError::ERROR_NONE,
            last_error_message: String::new(),
            their_public_key: GecPubKey::new(),
            my_private_key: GecPrivKey::new(),
            their_public_ephemeral: GecPubKey::new(),
            my_private_ephemeral: GecPrivKey::new(),
            dictionary: None,
            destination_id: destination_id,
            sender_id: 0,
            rx_msg_class: PprzMsgClassID::Telemetry,
            msg1: None,
            msg2: None,
            msg3: None,
        }
    }

    /// set the pprzlink version (1.0 or 2.0)
    pub fn set_pprzlink_version(&mut self, version: PprzProtocolVersion) {
        self.protocol = version;
    }

    /// set the message class for incoming messages
    pub fn set_msg_class(&mut self, msg_class: PprzMsgClassID) {
        self.rx_msg_class = msg_class;
    }

    /// get current stage
    pub fn get_stage(&self) -> &StsStage {
        &self.stage
    }

    /// set the destination ID for the messages
    pub fn set_destination(&mut self, id: u8) {
        self.destination_id = id;
    }

    /// set sender ID for the messages
    pub fn set_sender(&mut self, id: u8) {
        self.sender_id = id;
    }

    /// Checks if the message is in the whitelist, and if so, prepends a crypto byte
    /// Assumes payload in form of source_ID .. msg_payload
    /// Returns plaintext pprzlink 2.0 message
    pub fn pack_plaintext_message(&mut self, payload: &[u8]) -> Result<Vec<u8>, String> {
        let msg_id;
        match self.protocol {
            PprzProtocolVersion::ProtocolV1 => {
                msg_id = V1_MSG_ID;
            }
            PprzProtocolVersion::ProtocolV2 => {
                msg_id = V2_MSG_ID;
            }
        }
        if payload.len() <= msg_id {
            return Err(String::from("Plaintext msg payload too short"));
        }
        // check if the message ID is whitelisted
        if self.allowed_msg_ids.contains(&payload[msg_id]) {
            let mut v = vec![PPRZ_MSG_TYPE_PLAINTEXT];
            v.extend_from_slice(&payload);
            return Ok(v);
        }
        return Err(String::from("Plaintext msg not in the whitelist"));
    }

    /// Attemp message encryption
    /// Adds crypto_byte, counter and tag
    /// Assumes that the first two bytes of 'payload' are source_ID and dest_ID
    /// and these two bytes will be authenticated
    /// Returns encrypted pprzlink 2.0 message (crypto_byte .. tag)
    pub fn encrypt_message(&mut self, payload: &[u8]) -> Result<Vec<u8>, String> {
        let msg_id;
        let pprz_auth_len;
        match self.protocol {
            PprzProtocolVersion::ProtocolV1 => {
                msg_id = V1_MSG_ID;
                pprz_auth_len = V1_PPRZ_AUTH_LEN;
            }
            PprzProtocolVersion::ProtocolV2 => {
                msg_id = V2_MSG_ID;
                pprz_auth_len = V2_PPRZ_AUTH_LEN;
            }
        }

        if payload.len() <= msg_id {
            return Err(String::from("Msg payload too short"));
        }

        if !self.tx_sym_key.is_ready() {
            return Err(String::from("Encryption keys not ready"));
        }

        // increment counter
        let counter = self.tx_sym_key.ctr + 1;
        // convert to bytes
        let counter_as_bytes = pprzlink_counter_to_bytes(counter);

        // update nonce
        self.tx_sym_key.nonce[0..4].clone_from_slice(&counter_as_bytes);

        // update intermediate fields
        let auth: &[u8] = &payload[0..pprz_auth_len];
        let plaintext: &[u8] = &payload[pprz_auth_len..];
        let mut ciphertext = vec![0; plaintext.len()];
        let mut tag: [u8; PPRZ_MAC_LEN] = [0; PPRZ_MAC_LEN];
        let mut msg = Vec::with_capacity(payload.len() + PPRZ_CRYPTO_OVERHEAD + 1);
        msg.push(PPRZ_MSG_TYPE_ENCRYPTED); // crypto byte
        match chacha20poly1305_aead_encrypt(
            &mut ciphertext,
            &mut tag,
            plaintext,
            auth,
            &self.tx_sym_key.key,
            &self.tx_sym_key.nonce,
        ) {
            Ok(val) => {
                if val {
                    self.tx_sym_key.ctr = counter; // update counter
                    msg.extend_from_slice(&counter_as_bytes); // 4 bytes of counter
                    msg.extend_from_slice(auth); // 2 bytes of auth
                    msg.extend_from_slice(&ciphertext); // ciphertext
                    msg.extend_from_slice(&tag); // tag
                    return Ok(msg);
                }
            }
            Err(msg) => return Err(msg),
        };

        Ok(msg)
    }

    /// Attemp message decryption
    /// If a message is unencrypted, pass it through only if the MSG_ID is in the whitelist
    /// Returns Pprzlink 2.0 message bytes (source_ID .. msg payload)
    pub fn decrypt_message(&mut self, payload: &[u8]) -> Result<Vec<u8>, String> {
        let pprz_plaintext_msg_min_len;
        let pprz_plaintext_msg_id_idx;
        let pprz_encrypted_msg_min_len;
        let pprz_auth_len;
        let pprz_ciph_idx;

        match self.protocol {
            PprzProtocolVersion::ProtocolV1 => {
                pprz_plaintext_msg_min_len = V1_PPRZ_PLAINTEXT_MSG_MIN_LEN;
                pprz_plaintext_msg_id_idx = V1_PPRZ_PLAINTEXT_MSG_ID_IDX;
                pprz_encrypted_msg_min_len = V1_PPRZ_ENCRYPTED_MSG_MIN_LEN;
                pprz_auth_len = V1_PPRZ_AUTH_LEN;
                pprz_ciph_idx = V1_PPRZ_CIPH_IDX;
            }
            PprzProtocolVersion::ProtocolV2 => {
                pprz_plaintext_msg_min_len = V2_PPRZ_PLAINTEXT_MSG_MIN_LEN;
                pprz_plaintext_msg_id_idx = V2_PPRZ_PLAINTEXT_MSG_ID_IDX;
                pprz_encrypted_msg_min_len = V2_PPRZ_ENCRYPTED_MSG_MIN_LEN;
                pprz_auth_len = V2_PPRZ_AUTH_LEN;
                pprz_ciph_idx = V2_PPRZ_CIPH_IDX;
            }
        }

        let mut data = vec![];
        match payload[PPRZ_GEC_IDX] {
            PPRZ_MSG_TYPE_PLAINTEXT => {
                // check if we have enough data in the buffer
                if payload.len() < pprz_plaintext_msg_min_len {
                    return Err(String::from("Plaintext msg payload too short"));
                }
                // check if the message ID is whitelisted
                if self.allowed_msg_ids
                    .contains(&payload[pprz_plaintext_msg_id_idx])
                {
                    data.extend_from_slice(&payload[1..]);
                    return Ok(data);
                }
                return Err(String::from("Plaintext msg not in the whitelist"));
            }
            PPRZ_MSG_TYPE_ENCRYPTED => {
                // check if we have enough data in the buffer
                if payload.len() < pprz_encrypted_msg_min_len {
                    return Err(String::from("Encrypted msg payload too short"));
                }

                if !self.rx_sym_key.is_ready() {
                    return Err(String::from("Encryption keys not ready"));
                }

                // first check the message counter
                let counter: &[u8] = &payload[PPRZ_CNTR_IDX..PPRZ_CNTR_IDX + PPRZ_COUNTER_LEN];
                let counter_as_u32 =
                    pprzlink_bytes_to_counter(counter).expect("Error converting counter");

                // check against the saved counter
                if counter_as_u32 <= self.rx_sym_key.ctr {
                    return Err(String::from(
                        "Decryption error: received counter is not larger than the saved counter",
                    ));
                }

                // update nonce
                self.rx_sym_key.nonce[0..4]
                    .clone_from_slice(&payload[PPRZ_CNTR_IDX..PPRZ_CNTR_IDX + PPRZ_COUNTER_LEN]);

                // update intermediate fields
                let auth: &[u8] = &payload[PPRZ_AUTH_IDX..PPRZ_AUTH_IDX + pprz_auth_len];
                let tag: &[u8] = &payload[payload.len() - PPRZ_MAC_LEN..];
                let ciphertext: &[u8] = &payload[pprz_ciph_idx..payload.len() - PPRZ_MAC_LEN];
                let mut plaintext = Vec::with_capacity(auth.len() + ciphertext.len());
                plaintext.extend_from_slice(auth);
                plaintext.extend_from_slice(ciphertext);

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

    /// Input:
    /// payload - byte representation of paparazzi message v2.0, specifically
    /// in the format of:
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    ///
    /// Output:
    /// if status == Crypto_OK => returns an encrypted message in the format of:
    /// ```ignore
    /// 0 PPRZ_STX
    /// 1 msg len
    /// 2 crypto byte
    /// 3 counter MSB 1 {big-endian}
    /// 4 counter MSB 2 {big-endian}
    /// 5 counter MSB 3 {big-endian}
    /// 6 counter MSB 4 {big-endian}
    /// 7 	source_ID {auth}
    /// 8 	dest_ID {auth}
    /// 9: class_component_IN {encrypted}
    /// 10: msg_ID {encrypted}
    /// 11..end-19: optional msg payload {encrypted}
    /// end-18..end-2: tag (16 bytes) {tag}
    /// end-1 crc A
    /// end crc B
    /// ```
    ///
    /// if status != Crypto_OK => returns a plaintext message relevant to the internal stage
    /// of the STS protocol in the format of:
    /// ```ignore
    /// 0 PPRZ_STX
    /// 1 msg len
    /// 2 crypto byte
    /// 3 source_ID
    /// 4 dest_ID
    /// 10: class_component_IN
    /// 11: msg_ID
    /// 12..end-2: optional msg payload
    /// end-1 crc A
    /// end crc B
    /// ```
    ///
    /// If an error occurs during message processing, `None` is returned.
    /// In such case the last error can be read using `last_error` and `last_error_message`
    /// variables.
    pub fn construct_pprz_msg(&mut self, payload: &[u8]) -> Option<Vec<u8>> {
        match self.stage {
            StsStage::CryptoOK => {
                match self.encrypt_message(payload) {
                    Ok(v) => {
                        // message encrypted sucessfully
                        self.tx.reset(); // reset the underlying transport
                        self.tx.construct_pprz_msg(&v); // append STX and checksum
                        return Some(self.tx.buf.clone()); // return a message ready to be sent
                    }
                    Err(e) => {
                        self.last_error = StsError::ONGOING_COMM_ERROR;
                        self.last_error_message = e;
                        return None;
                    }
                }
            }
            StsStage::Init => {
                match self.party {
                    StsParty::Initiator => {
                        // 1. A generates an ephemeral (random) curve25519 key pair (Pae, Qae)
                        self.sts_ephemeral_curve25519_key_pair();
                        // and sends P_ae
                        let mut msg1 = self.sts_initiate_msg(); // get MSG1
                        self.msg1 = Some(msg1.clone()); // save for reuse
                        msg1 = self.pack_plaintext_message(&msg1).unwrap(); // add crypto byte (and pass whitelist)
                        self.tx.construct_pprz_msg(&msg1); // append STX and checksum
                        self.stage = StsStage::WaitMsg2; // update status
                        println!("returning msg1, waiting msg2");
                        return Some(self.tx.buf.clone()); // return a message ready to be sent
                    }
                    StsParty::Responder => {
                        // 2. B generates ephemeral curve25519 key pair (Pbe, Qbe).
                        self.sts_ephemeral_curve25519_key_pair();
                        self.stage = StsStage::WaitMsg1; // update status
                        return None; // return nothing
                    }
                }
            }
            StsStage::WaitMsg1 => {
                match self.party {
                    StsParty::Initiator => panic!("Shouldn't be here"), // shouldn't be here
                    StsParty::Responder => {
                        // B sends the message Pbe || Ekey=Kb,IV=Sb||zero(sig)
                        let msg = self.msg2.clone();
                        if let Some(tx_msg) = msg {
                            let tx_msg = self.pack_plaintext_message(&tx_msg).unwrap(); // add crypto byte (and pass whitelist)
                            self.tx.construct_pprz_msg(&tx_msg); // append STX and checksum
                            self.stage = StsStage::WaitMsg3; // update status
                            return Some(self.tx.buf.clone()); // return a message ready to be sent
                        }
                        return None;
                    }
                }
            }
            StsStage::WaitMsg2 => {
                match self.party {
                    StsParty::Initiator => {
                        //println!("sts->my_private_key: {:?}", self.my_private_key);
                        //println!("sts->their_public_key: {:?}", self.their_public_key);
                        //println!("sts->my_private_ephemeral: {:?}", self.my_private_ephemeral);
                        //println!("sts->their_public_ephemeral: {:?}",self.their_public_ephemeral);
                        //println!("sts->rx_sym_key: {:?}", self.rx_sym_key);
                        //println!("sts->tx_sym_key: {:?}", self.tx_sym_key);

                        // A sends the message3: Ekey=Ka,IV=Sa||zero(sig)
                        let msg = self.msg3.clone();
                        if let Some(tx_msg) = msg {
                            let tx_msg = self.pack_plaintext_message(&tx_msg).unwrap(); // add crypto byte (and pass whitelist)
                            self.tx.construct_pprz_msg(&tx_msg); // append STX and checksum
                            self.stage = StsStage::CryptoOK; // update status
                            println!("returning msg3, going ok");
                            return Some(self.tx.buf.clone()); // return a message ready to be sent
                        }

                        // otherwise just send msg2 again
                        let msg = self.msg1.clone();
                        if let Some(tx_msg) = msg {
                            let tx_msg = self.pack_plaintext_message(&tx_msg).unwrap(); // add crypto byte (and pass whitelist)
                            self.tx.construct_pprz_msg(&tx_msg); // append STX and checksum
                            println!("again returning msg1, waiting msg2");
                            return Some(self.tx.buf.clone()); // return a message ready to be sent
                        }
                        return None;
                    }
                    StsParty::Responder => panic!("Shouldn't be here"), // shouldn't be here
                }
            }
            StsStage::WaitMsg3 => {
                match self.party {
                    StsParty::Initiator => panic!("Shouldn't be here"), // shouldn't be here
                    StsParty::Responder => panic!("Shouldn't be here"), // shouldn't be here either
                }
            }
        }
        //None
    }

    /// Parse incoming message bytes (PPRZ_STX..CHCKSUM B) and returns a new decrypted message if it is available.
    /// Otherwise `None` is returned.
    /// While the status != Crypto_OK no message is returned, and all logic is handled internally.
    /// Returned message has format of
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    /// and can be directly processed by a parser
    pub fn parse_byte(&mut self, b: u8) -> Option<Vec<u8>> {
        match self.stage {
            StsStage::CryptoOK => {
                if self.rx.parse_byte(b) {
                    // parse the byte
                    // we just got a message
                    let b = self.rx.buf.clone(); // clone the buffer
                    self.rx.reset(); // reset the transport
                    match self.decrypt_message(&b) {
                        // attempt decryption
                        Ok(v) => return Some(v), // return payload
                        Err(e) => {
                            // log errors and return nothing
                            self.last_error = StsError::ONGOING_COMM_ERROR;
                            self.last_error_message = e;
                            return None;
                        }
                    }
                } else {
                    return None;
                }
            }
            StsStage::Init => {
                return None;
            } // do nothing and wait for init
            StsStage::WaitMsg1 => {
                match self.party {
                    StsParty::Initiator => panic!("Shouldn't be here"), // shouldn't be here
                    StsParty::Responder => {
                        // process msg1
                        if self.rx.parse_byte(b) {
                            // we just got a message
                            let b = self.rx.buf.clone(); // clone the buffer
                            self.rx.reset(); // reset the transport
                            match self.decrypt_message(&b) {
                                // attempt decryption
                                Ok(v) => {
                                    // we have a message
                                    match self.sts_process_mgs1(&v) {
                                        Ok(_) => {
                                            // if OK, everything was handled in the underlying function
                                            // so do nothing
                                            return None;
                                        }
                                        Err(e) => {
                                            // log errors and return nothing
                                            self.last_error = StsError::MSG1_ERROR;
                                            self.last_error_message = e;
                                            return None;
                                        }
                                    }
                                }
                                Err(e) => {
                                    // log errors and return nothing
                                    self.last_error = StsError::ONGOING_COMM_ERROR;
                                    self.last_error_message = e;
                                    return None;
                                }
                            }
                        } else {
                            return None;
                        }
                    }
                }
            }
            StsStage::WaitMsg2 => {
                match self.party {
                    StsParty::Initiator => {
                        // process msg2
                        if self.rx.parse_byte(b) {
                            //println!("just got a new message");
                            let b = self.rx.buf.clone(); // clone the buffer
                                                         //println!("buf: {:?}", b);
                            self.rx.reset(); // reset the transport
                            match self.decrypt_message(&b) {
                                // attempt decryption
                                Ok(v) => {
                                    // we have a message
                                    println!("message successfully decrypted");
                                    match self.sts_process_mgs2(&v) {
                                        Ok(_) => {
                                            // if OK, everything was handled in the underlying function
                                            // so do nothing
                                            println!("processed msg2");
                                            return None;
                                        }
                                        Err(e) => {
                                            println!("we have an error: {}", e);
                                            // log errors and return nothing
                                            self.last_error = StsError::MSG2_ERROR;
                                            self.last_error_message = e;
                                            return None;
                                        }
                                    }
                                }
                                Err(e) => {
                                    // log errors and return nothing
                                    //println!("message wasn't decrypted");
                                    //println!("last error: {}", e.clone());
                                    self.last_error = StsError::ONGOING_COMM_ERROR;
                                    self.last_error_message = e;
                                    return None;
                                }
                            }
                        } else {
                            return None;
                        }
                    }
                    StsParty::Responder => panic!("Shouldn't be here"), // shouldn't be here
                }
            }
            StsStage::WaitMsg3 => {
                match self.party {
                    StsParty::Initiator => panic!("Shouldn't be here"), // shouldn't be here
                    StsParty::Responder => {
                        // process msg3
                        if self.rx.parse_byte(b) {
                            let b = self.rx.buf.clone(); // clone the buffer
                            self.rx.reset(); // reset the transport
                            match self.decrypt_message(&b) {
                                // attempt decryption
                                Ok(v) => {
                                    // we have a message
                                    match self.sts_process_mgs3(&v) {
                                        Ok(_) => {
                                            // if OK, we are ready for an ongoing communication
                                            self.stage = StsStage::CryptoOK; // update status
                                            return None;
                                        }
                                        Err(e) => {
                                            // log errors and return nothing
                                            self.last_error = StsError::MSG3_ERROR;
                                            self.last_error_message = e;
                                            return None;
                                        }
                                    }
                                }
                                Err(e) => {
                                    // log errors and return nothing
                                    self.last_error = StsError::ONGOING_COMM_ERROR;
                                    self.last_error_message = e;
                                    return None;
                                }
                            }
                        } else {
                            return None;
                        }
                    }
                }
            }
        }
    }

    /// Generate ephemeral curve25519 key pair (Pbe, Qbe).
    /// and store the keypair as `my_private_ephemeral`
    fn sts_ephemeral_curve25519_key_pair(&mut self) {
        // ephemeral keys
        let mut q_e = vec![0; PPRZ_KEY_LEN];
        let mut p_e = vec![0; PPRZ_KEY_LEN];
        let mut basepoint: [u8; PPRZ_KEY_LEN] = [0; PPRZ_KEY_LEN];
        basepoint[0] = PPRZ_CURVE_BASEPOINT;

        let mut rng = OsRng::new().unwrap(); // get a RNG
        rng.fill_bytes(q_e.as_mut_slice()); // generate random Q_ae
        assert_eq!(
            curve25519_crypto_scalarmult(p_e.as_mut_slice(), q_e.as_slice(), &basepoint),
            Ok(())
        );
        self.my_private_ephemeral.set(&q_e, &p_e).unwrap(); // update my private ephemeral key
    }

    /// NOTE: for INITIATOR party only
    /// First step in STS protocol
    /// Party `A` generates an ephemeral (random) curve25519 key pair (Pae, Qae) and sends Pae.
    /// generate public and private keys
    /// Returns the message as:
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    fn sts_initiate_msg(&mut self) -> Vec<u8> {
        assert!(self.my_private_ephemeral.is_ready());
        assert!(self.my_private_key.is_ready());
        assert!(self.their_public_key.is_ready());

        // access the dictionary and create a message.
        let mut msg1;
        match self.dictionary {
            Some(ref dict) => {
                msg1 = dict.find_msg_by_name(&"KEY_EXCHANGE_GCS").unwrap();
            }
            None => panic!("Error: Dictionary not provided!"),
        }

        // update message
        msg1.set_sender(self.sender_id);
        msg1.set_destinaton(self.destination_id);
        msg1.update_single_field("msg_type", PprzMsgBaseType::Uint8(StsMsgType::P_AE as u8));
        msg1.update_single_field(
            "msg_data",
            PprzMsgBaseType::Uint8Arr(self.my_private_ephemeral.pubkey.to_vec()),
        );

        // make sure the message is in the whitelist
        if !self.allowed_msg_ids.contains(&msg1.id) {
            self.allowed_msg_ids.push(msg1.id);
        }

        return msg1.to_bytes();
    }

    /// NOTE: for RESPONDER party only
    /// Process incoming message (expected MSG1)
    /// if the right (KEY_EXCHANGE) message received with the right data (P_AE)
    /// and the right P_AE.len=PPRZ_KEY_LEN, the internal state of Sts gets updated
    /// (key derivation etc), and msg2 is prepared to be sent.
    /// Input: decrypted message (source_ID .. msg payload)
    /// Returns either `Ok()` if the message2 was sucessfully prepared or an Error string
    ///
    /// Note the generated message has a format of:
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    fn sts_process_mgs1(&mut self, payload: &[u8]) -> Result<(), String> {
        assert!(self.my_private_ephemeral.is_ready());
        assert!(self.my_private_key.is_ready());
        assert!(self.their_public_key.is_ready());
        // check if the incoming message is really KEY_EXCHANGE_GCS
        let mut msg;
        match self.dictionary {
            Some(ref dict) => {
                let name = dict.get_msg_name(
                    self.rx_msg_class,
                    PprzMessage::get_msg_id_from_buf(payload, dict.protocol),
                ).expect("thread main: message name not found");
                if name != "KEY_EXCHANGE_GCS" {
                    let s = String::from("Error, received message is not KEY_EXCHANGE_GCS, but ")
                        + &name;
                    return Err(s);
                }
                msg = dict.find_msg_by_name(&name)
                    .expect("thread main: no message found");
                // update message fields with real values
                msg.update(payload);
            }
            None => panic!("Error: Dictionary not provided!"),
        }

        // check if the message has the correct type and value
        match msg.get_single_field("msg_type") {
            Some(f) => {
                // see if it is P_AE and pass though if it is
                if let PprzMsgBaseType::Uint8(a) = f {
                    if a != StsMsgType::P_AE as u8 {
                        return Err(String::from("Error: msg_type != P_AE"));
                    }
                } else {
                    return Err(String::from("Error: msg_type != PprzMsgBaseType::Uint8(a)"));
                }
            }
            None => {
                return Err(String::from("Error: message doesn't have 'msg_type' field"));
            }
        }

        //  check if the message data are the correct length and value
        match msg.get_single_field("msg_data") {
            Some(f) => {
                // see if it is uint8[] and pass though if it is
                if let PprzMsgBaseType::Uint8Arr(a) = f {
                    if a.len() != PPRZ_KEY_LEN {
                        return Err(String::from("Error: msg_data.len != PPRZ_KEY_LEN"));
                    } else {
                        // all good
                        self.their_public_ephemeral.set(&a).unwrap();
                    }
                } else {
                    return Err(String::from(
                        "Error: msg_data != PprzMsgBaseType::Uint8Arr(a)",
                    ));
                }
            }
            None => {
                return Err(String::from("Error: message doesn't have 'msg_data' field"));
            }
        }
        assert!(self.their_public_ephemeral.is_ready());

        // 3. B computes the shared secret: z = scalar_multiplication(Qbe, Pae)
        let mut z = vec![0; PPRZ_KEY_LEN];
        assert!(self.my_private_ephemeral.is_ready());
        assert_eq!(
            curve25519_crypto_scalarmult(
                z.as_mut_slice(),
                &self.my_private_ephemeral.privkey,
                &self.their_public_ephemeral.pubkey,
            ),
            Ok(())
        );

        // 4. B uses the key derivation function kdf(z,1) to compute Kb || Sb, kdf(z,0) to
        // compute Ka || Sa.
        // kdf(z,partyIdent) = SHA512( 0 || z || partyIdent)
        // (0 for A, 1 for B)

        // kdf(z,0) to compute Ka || Sa
        let mut ka_sa = vec![0; PPRZ_HASH_LEN];
        let mut input = z.clone();
        input.push(StsParty::Initiator as u8);
        assert_eq!(
            sha2_512_hash(ka_sa.as_mut_slice(), input.as_slice()),
            Ok(())
        );

        // update RX key
        self.rx_sym_key
            .set(
                &ka_sa[0..PPRZ_KEY_LEN],                             // key
                &ka_sa[PPRZ_KEY_LEN..PPRZ_KEY_LEN + PPRZ_NONCE_LEN], // nonce
                0,                                                   // set counter to zero
            )
            .unwrap(); // shouldn't fail

        // kdf(z,1) to compute Kb || Sb
        let mut kb_sb = vec![0; PPRZ_HASH_LEN];
        let mut input = z.clone();
        input.push(StsParty::Responder as u8);
        assert_eq!(
            sha2_512_hash(kb_sb.as_mut_slice(), input.as_slice()),
            Ok(())
        );

        // update TX key
        self.tx_sym_key
            .set(
                &kb_sb[0..PPRZ_KEY_LEN],                             // key
                &kb_sb[PPRZ_KEY_LEN..PPRZ_KEY_LEN + PPRZ_NONCE_LEN], // nonce
                0,                                                   // set counter to zero
            )
            .unwrap(); // shouldn't fail

        // 5. B computes the ed25519 signature: sig = signQb(Pbe || Pae)
        let mut sig = vec![0; PPRZ_SIGN_LEN];
        let mut pbe_pae = vec![];
        pbe_pae.extend_from_slice(&self.my_private_ephemeral.pubkey);
        pbe_pae.extend_from_slice(&self.their_public_ephemeral.pubkey);
        assert_eq!(
            ed25519_sign(
                sig.as_mut_slice(),
                &self.my_private_key.privkey,
                pbe_pae.as_slice(),
            ),
            Ok(())
        );

        // 6. B computes and sends the message Pbe || Ekey=Kb,IV=Sb||zero(sig);
        // update intermediate fields
        let auth = vec![];
        let mut ciphertext = vec![0; PPRZ_SIGN_LEN];
        let mut tag: [u8; PPRZ_MAC_LEN] = [0; PPRZ_MAC_LEN];
        let mut msg_data =
            Vec::with_capacity(PPRZ_KEY_LEN + ciphertext.len() + PPRZ_CRYPTO_OVERHEAD);
        match chacha20poly1305_aead_encrypt(
            &mut ciphertext,
            &mut tag,
            &sig,
            &auth,
            &self.tx_sym_key.key,
            &self.tx_sym_key.nonce,
        ) {
            Ok(val) => {
                if val {
                    msg_data.extend_from_slice(&self.my_private_ephemeral.pubkey); // P_be
                    msg_data.extend_from_slice(&ciphertext); // encrypted signature
                    msg_data.extend_from_slice(&tag); // tag
                }
            }
            Err(msg) => return Err(msg),
        };

        assert_eq!(msg_data.len(), PPRZ_KEY_LEN + PPRZ_SIGN_LEN + PPRZ_MAC_LEN);

        // access the dictionary and create a message.
        let mut msg2;
        match self.dictionary {
            Some(ref dict) => {
                msg2 = dict.find_msg_by_name(&"KEY_EXCHANGE_UAV").unwrap();
            }
            None => panic!("Error: Dictionary not provided!"),
        }

        // update message
        msg2.set_sender(self.sender_id);
        msg2.set_destinaton(self.destination_id);
        msg2.update_single_field("msg_type", PprzMsgBaseType::Uint8(StsMsgType::P_BE as u8));
        msg2.update_single_field("msg_data", PprzMsgBaseType::Uint8Arr(msg_data));

        // make sure the message is in the whitelist
        if !self.allowed_msg_ids.contains(&msg2.id) {
            self.allowed_msg_ids.push(msg2.id);
        }

        // save for later use
        self.msg2 = Some(msg2.to_bytes());

        Ok(())
    }

    /// NOTE: for INITIATOR party only
    /// Process incoming message (expected MSG2)
    /// if the right (KEY_EXCHANGE) message received with the right data (P_BE)
    /// and the right P_BE.len=PPRZ_KEY_LEN+PPRZ_SIGN_LEN+PPRZ_MAC_LEN, the internal state of Sts gets updated
    /// (key derivation etc), and msg3 is prepared to be sent.
    /// Input: decrypted message (source_ID .. msg payload)
    /// Returns either `Ok()` if the message3 was sucessfully prepared or an Error string
    ///
    /// Note the generated message has a format of:
    /// Pprzlink 2.0
    /// ```ignore
    /// payload[0] source SENDER_ID
    /// payload[1] destination ID
    /// payload[2] class/component
    /// payload[3] MSG_ID
    /// payload[4-end] MSG_PAYLOAD
    /// ```
    fn sts_process_mgs2(&mut self, payload: &[u8]) -> Result<(), String> {
        assert!(self.my_private_key.is_ready());
        assert!(self.their_public_key.is_ready());
        assert!(self.my_private_ephemeral.is_ready());
        // check if the incoming message is really KEY_EXCHANGE_UAV
        let mut msg;
        match self.dictionary {
            Some(ref dict) => {
                let name = dict.get_msg_name(
                    self.rx_msg_class,
                    PprzMessage::get_msg_id_from_buf(payload, dict.protocol),
                ).expect("thread main: message name not found");
                if name != "KEY_EXCHANGE_UAV" {
                    let s = String::from("Error, received message is not KEY_EXCHANGE_UAV, but ")
                        + &name;
                    return Err(s);
                }
                msg = dict.find_msg_by_name(&name)
                    .expect("thread main: no message found");
                // update message fields with real values
                msg.update(payload);
            }
            None => panic!("Error: Dictionary not provided!"),
        }

        // check if the message has the correct type and value
        match msg.get_single_field("msg_type") {
            Some(f) => {
                // see if it is P_AE and pass though if it is
                if let PprzMsgBaseType::Uint8(a) = f {
                    if a != StsMsgType::P_BE as u8 {
                        return Err(String::from("Error: msg_type != P_BE"));
                    }
                } else {
                    return Err(String::from("Error: msg_type != PprzMsgBaseType::Uint8(a)"));
                }
            }
            None => {
                return Err(String::from("Error: message doesn't have 'msg_type' field"));
            }
        }

        //  check if the message data are the correct length and value
        let msg2;
        match msg.get_single_field("msg_data") {
            Some(f) => {
                // see if it is uint8[] and pass though if it is
                if let PprzMsgBaseType::Uint8Arr(a) = f {
                    if a.len() != (PPRZ_KEY_LEN + PPRZ_SIGN_LEN + PPRZ_MAC_LEN) {
                        return Err(String::from(
                            "Error: msg_data.len != (PPRZ_KEY_LEN+PPRZ_SIGN_LEN+PPRZ_MAC_LEN)",
                        ));
                    } else {
                        // all good
                        msg2 = a;
                    }
                } else {
                    return Err(String::from(
                        "Error: msg_data != PprzMsgBaseType::Uint8Arr(a)",
                    ));
                }
            }
            None => {
                return Err(String::from("Error: message doesn't have 'msg_data' field"));
            }
        }
        self.their_public_ephemeral
            .set(&msg2[0..PPRZ_KEY_LEN])
            .unwrap();
        assert!(self.their_public_ephemeral.is_ready());

        // 7. A computes the shared secret: z = scalar_mseultiplication(Qae, Pbe)
        let mut z = vec![0; PPRZ_KEY_LEN];
        assert_eq!(
            curve25519_crypto_scalarmult(
                z.as_mut_slice(),
                &self.my_private_ephemeral.privkey,
                &self.their_public_ephemeral.pubkey,
            ),
            Ok(())
        );

        // 8. A uses the key derivation function kdf(z,1) to compute Kb || Sb, kdf(z,0) to
        // compute Ka || Sa.
        // kdf(z,partyIdent) = SHA512( 0 || z || partyIdent)
        // (0 for A, 1 for B)

        // kdf(z,0) to compute Ka || Sa
        let mut ka_sa = vec![0; PPRZ_HASH_LEN];
        let mut input = z.clone();
        input.push(StsParty::Initiator as u8);
        assert_eq!(
            sha2_512_hash(ka_sa.as_mut_slice(), input.as_slice()),
            Ok(())
        );

        // update TX key
        self.tx_sym_key
            .set(
                &ka_sa[0..PPRZ_KEY_LEN],                             // key
                &ka_sa[PPRZ_KEY_LEN..PPRZ_KEY_LEN + PPRZ_NONCE_LEN], // nonce
                0,                                                   // set counter to zero
            )
            .unwrap(); // shouldn't fail

        // kdf(z,1) to compute Kb || Sb
        let mut kb_sb = vec![0; PPRZ_HASH_LEN];
        let mut input = z.clone();
        input.push(StsParty::Responder as u8);
        assert_eq!(
            sha2_512_hash(kb_sb.as_mut_slice(), input.as_slice()),
            Ok(())
        );

        // update RX key
        self.rx_sym_key
            .set(
                &kb_sb[0..PPRZ_KEY_LEN],                             // key
                &kb_sb[PPRZ_KEY_LEN..PPRZ_KEY_LEN + PPRZ_NONCE_LEN], // nonce
                0,                                                   // set counter to zero
            )
            .unwrap(); // shouldn't fail

        // A decrypts the remainder of the message, verifies the signature.
        let auth = vec![];
        let ciphertext = &msg2[PPRZ_KEY_LEN..PPRZ_KEY_LEN + PPRZ_SIGN_LEN];
        let mut signature: [u8; PPRZ_SIGN_LEN] = [0; PPRZ_SIGN_LEN];
        let tag = &msg2[PPRZ_KEY_LEN + PPRZ_SIGN_LEN..PPRZ_KEY_LEN + PPRZ_SIGN_LEN + PPRZ_MAC_LEN];
        match chacha20poly1305_aead_decrypt(
            &mut signature,
            &tag,
            &ciphertext,
            &auth,
            &self.rx_sym_key.key,
            &self.rx_sym_key.nonce,
        ) {
            Ok(val) => {
                if val {
                    // decryption was sucessfull, verify the signature
                    // 9. A verifies the signature.");
                    let mut pbe_pae = vec![];
                    pbe_pae.extend_from_slice(&self.their_public_ephemeral.pubkey);
                    pbe_pae.extend_from_slice(&self.my_private_ephemeral.pubkey);

                    let success =
                        match ed25519_verify(&self.their_public_key.pubkey, &pbe_pae, &signature) {
                            Ok(val) => val,
                            Err(msg) => panic!("Error! {}", msg),
                        };
                    assert_eq!(success, true);
                    // A signature verified
                }
            }
            Err(msg) => return Err(msg),
        };

        // 10. A computes the ed25519 signature: sig = signQa(Pae || Pbe)
        let mut sig = vec![0; PPRZ_SIGN_LEN];
        let mut pae_pbe = vec![];
        pae_pbe.extend_from_slice(&self.my_private_ephemeral.pubkey);
        pae_pbe.extend_from_slice(&self.their_public_ephemeral.pubkey);
        assert_eq!(
            ed25519_sign(
                sig.as_mut_slice(),
                &self.my_private_key.privkey,
                pae_pbe.as_slice(),
            ),
            Ok(())
        );

        // 11. A computes and sends the message Ekey=Ka,IV=Sa||zero(sig)
        let auth = vec![];
        let mut ciphertext = vec![0; PPRZ_SIGN_LEN];
        let mut tag: [u8; PPRZ_MAC_LEN] = [0; PPRZ_MAC_LEN];
        let mut msg_data =
            Vec::with_capacity(PPRZ_KEY_LEN + ciphertext.len() + PPRZ_CRYPTO_OVERHEAD);
        match chacha20poly1305_aead_encrypt(
            &mut ciphertext,
            &mut tag,
            &sig,
            &auth,
            &self.tx_sym_key.key,
            &self.tx_sym_key.nonce,
        ) {
            Ok(val) => {
                if val {
                    msg_data.extend_from_slice(&ciphertext); // encrypted signature
                    msg_data.extend_from_slice(&tag); // tag
                }
            }
            Err(msg) => return Err(msg),
        };

        // access the dictionary and create a message.
        let mut msg3;
        match self.dictionary {
            Some(ref dict) => {
                msg3 = dict.find_msg_by_name(&"KEY_EXCHANGE_GCS").unwrap();
            }
            None => panic!("Error: Dictionary not provided!"),
        }

        // update message
        msg3.set_sender(self.sender_id);
        msg3.set_destinaton(self.destination_id);
        msg3.update_single_field("msg_type", PprzMsgBaseType::Uint8(StsMsgType::SIG as u8));
        msg3.update_single_field("msg_data", PprzMsgBaseType::Uint8Arr(msg_data));

        // make sure the message is in the whitelist
        if !self.allowed_msg_ids.contains(&msg3.id) {
            self.allowed_msg_ids.push(msg3.id);
        }

        // save for later use
        self.msg3 = Some(msg3.to_bytes());

        Ok(())
    }

    /// NOTE: for RESPONDER party only
    /// Process incoming message (expected MSG3)
    /// if the right (KEY_EXCHANGE) message received with the right data (SIG)
    /// and the right SIG.len=PPRZ_SIGN_LEN, and the signature is verified, Ok() is returned.
    /// Input: decrypted message (source_ID .. msg payload)
    /// Returns either `Ok()` if this party is ready for ongoing communication
    fn sts_process_mgs3(&mut self, payload: &[u8]) -> Result<(), String> {
        assert!(self.my_private_key.is_ready());
        assert!(self.their_public_key.is_ready());
        assert!(self.my_private_ephemeral.is_ready());
        assert!(self.their_public_ephemeral.is_ready());
        // check if the incoming message is really KEY_EXCHANGE_GCS
        let mut msg;
        match self.dictionary {
            Some(ref dict) => {
                let name = dict.get_msg_name(
                    self.rx_msg_class,
                    PprzMessage::get_msg_id_from_buf(payload, dict.protocol),
                ).expect("thread main: message name not found");
                if name != "KEY_EXCHANGE_GCS" {
                    let s = String::from("Error, received message is not KEY_EXCHANGE_GCS, but ")
                        + &name;
                    return Err(s);
                }
                msg = dict.find_msg_by_name(&name)
                    .expect("thread main: no message found");
                // update message fields with real values
                msg.update(payload);
            }
            None => panic!("Error: Dictionary not provided!"),
        }

        // check if the message has the correct type and value
        match msg.get_single_field("msg_type") {
            Some(f) => {
                // see if it is SIG and pass though if it is
                if let PprzMsgBaseType::Uint8(a) = f {
                    if a != StsMsgType::SIG as u8 {
                        return Err(String::from("Error: msg_type != SIG"));
                    }
                } else {
                    return Err(String::from("Error: msg_type != PprzMsgBaseType::Uint8(a)"));
                }
            }
            None => {
                return Err(String::from("Error: message doesn't have 'msg_type' field"));
            }
        }

        //  check if the message data are the correct length and value
        let msg3;
        match msg.get_single_field("msg_data") {
            Some(f) => {
                // see if it is uint8[] and pass though if it is
                if let PprzMsgBaseType::Uint8Arr(a) = f {
                    assert_eq!(a.len(), (PPRZ_SIGN_LEN + PPRZ_MAC_LEN));
                    if a.len() != (PPRZ_SIGN_LEN + PPRZ_MAC_LEN) {
                        return Err(String::from(
                            "Error: msg_data.len != PPRZ_SIG_LEN + PPRZ_MAC_LEN",
                        ));
                    } else {
                        // all good
                        msg3 = a;
                    }
                } else {
                    return Err(String::from(
                        "Error: msg_data != PprzMsgBaseType::Uint8Arr(a)",
                    ));
                }
            }
            None => {
                return Err(String::from("Error: message doesn't have 'msg_data' field"));
            }
        }

        // 13. B decrypts the message and verifies the signature.
        let auth = vec![];
        let ciphertext = &msg3[0..PPRZ_SIGN_LEN];
        let mut signature: [u8; PPRZ_SIGN_LEN] = [0; PPRZ_SIGN_LEN];
        let tag = &msg3[PPRZ_SIGN_LEN..PPRZ_SIGN_LEN + PPRZ_MAC_LEN];
        match chacha20poly1305_aead_decrypt(
            &mut signature,
            &tag,
            &ciphertext,
            &auth,
            &self.rx_sym_key.key,
            &self.rx_sym_key.nonce,
        ) {
            Ok(val) => {
                if val {
                    // decryption was sucessfull, verify the signature
                    let mut pae_pbe = vec![];
                    pae_pbe.extend_from_slice(&self.their_public_ephemeral.pubkey);
                    pae_pbe.extend_from_slice(&self.my_private_ephemeral.pubkey);

                    let success =
                        match ed25519_verify(&self.their_public_key.pubkey, &pae_pbe, &signature) {
                            Ok(val) => val,
                            Err(msg) => panic!("Error! {}", msg),
                        };
                    assert_eq!(success, true);
                    // signature verified
                }
            }
            Err(msg) => return Err(msg),
        };

        Ok(())
    }
}

/// Private key container
/// Contains both public key P_a and
/// private key Q_a
#[derive(Debug)]
pub struct GecPrivKey {
    privkey: [u8; PPRZ_KEY_LEN],
    pubkey: [u8; PPRZ_KEY_LEN],
    ready: bool,
}

impl GecPrivKey {
    pub fn new() -> GecPrivKey {
        let q = [0; PPRZ_KEY_LEN];
        let p = [0; PPRZ_KEY_LEN];
        GecPrivKey {
            privkey: q,
            pubkey: p,
            ready: false,
        }
    }

    pub fn is_ready(&self) -> bool {
        return self.ready;
    }

    #[allow(dead_code)]
    pub fn set(&mut self, q: &[u8], p: &[u8]) -> Result<(), String> {
        if q.len() != PPRZ_KEY_LEN {
            return Err(String::from(
                "GecPrivKey update Error: q.len() != PPRZ_KEY_LEN",
            ));
        }
        if p.len() != PPRZ_KEY_LEN {
            return Err(String::from(
                "GecPrivKey update Error: p.len() != PPRZ_KEY_LEN",
            ));
        }

        self.privkey.clone_from_slice(q);
        self.pubkey.clone_from_slice(p);
        self.ready = true;

        Ok(())
    }
}

/// Public key container
/// Contains public key P_b
#[derive(Debug)]
pub struct GecPubKey {
    pubkey: [u8; PPRZ_KEY_LEN],
    ready: bool,
}

impl GecPubKey {
    //#[allow(dead_code)]
    pub fn new() -> GecPubKey {
        let p = [0; PPRZ_KEY_LEN];
        GecPubKey {
            pubkey: p,
            ready: false,
        }
    }

    //#[allow(dead_code)]
    pub fn is_ready(&self) -> bool {
        return self.ready;
    }

    #[allow(dead_code)]
    pub fn set(&mut self, p: &[u8]) -> Result<(), String> {
        if p.len() != PPRZ_KEY_LEN {
            return Err(String::from(
                "GecPubKey update Error: p.len() != PPRZ_KEY_LEN",
            ));
        }
        self.pubkey.clone_from_slice(p);
        self.ready = true;

        Ok(())
    }
}

/// Party type
#[derive(Debug)]
pub enum StsParty {
    Initiator = 0,
    Responder = 1,
}

/// Stage of station-to-station key exchange
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum StsStage {
    Init,
    WaitMsg1,
    WaitMsg2,
    WaitMsg3,
    CryptoOK,
}

/// key exchange message type
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum StsMsgType {
    P_AE = 0,
    P_BE = 1,
    SIG = 2,
}

/// key exchange error
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum StsError {
    ERROR_NONE,
    // RESPONDER ERRORS
    MSG1_ERROR,
    MSG1_TIMEOUT_ERROR,
    MSG1_ENCRYPT_ERROR,
    MSG3_ERROR,
    MSG3_TIMEOUT_ERROR,
    MSG3_DECRYPT_ERROR,
    MSG3_SIGNVERIFY_ERROR,
    // INITIATOR ERRORS
    MSG2_ERROR,
    MSG2_TIMEOUT_ERROR,
    MSG2_DECRYPT_ERROR,
    MSG2_SIGNVERIFY_ERROR,
    MSG3_ENCRYPT_ERROR,
    // BOTH PARTIES
    UNEXPECTED_MSG_TYPE_ERROR,
    UNEXPECTED_STS_STAGE_ERROR,
    UNEXPECTED_MSG_ERROR,
    ONGOING_COMM_ERROR,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decryption() {
        let mut trans = SecurePprzTransport::new(StsParty::Initiator, 0);
        assert_eq!(trans.rx_sym_key.set(&KEY, &NONCE, 0), Ok(()));

        let mut buf = vec![PPRZ_MSG_TYPE_ENCRYPTED];
        buf.extend_from_slice(&NONCE[0..4]); // counter
        buf.extend_from_slice(&AUTH); // auth data
        buf.extend_from_slice(&CIPHERTEXT); // encrypted payload
        buf.extend_from_slice(&MAC); // tag

        println!("Payload to be decrypted: {:?}", buf);
        assert_eq!(trans.decrypt_message(&buf).unwrap(), [1, 2, 4, 3]);
    }

    #[test]
    fn test_encryption() {
        let mut trans = SecurePprzTransport::new(StsParty::Initiator, 0);
        let cntr = &NONCE[0..4];
        let cntr = pprzlink_bytes_to_counter(cntr).unwrap();
        assert_eq!(trans.tx_sym_key.set(&KEY, &NONCE, cntr - 1), Ok(()));

        let mut buf_plaintext: Vec<u8> = vec![];
        buf_plaintext.extend_from_slice(&AUTH);
        buf_plaintext.extend_from_slice(&PLAINTEXT);

        let mut buf = vec![PPRZ_MSG_TYPE_ENCRYPTED];
        buf.extend_from_slice(&NONCE[0..4]); // counter
        buf.extend_from_slice(&AUTH); // auth data
        buf.extend_from_slice(&CIPHERTEXT); // encrypted payload
        buf.extend_from_slice(&MAC); // tag

        println!("Payload: {:?}", buf_plaintext);
        assert_eq!(trans.encrypt_message(buf_plaintext.as_ref()).unwrap(), buf);
    }

    #[test]
    #[should_panic]
    fn test_incorrect_decryption() {
        let mut trans = SecurePprzTransport::new(StsParty::Initiator, 0);
        assert_eq!(trans.rx_sym_key.set(&KEY, &NONCE, 0), Ok(()));

        let mut buf = vec![PPRZ_MSG_TYPE_ENCRYPTED];
        buf.extend_from_slice(&NONCE[4..8]); // counter -> provide incorrect counter for the test
        buf.extend_from_slice(&AUTH); // auth data
        buf.extend_from_slice(&CIPHERTEXT); // encrypted payload
        buf.extend_from_slice(&MAC); // tag

        trans.decrypt_message(&buf).unwrap();
    }

    #[test]
    fn test_plaintext() {
        let mut trans = SecurePprzTransport::new(StsParty::Initiator, 0);
        trans.allowed_msg_ids.push(3);

        let mut buf = vec![PPRZ_MSG_TYPE_PLAINTEXT];
        buf.extend_from_slice(&AUTH); // auth data
        buf.extend_from_slice(&PLAINTEXT); // encrypted payload

        assert_eq!(trans.decrypt_message(&buf).unwrap(), [1, 2, 4, 3]);
    }

    static KEY: [u8; 32] = [
        0x70, 0x03, 0xAA, 0x0A, 0x8E, 0xE9, 0xA8, 0xFF, 0xD5, 0x46, 0x1E, 0xEC, 0x7C, 0xC1, 0xC1,
        0xA1, 0x6A, 0x43, 0xC9, 0xD4, 0xB3, 0x2B, 0x94, 0x7E, 0x76, 0xF9, 0xD8, 0xE8, 0x1A, 0x31,
        0x5D, 0xA8,
    ];

    static CIPHERTEXT: [u8; 2] = [0xa1, 0x7b];
    static PLAINTEXT: [u8; 2] = [4, 3];
    static AUTH: [u8; 2] = [1, 2];
    static MAC: [u8; 16] = [
        0x83, 0xf6, 0x95, 0x66, 0x4a, 0xa4, 0x82, 0x82, 0x12, 0xf0, 0x7f, 0xa1, 0xf, 0x92, 0x86,
        0xea,
    ];
    static NONCE: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc];
}
