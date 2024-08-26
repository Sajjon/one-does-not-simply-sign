use crate::prelude::*;

use super::{
    signatures_collector_dependencies::*, signatures_collector_preprocessor::*,
    signatures_collector_state::*,
};

/// A coordinator which gathers signatures from several factor sources of different
/// kinds, in increasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By increasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
pub struct SignaturesCollector {
    /// Stateless immutable values used by the collector to gather signatures
    /// from factor sources.
    dependencies: SignaturesCollectorDependencies,

    /// Mutable internal state of the collector which builds up the list
    /// of signatures from each used factor source.
    state: RefCell<SignaturesCollectorState>,
}

impl SignaturesCollector {
    /// Used by our tests. But Sargon will typically wanna use `SignaturesCollector::new` and passing
    /// it a
    pub(crate) fn with(
        all_factor_sources_in_profile: IndexSet<HDFactorSource>,
        transactions: IndexSet<TXToSign>,
        interactors: Arc<dyn SignatureCollectingInteractors>,
    ) -> Self {
        let preprocessor = SignaturesCollectorPreprocessor::new(transactions);
        let (petitions, factors) = preprocessor.preprocess(all_factor_sources_in_profile);

        let dependencies = SignaturesCollectorDependencies::new(interactors, factors);
        let state = SignaturesCollectorState::new(petitions);

        Self {
            dependencies,
            state: RefCell::new(state),
        }
    }

    pub fn with_signers_extraction<F>(
        all_factor_sources_in_profile: IndexSet<HDFactorSource>,
        transactions: IndexSet<TransactionIntent>,
        interactors: Arc<dyn SignatureCollectingInteractors>,
        extract_signers: F,
    ) -> Result<Self>
    where
        F: Fn(TransactionIntent) -> Result<TXToSign>,
    {
        let transactions = transactions
            .into_iter()
            .map(extract_signers)
            .collect::<Result<IndexSet<TXToSign>>>()?;

        let collector = Self::with(all_factor_sources_in_profile, transactions, interactors);

        Ok(collector)
    }

    pub fn new(
        transactions: IndexSet<TransactionIntent>,
        interactors: Arc<dyn SignatureCollectingInteractors>,
        profile: &Profile,
    ) -> Result<Self> {
        Self::with_signers_extraction(
            profile.factor_sources.clone(),
            transactions,
            interactors,
            |i| TXToSign::extracting_from_intent_and_profile(&i, profile),
        )
    }
}

