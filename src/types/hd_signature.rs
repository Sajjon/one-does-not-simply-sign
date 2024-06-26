use crate::prelude::*;

/// The input used to produce a `HDSignature`. Can be used to see two signatures
/// has the same signer, which would be a bug.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug(
    "HDSignatureInput {{ intent_hash: {:?}, owned_factor_instance: {:?} }}",
    intent_hash,
    owned_factor_instance
)]
pub struct HDSignatureInput {
    /// Hash which was signed.
    pub intent_hash: IntentHash,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,
}
impl HDSignatureInput {
    /// Constructs a new `HDSignatureInput`.
    pub fn new(intent_hash: IntentHash, owned_factor_instance: OwnedFactorInstance) -> Self {
        Self {
            intent_hash,
            owned_factor_instance,
        }
    }
}

impl HasSampleValues for HDSignatureInput {
    fn sample() -> Self {
        Self::new(IntentHash::sample(), OwnedFactorInstance::sample())
    }
    fn sample_other() -> Self {
        Self::new(
            IntentHash::sample_other(),
            OwnedFactorInstance::sample_other(),
        )
    }
}

#[cfg(test)]
mod hd_signature_input_tests {
    use super::*;

    type Sut = HDSignatureInput;

    #[test]
    fn equality_of_samples() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            IndexSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample(),
                Sut::sample_other()
            ])
            .len(),
            2
        );
    }
}

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug("HDSignature {{ input: {:?} }}", input)]
pub struct HDSignature {
    /// The input used to produce this `HDSignature`
    pub input: HDSignatureInput,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the FactorSource identified by
    /// `owned_hd_factor_instance.factor_source_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: Signature,
}

impl HDSignature {
    pub fn produced_signing_with_input(input: HDSignatureInput) -> Self {
        let signature = Signature::produced_by_input(&input);
        Self::with_details(input, signature)
    }

    pub fn produced_signing_with(
        intent_hash: IntentHash,
        owned_factor_instance: OwnedFactorInstance,
    ) -> Self {
        let input = HDSignatureInput::new(intent_hash, owned_factor_instance);
        Self::produced_signing_with_input(input)
    }

    /// Constructs a HDSignature from an already produced `Signature`.
    fn with_details(input: HDSignatureInput, signature: Signature) -> Self {
        Self { input, signature }
    }

    pub fn intent_hash(&self) -> &IntentHash {
        &self.input.intent_hash
    }

    pub fn owned_factor_instance(&self) -> &OwnedFactorInstance {
        &self.input.owned_factor_instance
    }

    pub fn used_same_input_as(&self, other: &Self) -> bool {
        self.input == other.input
    }
}

impl HasSampleValues for HDSignature {
    fn sample() -> Self {
        let input = HDSignatureInput::sample();
        Self::produced_signing_with_input(input)
    }
    fn sample_other() -> Self {
        let input = HDSignatureInput::sample_other();
        Self::produced_signing_with_input(input)
    }
}

#[cfg(test)]
mod hd_signature_tests {
    use super::*;

    type Sut = HDSignature;

    #[test]
    fn equality_of_samples() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            IndexSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample(),
                Sut::sample_other()
            ])
            .len(),
            2
        );
    }
}
