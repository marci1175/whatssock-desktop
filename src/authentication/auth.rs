use std::{fs, path::PathBuf};

use anyhow::Result;
use chacha20poly1305::{aead::{Aead, OsRng}, AeadCore, ChaCha20Poly1305, KeyInit};
use machineid_rs::{HWIDComponent, IdBuilder};
use sha2::{Digest, Sha256};

use crate::authentication::UserSession;

pub fn deserialize_into_user_session(json_input: String) -> Result<UserSession> {
    Ok(serde_json::from_str(&json_input)?)
}

pub fn store_user_session_on_disk(user_session: &UserSession, path: PathBuf) -> Result<()> {
    // Get HWID key to encrypt the user session with.
    let hwid = mid::data("hwid_key")?.result.concat();
    
    let serialized_bytes = rmp_serde::to_vec(&user_session)?;

    let hwid_key = Sha256::digest(hwid.as_bytes());
    
    let cipher = ChaCha20Poly1305::new(&hwid_key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // Encrypt the bytes
    let encrypted_user_session = cipher.encrypt(&nonce, serialized_bytes.as_slice()).map_err(|err| anyhow::Error::msg(err.to_string()))?;

    fs::write(&path, encrypted_user_session)?;

    Ok(())
}