extern crate hmac;
extern crate sha2;

use base64::{encode_config, URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

const SECRET: &str = include_str!("secret.key.sample");

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    sub: String,
    exp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    alg: String,
    typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    header: Header,
    payload: Payload,
    signature: String,
}

impl UserToken {
    pub fn generate_token() -> String {
        let header = Header {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
        };

        let header_json = serde_json::to_string(&header).unwrap();
        let header_b64 = encode_config(&header_json, URL_SAFE_NO_PAD);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let payload = Payload {
            sub: "user".to_string(),
            exp: now + 3600, // 1 hour from now
        };

        let payload_json = serde_json::to_string(&payload).unwrap();
        let payload_b64 = encode_config(&payload_json, URL_SAFE_NO_PAD);

        let msg = format!("{}.{}", header_b64, payload_b64);
        let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        mac.update(msg.as_bytes());

        let signature = mac.finalize().into_bytes();
        let signature_b64 = encode_config(&signature, URL_SAFE_NO_PAD);

        let jwt = format!("{}.{}.{}", header_b64, payload_b64, signature_b64);

        return jwt;
    }

    pub fn verify_token(token: &str) -> bool {
        let parts: Vec<&str> = token.split(".").collect();

        if (parts.len() != 3) || (parts[2].len() == 0) {
            return false;
        }

        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let signature_b64 = parts[2];

        let payload_bytes = match base64::decode(payload_b64) {
            Ok(bytes) => bytes,
            Err(error) => {
                println!("Error decoding base64: {}", error);
                Vec::new()
            }
        };

        let payload: Payload = match serde_json::from_slice(&payload_bytes) {
            Ok(payload) => payload,
            Err(error) => {
                println!("Error parsing JSON: {}", error);
                return false;
            }
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if payload.exp < now {
            return false;
        }

        let msg = format!("{}.{}", header_b64, payload_b64);
        let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        mac.update(msg.as_bytes());

        let computed_signature = mac.finalize().into_bytes();
        let computed_signature_b64 = encode_config(&computed_signature, URL_SAFE_NO_PAD);

        computed_signature_b64 == signature_b64
    }
}
