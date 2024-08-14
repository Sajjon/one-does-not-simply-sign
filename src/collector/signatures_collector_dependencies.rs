use crate::prelude::*;

use super::factor_sources_of_kind::*;

pub(super) struct SignaturesCollectorDependencies {
    /// A collection of "interactors" used to sign with factor sources.
    pub(super) interactors: Arc<dyn SignatureCollectingInteractors>,

    /// Factor sources grouped by kind, sorted according to "friction order",
    /// that is, we want to control which FactorSourceKind users sign with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to use a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    pub(super) factors_of_kind: IndexSet<FactorSourcesOfKind>,
}

impl SignaturesCollectorDependencies {
    fn with(
        interactors: Arc<dyn SignatureCollectingInteractors>,
        factors_of_kind: IndexSet<FactorSourcesOfKind>,
    ) -> Self {
        Self {
            interactors,
            factors_of_kind,
        }
    }

    pub fn new(
        interactors: Arc<dyn SignatureCollectingInteractors>,
        used_factor_sources: HashSet<FactorSource>,
    ) -> Self {
        let factors_of_kind = used_factor_sources
            .into_iter()
            .into_grouping_map_by(|x| x.kind())
            .collect::<IndexSet<FactorSource>>();

        let mut factors_of_kind = factors_of_kind
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().sorted().collect::<IndexSet<_>>()))
            .collect::<IndexMap<FactorSourceKind, IndexSet<FactorSource>>>();

        factors_of_kind.sort_keys();
        let factors_of_kind = factors_of_kind
            .into_iter()
            .map(|(k, v)| FactorSourcesOfKind::new(k, v).unwrap())
            .collect::<IndexSet<_>>();

        Self::with(interactors, factors_of_kind)
    }
}