impl TXToSign {
    pub fn extracting_from_intent_and_profile(
        intent: &TransactionIntent,
        profile: &Profile,
    ) -> Result<Self> {
        let intent_hash = intent.intent_hash.clone();
        let summary = intent.manifest_summary();
        let mut entities_requiring_auth: IndexSet<AccountOrPersona> = IndexSet::new();

        let accounts = summary
            .addresses_of_accounts_requiring_auth
            .into_iter()
            .map(|a| profile.account_by_address(a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            accounts
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );

        let personas = summary
            .addresses_of_personas_requiring_auth
            .into_iter()
            .map(|a| profile.persona_by_address(a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            personas
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );

        Ok(Self::with(intent_hash, entities_requiring_auth))
    }
}

impl SignaturesCollector {
    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    ///
    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub(crate) fn continue_if_necessary(&self) -> Result<bool> {
        self.state
            .borrow()
            .petitions
            .borrow()
            .continue_if_necessary()
    }

    fn get_interactor(&self, kind: FactorSourceKind) -> SigningInteractor {
        self.dependencies.interactors.interactor_for(kind)
    }

    async fn sign_with_factors_of_kind(
        &self,
        factor_sources_of_kind: FactorSourcesOfKind,
    ) -> Result<()> {
        let interactor = self.get_interactor(factor_sources_of_kind.kind);

        let client = SignWithFactorClient::new(interactor);

        let result = client
            .use_factor_sources(factor_sources_of_kind.factor_sources(), self)
            .await;

        match result {
            Ok(_) => {}
            Err(_) => self.process_batch_response(SignWithFactorSourceOrSourcesOutcome::Skipped {
                ids_of_skipped_factors_sources: factor_sources_of_kind.factor_source_ids(),
            }),
        }
        Ok(())
    }

    /// In decreasing "friction order"
    async fn sign_with_factors(&self) -> Result<()> {
        let factors_of_kind = self.dependencies.factors_of_kind.clone();
        for factor_sources_of_kind in factors_of_kind.into_iter() {
            if !self.continue_if_necessary()? {
                break; // finished early, we have fulfilled signing requirements of all transactions
            }
            self.sign_with_factors_of_kind(factor_sources_of_kind)
                .await?;
        }
        Ok(())
    }
}

impl SignaturesCollector {
    fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> BatchTXBatchKeySigningRequest {
        self.state
            .borrow()
            .petitions
            .borrow()
            .input_for_interactor(factor_source_id)
    }

    pub(crate) fn request_for_serial_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> SerialBatchSigningRequest {
        let batch_signing_request = self.input_for_interactor(factor_source_id);

        SerialBatchSigningRequest::new(
            batch_signing_request,
            self.invalid_transactions_if_skipped(factor_source_id)
                .into_iter()
                .collect_vec(),
        )
    }

    pub(crate) fn request_for_parallel_interactor(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> ParallelBatchSigningRequest {
        let per_factor_source = factor_source_ids
            .clone()
            .iter()
            .map(|fid| (*fid, self.input_for_interactor(fid)))
            .collect::<IndexMap<FactorSourceIDFromHash, BatchTXBatchKeySigningRequest>>();

        let invalid_transactions_if_skipped =
            self.invalid_transactions_if_skipped_factor_sources(factor_source_ids);

        // Prepare the request for the interactor
        ParallelBatchSigningRequest::new(per_factor_source, invalid_transactions_if_skipped)
    }

    pub(super) fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        self.state
            .borrow()
            .petitions
            .borrow()
            .invalid_transactions_if_skipped(factor_source_id)
    }

    fn invalid_transactions_if_skipped_factor_sources(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        factor_source_ids
            .into_iter()
            .flat_map(|f| self.invalid_transactions_if_skipped(&f))
            .collect::<IndexSet<_>>()
    }

    pub(crate) fn process_batch_response(
        &self,
        response: SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>,
    ) {
        let state = self.state.borrow_mut();
        let petitions = state.petitions.borrow_mut();
        petitions.process_batch_response(response)
    }

    fn outcome(self) -> SignaturesOutcome {
        let state = self.state.borrow_mut();
        let petitions = state.petitions.borrow_mut();
        let expected_number_of_transactions = petitions.txid_to_petition.borrow().len();
        drop(petitions);
        drop(state);
        let outcome = self.state.into_inner().petitions.into_inner().outcome();
        assert_eq!(
            outcome.failed_transactions().len() + outcome.successful_transactions().len(),
            expected_number_of_transactions
        );
        outcome
    }
}

impl SignaturesCollector {
    pub async fn collect_signatures(self) -> SignaturesOutcome {
        _ = self
            .sign_with_factors() // in decreasing "friction order"
            .await
            .inspect_err(|e| eprintln!("Failed to use factor sources: {:#?}", e));

        self.outcome()
    }
}

#[cfg(test)]
impl SignaturesCollector {
    /// Used by tests
    pub(crate) fn petitions(self) -> Petitions {
        self.state.into_inner().petitions.into_inner()
    }
}

#[cfg(test)]
mod tests {

    use std::iter;

    use super::*;

    #[test]
    fn invalid_profile_unknown_account() {
        let res = SignaturesCollector::new(
            IndexSet::from_iter([TransactionIntent::new([Account::a0().entity_address()], [])]),
            Arc::new(TestSignatureCollectingInteractors::new(
                SimulatedUser::prudent_no_fail(),
            )),
            &Profile::new(IndexSet::new(), [], []),
        );
        assert!(matches!(res, Err(CommonError::UnknownAccount)));
    }

    #[test]
    fn invalid_profile_unknown_persona() {
        let res = SignaturesCollector::new(
            IndexSet::from_iter([TransactionIntent::new([], [Persona::p0().entity_address()])]),
            Arc::new(TestSignatureCollectingInteractors::new(
                SimulatedUser::prudent_no_fail(),
            )),
            &Profile::new(IndexSet::new(), [], []),
        );
        assert!(matches!(res, Err(CommonError::UnknownPersona)));
    }

    #[actix_rt::test]
    async fn valid_profile() {
        let factors_sources = HDFactorSource::all();
        let persona = Persona::p0();
        let collector = SignaturesCollector::new(
            IndexSet::from_iter([TransactionIntent::new([], [persona.entity_address()])]),
            Arc::new(TestSignatureCollectingInteractors::new(
                SimulatedUser::prudent_no_fail(),
            )),
            &Profile::new(factors_sources, [], [&persona]),
        )
        .unwrap();
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful())
    }

    #[test]
    fn test_profile() {
        let factor_sources = &HDFactorSource::all();
        let a0 = &Account::a0();
        let a1 = &Account::a1();
        let a2 = &Account::a2();
        let a6 = &Account::a6();

        let p0 = &Persona::p0();
        let p1 = &Persona::p1();
        let p2 = &Persona::p2();
        let p6 = &Persona::p6();

        let t0 = TransactionIntent::address_of([a0, a1], [p0, p1]);
        let t1 = TransactionIntent::address_of([a0, a1, a2], []);
        let t2 = TransactionIntent::address_of([], [p0, p1, p2]);
        let t3 = TransactionIntent::address_of([a6], [p6]);

        let profile = Profile::new(factor_sources.clone(), [a0, a1, a2, a6], [p0, p1, p2, p6]);

        let collector = SignaturesCollector::new(
            IndexSet::<TransactionIntent>::from_iter([
                t0.clone(),
                t1.clone(),
                t2.clone(),
                t3.clone(),
            ]),
            Arc::new(TestSignatureCollectingInteractors::new(
                SimulatedUser::prudent_no_fail(),
            )),
            &profile,
        )
        .unwrap();

        let petitions = collector.petitions();

        assert_eq!(petitions.txid_to_petition.borrow().len(), 4);

        {
            let petitions_ref = petitions.txid_to_petition.borrow();
            let petition = petitions_ref.get(&t3.intent_hash).unwrap();
            let for_entities = petition.for_entities.borrow().clone();
            let pet6 = for_entities.get(&a6.address()).unwrap();

            let paths6 = pet6
                .all_factor_instances()
                .iter()
                .map(|f| f.factor_instance().derivation_path())
                .collect_vec();

            pretty_assertions::assert_eq!(
                paths6,
                iter::repeat_n(
                    DerivationPath::new(
                        NetworkID::Mainnet,
                        CAP26EntityKind::Account,
                        CAP26KeyKind::T9n,
                        HDPathComponent::non_hardened(6)
                    ),
                    5
                )
                .collect_vec()
            );
        }

        let assert_petition = |t: &TransactionIntent,
                               threshold_factors: HashMap<
            AddressOfAccountOrPersona,
            HashSet<FactorSourceIDFromHash>,
        >,
                               override_factors: HashMap<
            AddressOfAccountOrPersona,
            HashSet<FactorSourceIDFromHash>,
        >| {
            let petitions_ref = petitions.txid_to_petition.borrow();
            let petition = petitions_ref.get(&t.intent_hash).unwrap();
            assert_eq!(petition.intent_hash, t.intent_hash);

            let mut addresses = threshold_factors.keys().collect::<HashSet<_>>();
            addresses.extend(override_factors.keys().collect::<HashSet<_>>());

            assert_eq!(
                petition
                    .for_entities
                    .borrow()
                    .keys()
                    .collect::<HashSet<_>>(),
                addresses
            );

            assert!(petition
                .for_entities
                .borrow()
                .iter()
                .all(|(a, p)| { p.entity == *a }));

            assert!(petition
                .for_entities
                .borrow()
                .iter()
                .all(|(_, p)| { p.intent_hash == t.intent_hash }));

            for (k, v) in petition.for_entities.borrow().iter() {
                let threshold = threshold_factors.get(k);
                if let Some(actual_threshold) = &v.threshold_factors {
                    let threshold = threshold.unwrap().clone();
                    assert_eq!(
                        actual_threshold
                            .borrow()
                            .factor_instances()
                            .into_iter()
                            .map(|f| f.factor_source_id)
                            .collect::<HashSet<_>>(),
                        threshold
                    );
                } else {
                    assert!(threshold.is_none());
                }

                let override_ = override_factors.get(k);
                if let Some(actual_override) = &v.override_factors {
                    let override_ = override_.unwrap().clone();
                    assert_eq!(
                        actual_override
                            .borrow()
                            .factor_instances()
                            .into_iter()
                            .map(|f| f.factor_source_id)
                            .collect::<HashSet<_>>(),
                        override_
                    );
                } else {
                    assert!(override_.is_none());
                }
            }
        };
        assert_petition(
            &t0,
            HashMap::from_iter([
                (
                    a0.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs0()]),
                ),
                (
                    a1.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs1()]),
                ),
                (
                    p0.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs0()]),
                ),
                (
                    p1.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs1()]),
                ),
            ]),
            HashMap::new(),
        );

        assert_petition(
            &t1,
            HashMap::from_iter([
                (
                    a0.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs0()]),
                ),
                (
                    a1.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs1()]),
                ),
                (
                    a2.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs0()]),
                ),
            ]),
            HashMap::new(),
        );

        assert_petition(
            &t2,
            HashMap::from_iter([
                (
                    p0.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs0()]),
                ),
                (
                    p1.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs1()]),
                ),
                (
                    p2.address(),
                    HashSet::from_iter([FactorSourceIDFromHash::fs0()]),
                ),
            ]),
            HashMap::new(),
        );

        assert_petition(
            &t3,
            HashMap::from_iter([
                (
                    a6.address(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::fs0(),
                        FactorSourceIDFromHash::fs3(),
                        FactorSourceIDFromHash::fs5(),
                    ]),
                ),
                (
                    p6.address(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::fs0(),
                        FactorSourceIDFromHash::fs3(),
                        FactorSourceIDFromHash::fs5(),
                    ]),
                ),
            ]),
            HashMap::from_iter([
                (
                    a6.address(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::fs1(),
                        FactorSourceIDFromHash::fs4(),
                    ]),
                ),
                (
                    p6.address(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::fs1(),
                        FactorSourceIDFromHash::fs4(),
                    ]),
                ),
            ]),
        );
    }
}
