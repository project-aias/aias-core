use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

use fair_blind_signature::{EJPubKey, FBSParameters, FBSSender, BlindedDigest, Subset};
use std::cell::{RefCell, RefMut}; 

use rand::rngs::OsRng;
use rsa::{BigUint, PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme, PublicKeyParts};


#[derive(Clone)]
struct TestCipherPubkey {}

impl EJPubKey for TestCipherPubkey {
    fn encrypt(&self, message: String) -> String {
        return message;
    }

    fn dencrypt(&self, message: String) -> String {
        return message;
    }
}



thread_local!(static ODB: RefCell<Option<FBSSender<TestCipherPubkey>>> = RefCell::new(None)); 

pub fn new() {
    let mut rng = OsRng;
    let bits = 2048;
    let signer_privkey = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let signer_pubkey = RSAPublicKey::from(&signer_privkey);

    let judge_pubkey = TestCipherPubkey {};

    let parameters = FBSParameters {
        signer_pubkey: signer_pubkey,
        judge_pubkey: judge_pubkey,
        k: 40,
        id: 10
    };

    ODB.with(|odb_cell| { 
        let mut odb = odb_cell.borrow_mut();
        *odb = Some(FBSSender::new(parameters));
    }); 
}

pub fn destroy() {
    ODB.with(|odb_cell| { 
        let mut odb = odb_cell.borrow_mut(); 
        *odb = None;
    }); 
}

pub fn blind(message: String) -> String {
    let mut serialized = "".to_string();

    ODB.with(|odb_cell| { 
        let mut odb = odb_cell.borrow_mut();
        let sender = odb.as_mut().unwrap();
        let (digest, _ , _, _) = sender.blind(message).unwrap();

        serialized = serde_json::to_string(&digest).unwrap();
    });

    serialized
}

pub fn set_subset(subset: String) {
    let subset: Subset = serde_json::from_str(&subset).expect("Parsing json error");

    ODB.with(|odb_cell| { 
        let mut odb = odb_cell.borrow_mut();
        let sender = odb.as_mut().unwrap();
        
        sender.set_subset(subset);
    });
}

#[no_mangle]
pub extern fn init_aias_ios(){
    new();
}

#[no_mangle]
pub extern fn init_aias_ios_free() {
    destroy();
}

#[no_mangle]
pub extern fn blind_dig_ios(to: *const c_char) -> *mut c_char{
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let result = blind(recipient.to_string());
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern fn blind_sig_ios_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[test]
fn test_init_and_destroy() {
    new();
    let result = blind("aaa".to_string());
    println!("{}", result);
    destroy();
}
