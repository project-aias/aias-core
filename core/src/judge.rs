use crate::crypto::DistributedRSAPrivKey;

use fair_blind_signature::Judge;
use serde_json;

use distributed_rsa::PlainShareSet;
use rsa::{RSAPrivateKey, RSAPublicKey};
use std::cell::RefCell;

thread_local!(static ODB: RefCell<Option<Judge<DistributedRSAPrivKey>>> = RefCell::new(None));

pub struct ShareSet {
    pub share_set: PlainShareSet,
}

impl ShareSet {
    pub fn from_shares_vec(src: Vec<String>) -> serde_json::Result<Self> {
        let mut plain_shares = Vec::new();
        for share in &src {
            plain_shares.push(serde_json::from_str(share)?)
        }

        let share_set = PlainShareSet { plain_shares };
        Ok(ShareSet { share_set })
    }

    pub fn open_id(&self) -> Result<String, String> {
        let decrypted = self.share_set.decrypt();
        let decrypted_bytes = decrypted.to_bytes_le();
        let decrypted_str = String::from_utf8(decrypted_bytes)
            .map_err(|e| format!("failed to convert bytes into string: {}", e))?;

        decrypted_str
            .split(':')
            .next()
            .ok_or("failed to get ID part".to_string())
            .map(|v| v.to_string())
    }
}

pub fn divide_keys(prevkey: String, pubkey: String, count: u32) -> DistributedRSAPrivKey {
    let privkey = pem::parse(prevkey).expect("failed to parse pem");
    let privkey = RSAPrivateKey::from_pkcs1(&privkey.contents).expect("failed to parse pkcs1");

    let pubkey = pem::parse(pubkey).expect("failed to parse pem");
    let pubkey = RSAPublicKey::from_pkcs8(&pubkey.contents).expect("failed to parse pkcs8");

    DistributedRSAPrivKey::new(&privkey, &pubkey, count)
}

pub fn open(plain_shares: Vec<String>) -> Result<String, String> {
    let share_set = ShareSet::from_shares_vec(plain_shares)
        .map_err(|e| format!("failed to create share set: {}", e))?;
    share_set.open_id()
}
