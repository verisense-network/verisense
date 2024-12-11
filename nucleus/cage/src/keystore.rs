use sp_core::crypto::{KeyTypeId, VrfCrypto, VrfPublic};
use sp_core::sr25519::{
    vrf::{VrfSignature, VrfTranscript},
    Public,
};
use sp_keystore::KeystorePtr;

pub(crate) fn sign_to_participate(
    keystore: KeystorePtr,
    key_type: KeyTypeId,
    public_input: &[u8],
) -> Result<VrfSignature, Box<dyn std::error::Error>> {
    let public = keystore
        .sr25519_public_keys(key_type)
        .first()
        .copied()
        .ok_or("No vrf key found, please insert one")?;
    let input = VrfTranscript::new(b"nucleus", &[(b"register", public_input)]).into();
    keystore
        .sr25519_vrf_sign(key_type, &public, &input)
        .transpose()
        .ok_or("Message signing failed")?
        .map_err(|e| e.into())
}
