use crate::{
    api::Api,
    error::Error,
    types::{
        account_info::AccountInfo,
        extrinsic_params::{
            BaseExtrinsicParams, BaseExtrinsicParamsBuilder, ExtrinsicParams,
            GenericExtra, SignedPayload,
        },
        extrinsics::{GenericAddress, UncheckedExtrinsicV4},
    },
};
use codec::Compact;
use codec::Decode;
use codec::Encode;
use sp_core::{crypto::Pair, H256};
use sp_runtime::generic::Era;
use sp_runtime::{
    traits::IdentifyAccount, AccountId32, MultiSignature, MultiSigner,
};
use std::fmt;

impl Api {
    pub fn signer_account<P>(signer: &P) -> AccountId32
    where
        P: Pair,
        MultiSigner: From<P::Public>,
    {
        let multi_signer = MultiSigner::from(signer.public());
        multi_signer.into_account()
    }

    pub fn get_nonce_for_account(
        &self,
        account: &AccountId32,
    ) -> Result<u32, Error> {
        let account_info = self.get_account_info(account)?;
        match account_info {
            None => Ok(0),
            Some(account_info) => Ok(account_info.nonce),
        }
    }

    pub fn get_nonce<P>(&self, signer: &P) -> Result<u32, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
    {
        let signer_account = Self::signer_account(signer);
        self.get_nonce_for_account(&signer_account)
    }

    pub fn get_account_info(
        &self,
        account_id: &AccountId32,
    ) -> Result<Option<AccountInfo>, Error> {
        self.fetch_storage_map("System", "Account", account_id)
    }

    pub fn pallet_call_index(
        &self,
        pallet_name: &str,
        call_name: &str,
    ) -> Result<[u8; 2], Error> {
        Ok(self.metadata().pallet_call_index(pallet_name, call_name)?)
    }

    pub fn unsigned_extrinsic<Call>(
        &self,
        call: Call,
    ) -> UncheckedExtrinsicV4<Call>
    where
        Call: Encode,
    {
        UncheckedExtrinsicV4::new_unsigned(call)
    }

    pub fn compose_payload<Call>(
        &self,
        call: Call,
        extra: GenericExtra,
        head_hash: Option<H256>,
    ) -> Result<SignedPayload<Call>, Error>
    where
        Call: Encode + Clone + fmt::Debug,
    {
        let raw_payload: SignedPayload<Call> = SignedPayload::from_raw(
            call,
            extra,
            (
                self.runtime_version.spec_version,
                self.runtime_version.transaction_version,
                self.genesis_hash,
                head_hash.unwrap_or(self.genesis_hash),
                (),
                (),
                (),
            ),
        );
        Ok(raw_payload)
    }

    /// sign a bytes with the specified signer
    /// TODO: This should call an external API for the runtime
    /// otherwise, this api acts as if it is a wallet
    pub fn sign_message<P>(signer: &P, payload: &[u8]) -> P::Signature
    where
        P: Pair,
    {
        signer.sign(payload)
    }

    /// submit the extrinsic into the node
    pub fn submit_extrinsic<Call>(
        &self,
        xt: UncheckedExtrinsicV4<Call>,
    ) -> Result<Option<H256>, Error>
    where
        Call: Clone + fmt::Debug + Encode,
    {
        let encoded = xt.hex_encode();
        Ok(self.author_submit_extrinsic(encoded)?)
    }

    /// if Era uses some period and block number, the head_hash must be the head_has of the
    /// block_number used in the era
    pub fn compose_payload_and_extra<Call>(
        &self,
        nonce: u32,
        call: Call,
        era: Option<Era>,
        head_hash: Option<H256>,
        tip: Option<u128>,
    ) -> Result<(SignedPayload<Call>, GenericExtra), Error>
    where
        Call: Clone + fmt::Debug + Encode,
    {
        let tip = tip.unwrap_or(0);
        let era = era.unwrap_or(Era::immortal());
        let extra = GenericExtra(era, Compact(nonce), Compact(tip));

        let raw_payload: SignedPayload<Call> =
            self.compose_payload(call.clone(), extra.clone(), head_hash)?;

        Ok((raw_payload, extra))
    }

    pub fn sign_extrinsic_with_era<P, Call>(
        &self,
        signer: &P,
        call: Call,
        era: Option<Era>,
        head_hash: Option<H256>,
        tip: Option<u128>,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: sp_core::crypto::Pair,
        AccountId32: From<P::Public>,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Call: Clone + fmt::Debug + Encode,
    {
        let signer_account = AccountId32::from(signer.public());
        let nonce = self.get_nonce_for_account(&signer_account)?;
        let (payload, extra) = self.compose_payload_and_extra(
            nonce,
            call.clone(),
            era,
            head_hash,
            tip,
        )?;

        let signature = payload.using_encoded(|payload| signer.sign(payload));
        let multi_signature = MultiSignature::from(signature);

        let extrinsic = UncheckedExtrinsicV4::new_signed(
            call,
            GenericAddress::from(signer_account),
            multi_signature,
            extra,
        );
        Ok(extrinsic)
    }

    pub fn sign_extrinsic<P, Call>(
        &self,
        signer: &P,
        call: Call,
        tip: Option<u128>,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: sp_core::crypto::Pair,
        AccountId32: From<P::Public>,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Call: Clone + fmt::Debug + Encode,
    {
        self.sign_extrinsic_with_era(signer, call, None, None, tip)
    }

    /// create a payload ready for signing and the extra in opaque bytes
    pub fn compose_opaque_payload_and_extra<Call>(
        &self,
        nonce: u32,
        call: Call,
        era: Option<Era>,
        head_hash: Option<H256>,
        tip: Option<u128>,
    ) -> Result<(Vec<u8>, Vec<u8>), Error>
    where
        Call: Clone + fmt::Debug + Encode,
    {
        let (payload, extra) =
            self.compose_payload_and_extra(nonce, call, era, head_hash, tip)?;
        let payload_encoded = payload.encode();
        let payload_for_signing = if payload_encoded.len() > 256 {
            sp_core::blake2_256(&payload_encoded).to_vec()
        } else {
            payload_encoded
        };
        Ok((payload_for_signing, extra.encode()))
    }

    /// submit the signed call with signature and extra
    pub fn submit_signed_call<Call>(
        &self,
        call: Call,
        signer_account: &AccountId32,
        multi_signature: MultiSignature,
        extra: Vec<u8>,
    ) -> Result<Option<H256>, Error>
    where
        Call: Clone + fmt::Debug + Encode,
    {
        let extra = GenericExtra::decode(&mut extra.as_slice())?;
        let extrinsic = UncheckedExtrinsicV4::new_signed(
            call,
            GenericAddress::from(signer_account.clone()),
            multi_signature,
            extra,
        );
        let encoded = extrinsic.hex_encode();
        let tx_hash = self.author_submit_extrinsic(encoded)?;
        Ok(tx_hash)
    }
}
