use sp_core::crypto::KeyTypeId;
use sp_core::sr25519::{
    vrf::{VrfSignature, VrfTranscript},
    Public,
};
use sp_keystore::KeystorePtr;

pub(crate) fn sign_to_participate(
    keystore: KeystorePtr,
    key_type: KeyTypeId,
    public_input: &[u8],
) -> Result<(Public, VrfSignature), Box<dyn std::error::Error>> {
    let public = keystore
        .sr25519_public_keys(key_type)
        .first()
        .copied()
        .ok_or("No nvrf key found, please insert one")?;
    let input = VrfTranscript::new(b"nucleus", &[(b"register", public_input)]).into();
    Ok((
        public,
        keystore
            .sr25519_vrf_sign(key_type, &public, &input)
            .transpose()
            .ok_or("Message signing failed")??,
    ))
}

pub(crate) fn sign_tx(
    keystore: KeystorePtr,
    key_type: KeyTypeId,
    sign_payload: Vec<u8>,
) -> Result<[u8; 64], Box<dyn std::error::Error>> {
    let public = keystore
        .sr25519_public_keys(key_type)
        .first()
        .copied()
        .ok_or("No nvrf key found, please insert one")?;
    let signature = keystore
        .sr25519_sign(key_type, &public, &sign_payload)?
        .ok_or("fail to sign `register` tx signature, please check the keystore")?;
    Ok(signature.0)
}
