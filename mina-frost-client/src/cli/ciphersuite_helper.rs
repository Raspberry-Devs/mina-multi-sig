use std::{error::Error, marker::PhantomData};

use frost_core::{
    keys::{KeyPackage, PublicKeyPackage},
    Ciphersuite,
};

use frost_bluepallas::BluePallas;
use mina_tx::pallas_message::{translate_pk, PallasMessage};

type BluePallasSuite = BluePallas<PallasMessage>;

/// Additional information about a group, derived from the key packages.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    pub hex_verifying_key: String,
    pub mina_verifying_key: String,
    pub threshold: usize,
    pub num_participants: usize,
}

/// A trait that helps obtaining ciphersuite-dependent information.
pub trait CiphersuiteHelper<C: Ciphersuite> {
    fn group_info(
        &self,
        encoded_key_package: &[u8],
        encoded_public_key_package: &[u8],
    ) -> Result<GroupInfo, Box<dyn Error>>;
}

/// An implementation of CiphersuiteHelper that works for any Ciphersuite.
struct CiphersuiteHelperImpl<C: Ciphersuite> {
    _phantom: PhantomData<C>,
}

impl<C> Default for CiphersuiteHelperImpl<C>
where
    C: Ciphersuite,
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

/// Get a CiphersuiteHelper for the given ciphersuite.
pub(crate) fn ciphersuite_helper<C: Ciphersuite>() -> Box<dyn CiphersuiteHelper<C>> {
    Box::new(CiphersuiteHelperImpl::<C>::default())
}

impl<C> CiphersuiteHelper<C> for CiphersuiteHelperImpl<C>
where
    C: Ciphersuite + 'static,
{
    fn group_info(
        &self,
        encoded_key_package: &[u8],
        encoded_public_key_package: &[u8],
    ) -> Result<GroupInfo, Box<dyn Error>> {
        let key_package: KeyPackage<C> = postcard::from_bytes(encoded_key_package)?;
        let public_key_package: PublicKeyPackage<BluePallasSuite> =
            postcard::from_bytes(encoded_public_key_package)?;
        let hex_verifying_key = hex::encode(public_key_package.verifying_key().serialize()?);
        let mina_verifying_key = translate_pk(public_key_package.verifying_key())?.into_address();

        Ok(GroupInfo {
            hex_verifying_key,
            mina_verifying_key,
            threshold: *key_package.min_signers() as usize,
            num_participants: public_key_package.verifying_shares().len(),
        })
    }
}
