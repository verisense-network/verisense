use codec::Encode;
use sp_core::crypto::{KeyTypeId, VrfCrypto, VrfPublic};
use sp_core::sr25519::{
    vrf::{VrfSignature, VrfTranscript},
    Public,
};
use sp_keystore::KeystorePtr;
use sp_runtime::generic::SignedPayload;
use vrs_primitives::{Address, Signature};
use vrs_runtime::{RuntimeCall, SignedExtra};

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
    addr: Address,
    call: RuntimeCall,
    extra: SignedExtra,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let public = keystore
        .sr25519_public_keys(key_type)
        .first()
        .copied()
        .ok_or("No nvrf key found, please insert one")?;
    let raw_payload = SignedPayload::new(call, extra)
        .inspect_err(|e| {
            log::warn!("Unable to create signed payload: {:?}", e);
        })
        .map_err(|_| "unable to create signning payload")?;
    let to_be_signed = raw_payload.encode();
    let signature = keystore
        .sr25519_sign(key_type, &public, &to_be_signed)?
        .ok_or("no signature generated, please check the keystore")?;
    let (call, extra, _) = raw_payload.deconstruct();
    let signature = Signature::Sr25519(signature.0.into());
    Ok(signature)
}
