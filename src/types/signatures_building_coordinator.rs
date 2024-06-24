use crate::prelude::*;

/// A coordinator which gathers signatures from several factor sources of different
/// kinds for many transactions and for potentially multiple derivation paths
/// for each transaction.
pub struct SignaturesBuildingCoordinator {
    /// A context of drivers used to sign with factor sources.
    signing_drivers_context: Arc<dyn IsSigningDriversContext>,

    /// Factor sources grouped by kind, sorted according to "signing order",
    /// that is, we want to control which factor source kind users signs with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to sign with a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    factors_of_kind: IndexMap<FactorSourceKind, IndexSet<FactorSource>>,
}

impl SignaturesBuildingCoordinator {
    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: IndexSet<TransactionIntent>,
        signing_drivers_context: Arc<dyn IsSigningDriversContext>,
    ) -> Self {
        todo!()
    }
}

impl SignaturesBuildingCoordinator {
    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    fn continue_if_necessary(&self) -> Result<()> {
        todo!()
    }

    fn get_driver(&self, kind: FactorSourceKind) -> SigningDriver {
        self.signing_drivers_context
            .driver_for_factor_source_kind(kind)
    }

    async fn sign_with_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        kind: FactorSourceKind,
    ) -> Result<()> {
        assert!(factor_sources.iter().all(|f| f.kind() == kind));

        let signing_driver = self.get_driver(kind);

        signing_driver.sign(kind, factor_sources, self).await;

        todo!()
    }

    async fn do_sign(&self) -> Result<()> {
        let factors_of_kind = self.factors_of_kind.clone();
        for (kind, factor_sources) in factors_of_kind.into_iter() {
            self.sign_with_factor_sources(factor_sources, kind).await?;
            self.continue_if_necessary()?;
        }
        Ok(())
    }
}

impl SignaturesBuildingCoordinator {
    pub async fn sign(&self) -> Result<SignaturesOutcome> {
        self.do_sign().await?;
        let outcome = SignaturesOutcome::new(
            MaybeSignedTransactions::new(IndexMap::new()),
            MaybeSignedTransactions::new(IndexMap::new()),
        );
        Ok(outcome)
    }
}
