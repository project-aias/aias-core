use super::{
    destroy_ios, generate_check_parameter_ios, generate_ready_parameter_ios, new_ios,
    set_subset_ios, unblind_ios,
};

use aias_core::signer::Signer;
use aias_core::verifyer;

use aias_core::utils;

use std::os::raw::c_uint;


#[test]
fn test_init_and_destroy() {
    let pk1 = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxXo2zWkciUEZBcm/Exk8
Zac8NWskP59EAVFlO218xIXOV0FfphPB/tnbQh7GDXddo7XVEptHdHXyJlXXLihb
9vXbUZF2NDFLOhgDv7pa72VNLbw+jKR/FlsDtwv/bv7ZDqq+n79uavuJ8giX3qCf
+mtBmro7hG5AVve3JImhvA0FvTKJ0xCYUYw02st08He5RwFAXQK8G2cwahp+5ECH
MDdfFUaoxMfRN/+Hl9iqiJovKUJQ3545N2fDYdd0eqSlqL1N5xJxYX1GDMtGZgME
hHR6ntdfm7r43HDB4hk/MJIsNay6+K9tJBiz1qXG40G4NjMKzVrX9pi1Bv8G2RnP
/wIDAQAB
-----END PUBLIC KEY-----"#;

    let sk1 = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAxXo2zWkciUEZBcm/Exk8Zac8NWskP59EAVFlO218xIXOV0Ff
phPB/tnbQh7GDXddo7XVEptHdHXyJlXXLihb9vXbUZF2NDFLOhgDv7pa72VNLbw+
jKR/FlsDtwv/bv7ZDqq+n79uavuJ8giX3qCf+mtBmro7hG5AVve3JImhvA0FvTKJ
0xCYUYw02st08He5RwFAXQK8G2cwahp+5ECHMDdfFUaoxMfRN/+Hl9iqiJovKUJQ
3545N2fDYdd0eqSlqL1N5xJxYX1GDMtGZgMEhHR6ntdfm7r43HDB4hk/MJIsNay6
+K9tJBiz1qXG40G4NjMKzVrX9pi1Bv8G2RnP/wIDAQABAoIBAC3nRMnmvw1gpnJj
/Rhxa0qt3x8Dsr9fRC2SQBfaUYBVIivCNHukaBnXhlIOWTdUId4mLEtQ8QEvUYR7
u7MtCoOTjtGdIH7tXnE4l9Z/eRfg0lnpQhjrO+d0bJ6mGVAxyT7RjdIQa5hOtDgg
qzzC1a0eNXfEBoW4IxiUKGxD2eaeL2NEuwdysO8MrxvbpPLrK4KaQwansh5EdrvL
QmWtSSuB2rYVwWbp7Rs+NuKS4w7CRm9Zp6kN6yjBum+x2o3Wdj33Ww0HayeaRZ1i
nmVTyphfajKuDLYUavCo4tBE67LK/VHesxeFNM+6PjONUVmcRnT1eoATeLVE4vOO
M9kFUQECgYEA9WD+s2HpETsXxyWPp2wv1G8b2kZq0h85Vb971PwgRciHkBccHFHR
0Hgc/hFTHg86V0iVBbcsWtTNTsNbH7aXGNvWJVQiMPDNdKmavgAl3tpGRP7iffLF
503he0GQmVaDEBH4LqCi4Ix0u7wnOND9ie8hMzxtC+2cZyLY8y13o78CgYEAzgZu
JPMgD2BvSKDJYlP7j4OKj0+mQIdpW+ONuLZsbtTDs5GiggTeeeyQDvlESUMSypMj
rmS/GUHAnYft27YWjk48vlzrvrnLyzWLalGYLsUigQIf2BRJG43j8iXuuBKciOrf
P8dkByYXatkiA57CJXOJGJLPvMOfkr+p3i2L48ECgYA9eY52HIqKoZZkczmZRVZ6
T1fYCJpMiDwSCoYYpw3izcmAxPlq8uiw5NbGpEqBlmkUYv/KzchT/UpueC0FNfaG
6NSux3RFdJ7UooU9IsZaHa9LK9xMl50TRQS/n359nBn71bSq4d3MigPY4NumtV0/
yGQ19OaQ/XeYszdNPU/i+wKBgQCXSbeGIJaRVBJD9fYL43nd+A0+kZGW3xjqJh5C
3oqflFOlQDNiYKryQ1nB9R9E4SEiaowQGuENbfBAfbmX1o2XsDIA5AElTBAvx8D5
sLMc3RwqOeIibTsGJdqWTW6P8vLJxBduIT/90+XsS0gj+me80quAxQYRKmG6hE37
3dxUwQKBgQC2onc1n35vGlZ2HxWdnPOUyRnA8HNVXcskprR07ZqFQu36LxI8hHvz
O+zc6JPZDWBppJDWot9d5HeNEjDBMcSqcpeXXYU8XvxA+uECLPctLgNMWxyKFx95
0sVQIY0n9eLL7sg5aCUpGKf4Qc88wF8OPYnBzjCeiJusjkGhQ5rqdQ==
-----END RSA PRIVATE KEY-----"#;

    let signer_pubkey = pk1.to_string();
    let signer_privkey = sk1.to_string();

    let judge_pubkey = pk1.to_string();

    let signer_pubkey_c = utils::to_c_str(signer_pubkey.clone());
    let judge_pubkey_c = utils::to_c_str(judge_pubkey.to_string());
    let id = 10 as c_uint;

    new_ios(signer_pubkey_c, judge_pubkey_c, id);

    let message = utils::to_c_str("aaa".to_string());

    let ready_params = generate_ready_parameter_ios(message, judge_pubkey_c);
    let ready_params_str = utils::from_c_str(ready_params);

    let mut signer = Signer::new_with_blinded_digest(
        signer_privkey.clone(),
        signer_pubkey.clone(),
        ready_params_str,
        10,
    );

    let subset = signer.setup_subset();
    let subset = utils::to_c_str(subset.to_string());

    set_subset_ios(subset);

    let check_parameters = generate_check_parameter_ios();
    let check_parameters = utils::from_c_str(check_parameters);

    signer.check(check_parameters).unwrap();

    let blind_signature = signer.sign();
    let blind_signature = utils::to_c_str(blind_signature);
    let signature = unblind_ios(blind_signature);

    let signature = utils::from_c_str(signature);
    let message = utils::from_c_str(message);

    let signer_pubkey = utils::from_c_str(signer_pubkey_c);
    let judge_pubkey = utils::from_c_str(judge_pubkey_c);
    let result = verifyer::verify(signature, message, signer_pubkey, judge_pubkey.clone());

    assert_eq!(result, Ok(()));

    destroy_ios();
}
