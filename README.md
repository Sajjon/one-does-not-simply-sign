[![codecov](https://codecov.io/github/Sajjon/one-does-not-simply-sign/branch/main/graph/badge.svg?token=PTFupnAjyZ)](https://codecov.io/github/Sajjon/one-does-not-simply-sign)

# Concepts

`FactorInstanceAccumulator` a.k.a. `FIA` is an accumulator which iterates through
all factor sources and dispatches requests to collect their output - `FactorInstance`s -
and builds up - `accumulates` - a final result. The type of `FactorInstance` is
either a `Signature` or a `PublicKey`. FIA support batch operations, meaning it will
collect signatures for MANY transactions, or it will derive public keys for many
Security Shields. In both for both operation kinds, or rather for both processes,
it will need to derive a private key at some derivation path for some FactorSource
and either sign some hashes with it, or simply derive its public key.

FIA is generic over three types:

```rust
pub struct FactorInstanceAccumulator<ID, Path, Product> where ID: Hash, Path: HasDerivationPath, Product: HasHDPublicKey {
    state: ...,
    drivers:
}
```

Here are the two different operations kinds we use FIA for:

````rust
/// Produce many signatures per transaction intent per FactorSource
pub type FIATransactionSigning = FactorInstanceAccumulator<IntentHash, HDPublicKey, HDSignature>;

/// Derives many public keys per FactorSource, used to `SecurityStructureOfFactorSources -> SecurityStructureOfFactorInstances`.
pub type FIAApplySecurityShield = FactorInstanceAccumulator<SecurityShieldID, DerivationPath, HDPublicKey>;

/// Derive a single key, i.e. used for creation of a new account.
/// First generic not really used, but cannot use `()` since it is not `Hash`.
/// the identity of the input, namely the `DerivationPath`s as ID.
pub type FIAJustKeyDerivation = FactorInstanceAccumulator<DerivationPath, DerivationPath, HDPublicKey>;
```.



```rust
/// === REQUEST TYPES ===
pub trait HasDerivationPath {
    fn derivation_path(&self) -> DerivationPath;
}
impl HasDerivationPath for DerivationPath { ... }
pub struct HDPublicKey {
    derivation_path: DerivationPath,
    public_key: PublicKey,
}
impl HasDerivationPath for HDPublicKey { ... }


pub struct BatchUseFactorSourceRequest<ID, Path> where ID: Hash, Path: HasDerivationPath {
	inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>
}

pub type BatchDerivePublicKeysRequest = BatchUseFactorSourceRequest<SecurityShieldID, DerivationPath>;
pub type BatchSignTransactionsRequest = BatchUseFactorSourceRequest<IntentHash, HDPublicKey>;

/// === RESPONSE TYPES ===
pub trait HasHDPublicKey {
	fn hd_public_key(&self) -> HDPublicKey;
}
impl HasHDPublicKey for HDSignature { ... }
impl HasHDPublicKey for HDPublicKey { ... }

pub struct BatchUseFactorSourceResponse<ID, Product> where ID: Hash, Product: HasHDPublicKey {
	outputs: HashMap<ID, Vec<Product>>
}
pub type BatchDerivePublicKeysResponse = BatchUseFactorSourceResponse<SecurityShieldID, HDPublicKey>;
pub type BatchSignTransactionsResponse = BatchUseFactorSourceResponse<IntentHash, HDSignature>;

pub trait UseFactorSourceDriver {

}

pub trait SignWithFactorSourceDriver: UseFactorSourceDriver {
	/// Produces many signatures for many entities from many factor sources for many transactions.
	async fn batch_sign_transactions(
		&self,
		request: BatchSignTransactionsRequest
	) -> Result<BatchSignTransactionsResponse>;
}
impl<T: SignWithFactorSourceDriver> UseFactorSourceDriver for T {

}

pub trait DeriveKeysWithFactorSourceDriver: UseFactorSourceDriver {
    /// Derives many keys from many factor sources for many entities.
	async fn batch_derive_public_keys(
		&self,
		request: BatchDerivePublicKeysRequest
	) -> Result<BatchDerivePublicKeysResponse>;
}
````

```rust
/// ===== Public =====
impl<ID, Path, Product> FactorInstanceAccumulator {

    pub fn new(
        request: BatchUseFactorSourceRequest<ID, Path>,
        factor_sources: Vec<FactorSource>
    ) -> Result<Self> {
        ...
    }

    pub async fn accumulate(&self) -> Result<BatchUseFactorSourceResponse<ID, Product>> {
       ...
    }
}
```

And we need to prepare init FIA with inputs, here is a rough initial sketch:

```rust
impl FactorInstanceAccumulator
where
    (*const ID, *const IntentHash): TyEq,       // ID == IntentHash
    (*const Path, *const HDPublicKey): TyEq,    // Path == HDPublicKey
    (*const Product, *const HDSignature): TyEq, // Product == HDSignature
    // https://github.com/rust-lang/rust/issues/20041
{

    pub fn new_batch_sign_transactions(
        inputs: HashMap<FactorSourceID, HashMap<IntentHash, Vec<HDPublicKey>>>,
        factor_sources: Vec<FactorSource>
    ) -> Result<Self> {
        Self::new(
            BatchSignTransactionsRequest::new(inputs),
            factor_sources,
        )
    }

    pub fn new_batch_sign_transactions_grouping(
        instances_per_transaction: HashMap<IntentHash, Vec<HDPublicKey>>,
        factor_sources: Vec<FactorSource>
    ) -> Result<Self> {
        let inputs = ...
        Self::new_batch_sign_transactions(inputs, factor_sources)
    }

    pub fn new_batch_sign_by_analyzing_transactions_using<F>(
        transactions: Vec<TransactionIntent>,
        entities: Vec<AccountOrPersona>,
        factor_sources: Vec<FactorSource>,
        signers_of_transaction: F
    ) -> Result<Self> where F: Fn(TransactionIntent) -> Vec<HDPublicKey> {
       let inputs: HashMap<IntentHash, Vec<HDPublicKey>> = transaction.into_iter().
       Self::new_batch_sign_transactions_grouping(inputs, )
    }


    pub fn new_batch_sign_by_analyzing_transactions(
        transactions: Vec<TransactionIntent>,
        entities: Vec<AccountOrPersona>,
        factor_sources: Vec<FactorSource>
    ) -> Result<Self> {
       let inputs = ...
       Self::new_batch_sign_transactions(inputs)
    }
}
```

And analogously for Derivation of PublicKeys:

```rust
impl FactorInstanceAccumulator
where
    (*const ID, *const SecurityShieldID): TyEq,       // ID == SecurityShieldID
    (*const Path, *const DerivationPath): TyEq,    // Path == DerivationPath
    (*const Product, *const HDPublicKey): TyEq, // Product == HDPublicKey
    // https://github.com/rust-lang/rust/issues/20041
{

}
```
