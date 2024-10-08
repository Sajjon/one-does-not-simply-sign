use use_factors::prelude::*;

#[cfg(test)]
mod common_tests {

    use super::*;

    #[test]
    fn factors_sources() {
        assert_eq!(ALL_FACTOR_SOURCES.clone(), ALL_FACTOR_SOURCES.clone());
    }

    #[test]
    fn factors_source_ids() {
        assert_eq!(FactorSourceIDFromHash::fs0(), FactorSourceIDFromHash::fs0());
        assert_eq!(FactorSourceIDFromHash::fs1(), FactorSourceIDFromHash::fs1());
        assert_ne!(FactorSourceIDFromHash::fs0(), FactorSourceIDFromHash::fs1());
    }

    #[test]
    fn factor_instance_in_accounts() {
        assert_eq!(
            Account::a0().security_state.all_factor_instances(),
            Account::a0().security_state.all_factor_instances()
        );
        assert_eq!(
            Account::a6().security_state.all_factor_instances(),
            Account::a6().security_state.all_factor_instances()
        );
    }
}

#[cfg(test)]
mod key_derivation_tests {

    use super::CAP26EntityKind::*;
    use super::CAP26KeyKind::*;
    use super::NetworkID::*;
    use super::*;

    #[actix_rt::test]
    async fn failure() {
        let factor_source = fs_at(0);
        let paths = [0, 1, 2]
            .into_iter()
            .map(|i| DerivationPath::at(Mainnet, Account, T9n, i))
            .collect::<IndexSet<_>>();
        let collector = KeysCollector::new(
            HDFactorSource::all(),
            [(factor_source.factor_source_id(), paths.clone())]
                .into_iter()
                .collect::<IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>>(),
            Arc::new(TestDerivationInteractors::fail()),
        );
        let outcome = collector.collect_keys().await;
        println!("{:#?}", outcome);
        assert!(outcome.all_factors().is_empty())
    }

    mod multi_key {
        use super::*;

        #[actix_rt::test]
        async fn multi_keys_same_factor_source_different_indices() {
            let factor_source = fs_at(0);
            let paths = [0, 1, 2]
                .into_iter()
                .map(|i| DerivationPath::at(Mainnet, Account, T9n, i))
                .collect::<IndexSet<_>>();
            let collector =
                KeysCollector::new_test([(factor_source.factor_source_id(), paths.clone())]);
            let outcome = collector.collect_keys().await;
            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert!(outcome
                .all_factors()
                .into_iter()
                .all(|f| f.factor_source_id == factor_source.factor_source_id()));
        }

        #[actix_rt::test]
        async fn multi_keys_multi_factor_sources_single_index_per() {
            let path = DerivationPath::account_tx(Mainnet, HDPathComponent::non_hardened(0));
            let paths = IndexSet::from_iter([path]);
            let factor_sources = HDFactorSource::all();

            let collector = KeysCollector::new_test(
                factor_sources
                    .iter()
                    .map(|f| (f.factor_source_id(), paths.clone()))
                    .collect_vec(),
            );
            let outcome = collector.collect_keys().await;
            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet::<_>>(),
                factor_sources
                    .into_iter()
                    .map(|f| f.factor_source_id())
                    .collect::<HashSet::<_>>()
            );
        }

        #[actix_rt::test]
        async fn multi_keys_multi_factor_sources_multi_paths() {
            let paths = [0, 1, 2]
                .into_iter()
                .map(|i| DerivationPath::at(Mainnet, Account, T9n, i))
                .collect::<IndexSet<_>>();

            let factor_sources = HDFactorSource::all();

            let collector = KeysCollector::new_test(
                factor_sources
                    .iter()
                    .map(|f| (f.factor_source_id(), paths.clone()))
                    .collect_vec(),
            );
            let outcome = collector.collect_keys().await;

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet::<_>>(),
                factor_sources
                    .into_iter()
                    .map(|f| f.factor_source_id())
                    .collect::<HashSet::<_>>()
            );
        }

        #[actix_rt::test]
        async fn multi_keys_multi_factor_sources_multi_paths_complex() {
            let mut paths = IndexSet::new();

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Mainnet, Account, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Stokenet, Account, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Mainnet, Identity, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Stokenet, Identity, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Mainnet, Account, Rola, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Stokenet, Account, Rola, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Mainnet, Identity, Rola, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::at(Stokenet, Identity, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Mainnet, Account, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Stokenet, Account, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Mainnet, Identity, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Stokenet, Identity, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Mainnet, Account, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Stokenet, Account, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Mainnet, Identity, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    BIP32_SECURIFIED_HALF,
                    BIP32_SECURIFIED_HALF + 1,
                    BIP32_SECURIFIED_HALF + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::at(Stokenet, Identity, Rola, i)),
            );

            let factor_sources = HDFactorSource::all();

            let collector = KeysCollector::new_test(
                factor_sources
                    .iter()
                    .map(|f| (f.factor_source_id(), paths.clone()))
                    .collect_vec(),
            );
            let outcome = collector.collect_keys().await;

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert!(outcome.all_factors().len() > 200);

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet::<_>>(),
                factor_sources
                    .into_iter()
                    .map(|f| f.factor_source_id())
                    .collect::<HashSet::<_>>()
            );
        }
    }

    mod single_key {
        use super::*;

        struct Expected {
            index: HDPathComponent,
        }

        async fn do_test(
            key_space: KeySpace,
            factor_source: &HDFactorSource,
            network_id: NetworkID,
            entity_kind: CAP26EntityKind,
            key_kind: CAP26KeyKind,
            expected: Expected,
        ) {
            let collector =
                KeysCollector::with(factor_source, network_id, key_kind, entity_kind, key_space);

            let outcome = collector.collect_keys().await;
            let factors = outcome.all_factors();
            assert_eq!(factors.len(), 1);
            let factor = factors.first().unwrap();
            assert_eq!(
                factor.derivation_path(),
                DerivationPath::new(network_id, entity_kind, key_kind, expected.index)
            );
            assert_eq!(factor.factor_source_id, factor_source.factor_source_id());
        }

        mod securified {
            use super::*;

            async fn test(
                factor_source: &HDFactorSource,
                network_id: NetworkID,
                entity_kind: CAP26EntityKind,
                key_kind: CAP26KeyKind,
            ) {
                do_test(
                    KeySpace::Securified,
                    factor_source,
                    network_id,
                    entity_kind,
                    key_kind,
                    Expected {
                        index: HDPathComponent::non_hardened(BIP32_SECURIFIED_HALF),
                    },
                )
                .await
            }

            mod account {
                use super::*;

                async fn each_factor(network_id: NetworkID, key_kind: CAP26KeyKind) {
                    for factor_source in HDFactorSource::all().iter() {
                        test(factor_source, network_id, Account, key_kind).await
                    }
                }

                #[actix_rt::test]
                async fn single_first_account_mainnet_t9n() {
                    each_factor(Mainnet, T9n).await
                }
            }
        }

        mod unsecurified {
            use super::*;

            async fn test(
                factor_source: &HDFactorSource,
                network_id: NetworkID,
                entity_kind: CAP26EntityKind,
                key_kind: CAP26KeyKind,
            ) {
                do_test(
                    KeySpace::Unsecurified,
                    factor_source,
                    network_id,
                    entity_kind,
                    key_kind,
                    Expected {
                        index: HDPathComponent::non_hardened(0),
                    },
                )
                .await
            }

            mod account {
                use super::*;

                async fn each_factor(network_id: NetworkID, key_kind: CAP26KeyKind) {
                    for factor_source in HDFactorSource::all().iter() {
                        test(factor_source, network_id, Account, key_kind).await
                    }
                }

                #[actix_rt::test]
                async fn single_first_account_mainnet_t9n() {
                    each_factor(Mainnet, T9n).await
                }

                #[actix_rt::test]
                async fn single_first_account_stokenet_t9n() {
                    each_factor(Mainnet, T9n).await
                }

                #[actix_rt::test]
                async fn single_first_account_mainnet_rola() {
                    each_factor(Mainnet, Rola).await
                }

                #[actix_rt::test]
                async fn single_first_account_stokenet_rola() {
                    each_factor(Stokenet, Rola).await
                }
            }

            mod persona {
                use super::*;

                async fn each_factor(network_id: NetworkID, key_kind: CAP26KeyKind) {
                    for factor_source in HDFactorSource::all().iter() {
                        test(factor_source, network_id, Identity, key_kind).await
                    }
                }

                #[actix_rt::test]
                async fn single_first_persona_mainnet_t9n() {
                    each_factor(Mainnet, T9n).await
                }

                #[actix_rt::test]
                async fn single_first_persona_stokenet_t9n() {
                    each_factor(Mainnet, T9n).await
                }

                #[actix_rt::test]
                async fn single_first_persona_mainnet_rola() {
                    each_factor(Mainnet, Rola).await
                }

                #[actix_rt::test]
                async fn single_first_persona_stokenet_rola() {
                    each_factor(Stokenet, Rola).await
                }
            }
        }
    }
}

#[cfg(test)]
mod signing_tests {

    use super::*;

    mod multi_tx {

        use super::*;

        async fn multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
            sim: SimulatedUser,
        ) {
            let factor_sources = &HDFactorSource::all();
            let a0 = &Account::a0();
            let a1 = &Account::a1();
            let a2 = &Account::a2();

            let p0 = &Persona::p0();
            let p1 = &Persona::p1();
            let p2 = &Persona::p2();

            let t0 = TransactionIntent::address_of([a0, a1], [p0, p1]);
            let t1 = TransactionIntent::address_of([a0, a1, a2], []);
            let t2 = TransactionIntent::address_of([], [p0, p1, p2]);

            let profile = Profile::new(factor_sources.clone(), [a0, a1, a2], [p0, p1, p2]);

            let collector = SignaturesCollector::new(
                IndexSet::<TransactionIntent>::from_iter([t0.clone(), t1.clone(), t2.clone()]),
                Arc::new(TestSignatureCollectingInteractors::new(sim)),
                &profile,
            )
            .unwrap();

            let outcome = collector.collect_signatures().await;
            assert!(outcome.successful());
            assert!(outcome.failed_transactions().is_empty());
            assert_eq!(outcome.signatures_of_successful_transactions().len(), 10);
            assert_eq!(
                outcome
                    .successful_transactions()
                    .into_iter()
                    .map(|t| t.intent_hash)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    t0.clone().intent_hash,
                    t1.clone().intent_hash,
                    t2.clone().intent_hash,
                ])
            );
            let st0 = outcome
                .successful_transactions()
                .into_iter()
                .find(|st| st.intent_hash == t0.intent_hash)
                .unwrap();

            assert_eq!(
                st0.signatures
                    .clone()
                    .into_iter()
                    .map(|s| s.owned_factor_instance().owner.clone())
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([a0.address(), a1.address(), p0.address(), p1.address()])
            );

            let st1 = outcome
                .successful_transactions()
                .into_iter()
                .find(|st| st.intent_hash == t1.intent_hash)
                .unwrap();

            assert_eq!(
                st1.signatures
                    .clone()
                    .into_iter()
                    .map(|s| s.owned_factor_instance().owner.clone())
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([a0.address(), a1.address(), a2.address()])
            );

            let st2 = outcome
                .successful_transactions()
                .into_iter()
                .find(|st| st.intent_hash == t2.intent_hash)
                .unwrap();

            assert_eq!(
                st2.signatures
                    .clone()
                    .into_iter()
                    .map(|s| s.owned_factor_instance().owner.clone())
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([p0.address(), p1.address(), p2.address()])
            );

            // Assert sorted in increasing "friction order".
            assert_eq!(
                outcome
                    .signatures_of_successful_transactions()
                    .iter()
                    .map(|f| { f.factor_source_id().kind })
                    .collect::<IndexSet::<FactorSourceKind>>(),
                IndexSet::<FactorSourceKind>::from_iter([
                    FactorSourceKind::Device,
                    FactorSourceKind::Ledger
                ])
            );
        }

        #[derive(Clone, Debug)]
        struct Vector {
            simulated_user: SimulatedUser,
            expected: Expected,
        }
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Expected {
            successful_txs_signature_count: usize,
            signed_factor_source_kinds: IndexSet<FactorSourceKind>,
            expected_skipped_factor_source_count: usize,
        }
        async fn multi_securified_entities_with_sim_user(vector: Vector) {
            let factor_sources = &HDFactorSource::all();

            let a4 = &Account::a4();
            let a5 = &Account::a5();
            let a6 = &Account::a6();

            let p4 = &Persona::p4();
            let p5 = &Persona::p5();
            let p6 = &Persona::p6();

            let t0 = TransactionIntent::address_of([a5], [p5]);
            let t1 = TransactionIntent::address_of([a4, a5, a6], []);
            let t2 = TransactionIntent::address_of([a4, a6], [p4, p6]);
            let t3 = TransactionIntent::address_of([], [p4, p5, p6]);

            let profile = Profile::new(factor_sources.clone(), [a4, a5, a6], [p4, p5, p6]);

            let collector = SignaturesCollector::new(
                IndexSet::<TransactionIntent>::from_iter([
                    t0.clone(),
                    t1.clone(),
                    t2.clone(),
                    t3.clone(),
                ]),
                Arc::new(TestSignatureCollectingInteractors::new(
                    vector.simulated_user,
                )),
                &profile,
            )
            .unwrap();

            let outcome = collector.collect_signatures().await;

            assert_eq!(
                outcome.skipped_factor_sources().len(),
                vector.expected.expected_skipped_factor_source_count
            );

            assert!(outcome.successful());
            assert!(outcome.failed_transactions().is_empty());
            assert_eq!(
                outcome.signatures_of_successful_transactions().len(),
                vector.expected.successful_txs_signature_count
            );
            assert_eq!(
                outcome
                    .successful_transactions()
                    .into_iter()
                    .map(|t| t.intent_hash)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    t0.clone().intent_hash,
                    t1.clone().intent_hash,
                    t2.clone().intent_hash,
                    t3.clone().intent_hash,
                ])
            );

            // Assert sorted in increasing "friction order".
            assert_eq!(
                outcome
                    .signatures_of_successful_transactions()
                    .iter()
                    .map(|f| { f.factor_source_id().kind })
                    .collect::<IndexSet::<FactorSourceKind>>(),
                vector.expected.signed_factor_source_kinds
            );
        }

        mod with_failure {
            use super::*;

            #[actix_rt::test]
            async fn multi_securified_entities() {
                multi_securified_entities_with_sim_user(Vector {
                    simulated_user: SimulatedUser::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures([FactorSourceIDFromHash::fs1()]),
                    ),
                    expected: Expected {
                        successful_txs_signature_count: 24,
                        // We always end early
                        // `Device` FactorSourceKind never got used since it
                        // we are done after YubiKey.
                        signed_factor_source_kinds: IndexSet::<FactorSourceKind>::from_iter([
                            FactorSourceKind::Arculus,
                            FactorSourceKind::Yubikey,
                        ]),
                        expected_skipped_factor_source_count: 1,
                    },
                })
                .await;
            }

            #[actix_rt::test]
            async fn many_failing_tx() {
                let factor_sources = &HDFactorSource::all();
                let a0 = &Account::a0();
                let p3 = &Persona::p3();
                let failing_transactions = (0..100)
                    .map(|_| TransactionIntent::address_of([a0], []))
                    .collect::<IndexSet<_>>();
                let tx = TransactionIntent::address_of([], [p3]);
                let mut all_transactions = failing_transactions.clone();
                all_transactions.insert(tx.clone());

                let profile = Profile::new(factor_sources.clone(), [a0], [p3]);

                let collector = SignaturesCollector::new(
                    all_transactions,
                    Arc::new(TestSignatureCollectingInteractors::new(
                        SimulatedUser::prudent_with_failures(
                            SimulatedFailures::with_simulated_failures([
                                FactorSourceIDFromHash::fs0(),
                            ]),
                        ),
                    )),
                    &profile,
                )
                .unwrap();

                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                assert_eq!(
                    outcome
                        .failed_transactions()
                        .iter()
                        .map(|t| t.intent_hash.clone())
                        .collect_vec(),
                    failing_transactions
                        .iter()
                        .map(|t| t.intent_hash.clone())
                        .collect_vec()
                );

                assert_eq!(
                    outcome
                        .successful_transactions()
                        .into_iter()
                        .map(|t| t.intent_hash)
                        .collect_vec(),
                    vec![tx.intent_hash]
                )
            }
        }

        mod no_fail {
            use super::*;

            #[actix_rt::test]
            async fn multi_accounts_multi_personas_all_single_factor_controlled() {
                multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                    SimulatedUser::prudent_no_fail(),
                )
                .await;

                // Same result with lazy user, not able to skip without failures.
                multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                    SimulatedUser::lazy_sign_minimum([]),
                )
                .await
            }

            #[actix_rt::test]
            async fn multi_securified_entities() {
                multi_securified_entities_with_sim_user(Vector {
                    simulated_user: SimulatedUser::prudent_no_fail(),
                    expected: Expected {
                        successful_txs_signature_count: 32,
                        // We always end early
                        // `Device` FactorSourceKind never got used since it
                        // we are done after YubiKey.
                        signed_factor_source_kinds: IndexSet::<FactorSourceKind>::from_iter([
                            FactorSourceKind::Ledger,
                            FactorSourceKind::Arculus,
                            FactorSourceKind::Yubikey,
                        ]),
                        expected_skipped_factor_source_count: 0,
                    },
                })
                .await;

                multi_securified_entities_with_sim_user(Vector {
                    simulated_user: SimulatedUser::lazy_sign_minimum([]),
                    expected: Expected {
                        successful_txs_signature_count: 24,
                        // We always end early, this lazy user was able to skip
                        // Ledger.
                        signed_factor_source_kinds: IndexSet::<FactorSourceKind>::from_iter([
                            FactorSourceKind::Arculus,
                            FactorSourceKind::Yubikey,
                            FactorSourceKind::Device,
                        ]),
                        expected_skipped_factor_source_count: 2,
                    },
                })
                .await;
            }
        }
    }

    mod single_tx {
        use super::*;

        mod multiple_entities {
            use super::*;

            #[actix_rt::test]
            async fn prudent_user_single_tx_two_accounts_same_factor_source() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([
                    Account::unsecurified_mainnet(0, "A0", FactorSourceIDFromHash::fs0()),
                    Account::unsecurified_mainnet(1, "A1", FactorSourceIDFromHash::fs0()),
                ])]);

                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
                assert_eq!(
                    signatures
                        .into_iter()
                        .map(|s| s.derivation_path())
                        .collect::<HashSet<_>>(),
                    [
                        DerivationPath::account_tx(
                            NetworkID::Mainnet,
                            HDPathComponent::non_hardened(0)
                        ),
                        DerivationPath::account_tx(
                            NetworkID::Mainnet,
                            HDPathComponent::non_hardened(1)
                        ),
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>()
                )
            }

            #[actix_rt::test]
            async fn prudent_user_single_tx_two_accounts_different_factor_sources() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([
                    Account::a0(),
                    Account::a1(),
                ])]);

                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
            }
        }

        mod single_entity {

            use super::*;

            async fn prudent_user_single_tx_e0<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e0()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e0_assert_correct_intent_hash_is_signed<E: IsEntity>() {
                let tx = TXToSign::new([E::e0()]);
                let collector = SignaturesCollector::test_prudent([tx.clone()]);
                let signature = &collector.collect_signatures().await.all_signatures()[0];
                assert_eq!(signature.intent_hash(), &tx.intent_hash);
                assert_eq!(signature.derivation_path().entity_kind, E::kind());
            }

            async fn prudent_user_single_tx_e0_assert_correct_owner_has_signed<E: IsEntity>() {
                let entity = E::e0();
                let tx = TXToSign::new([entity.clone()]);
                let collector = SignaturesCollector::test_prudent([tx.clone()]);
                let signature = &collector.collect_signatures().await.all_signatures()[0];
                assert_eq!(signature.owned_factor_instance().owner, entity.address());
            }

            async fn prudent_user_single_tx_e0_assert_correct_owner_factor_instance_signed<
                E: IsEntity,
            >() {
                let entity = E::e0();
                let tx = TXToSign::new([entity.clone()]);
                let collector = SignaturesCollector::test_prudent([tx.clone()]);
                let signature = &collector.collect_signatures().await.all_signatures()[0];

                assert_eq!(
                    signature.owned_factor_instance().factor_instance(),
                    entity
                        .security_state()
                        .all_factor_instances()
                        .first()
                        .unwrap()
                );
            }

            async fn prudent_user_single_tx_e1<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e1()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e2<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e2()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e3<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e3()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e4<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e4()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
            }

            async fn prudent_user_single_tx_e5<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e5()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e6<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e6()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e7<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([E::e7()])]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();

                assert_eq!(signatures.len(), 5);
            }

            async fn lazy_sign_minimum_user_single_tx_e0<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e0()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_single_tx_e1<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e1()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_single_tx_e2<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e2()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_e3<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e3()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_e4<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e4()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
            }

            async fn lazy_sign_minimum_user_e5<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e5()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_e6<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e6()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();

                assert_eq!(signatures.len(), 2);
            }

            async fn lazy_sign_minimum_user_e7<E: IsEntity>() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::e7()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();

                assert_eq!(signatures.len(), 5);
            }

            async fn lazy_sign_minimum_user_e5_last_factor_used<E: IsEntity>() {
                let entity = E::e5();
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([entity.clone()]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);

                let signature = &signatures[0];

                assert_eq!(
                    signature
                        .owned_factor_instance()
                        .factor_instance()
                        .factor_source_id,
                    FactorSourceIDFromHash::fs4()
                );

                assert_eq!(
                    outcome.skipped_factor_sources(),
                    IndexSet::just(FactorSourceIDFromHash::fs1())
                )
            }

            async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity<
                E: IsEntity,
            >() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::securified_mainnet(
                        HDPathComponent::securified(0),
                        "all override",
                        |idx| {
                            MatrixOfFactorInstances::override_only(
                                HDFactorSource::all().into_iter().map(|f| {
                                    HierarchicalDeterministicFactorInstance::mainnet_tx_account(
                                        idx,
                                        f.factor_source_id(),
                                    )
                                }),
                            )
                        },
                    )]),
                ]);
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
                let signature = &signatures[0];
                assert_eq!(
                    signature
                        .owned_factor_instance()
                        .factor_instance()
                        .factor_source_id
                        .kind,
                    FactorSourceKind::Device
                );
            }

            async fn lazy_always_skip_user_single_tx_e0<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e0()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn fail_get_skipped_e0<E: IsEntity>() {
                let failing = IndexSet::<_>::from_iter([FactorSourceIDFromHash::fs0()]);
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [TXToSign::new([E::e0()])],
                    SimulatedFailures::with_simulated_failures(failing.clone()),
                );
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let skipped = outcome.skipped_factor_sources();
                assert_eq!(skipped, failing);
            }

            async fn lazy_always_skip_user_single_tx_e1<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e1()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_single_tx_e2<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e2()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e3<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e3()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e4<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e4()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e5<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e5()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e6<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e6()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e7<E: IsEntity>() {
                let collector =
                    SignaturesCollector::test_lazy_always_skip([TXToSign::new([E::e7()])]);
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn failure_e0<E: IsEntity>() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [TXToSign::new([E::e0()])],
                    SimulatedFailures::with_simulated_failures([FactorSourceIDFromHash::fs0()]),
                );
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
            }

            async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4<
                E: IsEntity,
            >() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [TXToSign::new([E::e4()])],
                    SimulatedFailures::with_simulated_failures([FactorSourceIDFromHash::fs3()]),
                );
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                assert_eq!(
                    outcome
                        .signatures_of_successful_transactions()
                        .into_iter()
                        .map(|f| f.factor_source_id())
                        .collect::<IndexSet<_>>(),
                    IndexSet::<_>::from_iter([
                        FactorSourceIDFromHash::fs0(),
                        FactorSourceIDFromHash::fs5()
                    ])
                );
            }

            async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4<
                E: IsEntity,
            >() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [TXToSign::new([E::e4()])],
                    SimulatedFailures::with_simulated_failures([FactorSourceIDFromHash::fs3()]),
                );
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                assert_eq!(
                    outcome.skipped_factor_sources(),
                    IndexSet::<_>::from_iter([FactorSourceIDFromHash::fs3()])
                );
            }

            mod account {
                use super::*;
                type E = Account;

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0() {
                    prudent_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0_assert_correct_intent_hash_is_signed() {
                    prudent_user_single_tx_e0_assert_correct_intent_hash_is_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0_assert_correct_owner_has_signed() {
                    prudent_user_single_tx_e0_assert_correct_owner_has_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0_assert_correct_owner_factor_instance_signed() {
                    prudent_user_single_tx_e0_assert_correct_owner_factor_instance_signed::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a1() {
                    prudent_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a2() {
                    prudent_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a3() {
                    prudent_user_single_tx_e3::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a4() {
                    prudent_user_single_tx_e4::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a5() {
                    prudent_user_single_tx_e5::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a6() {
                    prudent_user_single_tx_e6::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a7() {
                    prudent_user_single_tx_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_a0() {
                    lazy_sign_minimum_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_a1() {
                    lazy_sign_minimum_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_a2() {
                    lazy_sign_minimum_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a3() {
                    lazy_sign_minimum_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a4() {
                    lazy_sign_minimum_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a5() {
                    lazy_sign_minimum_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a6() {
                    lazy_sign_minimum_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a7() {
                    lazy_sign_minimum_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a5_last_factor_used() {
                    lazy_sign_minimum_user_e5_last_factor_used::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_account(
                ) {
                    lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_a0() {
                    lazy_always_skip_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn fail_get_skipped_a0() {
                    fail_get_skipped_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_a1() {
                    lazy_always_skip_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_a2() {
                    lazy_always_skip_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a3() {
                    lazy_always_skip_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a4() {
                    lazy_always_skip_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a5() {
                    lazy_always_skip_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a6() {
                    lazy_always_skip_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a7() {
                    lazy_always_skip_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn failure() {
                    failure_e0::<E>().await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4::<E>().await
                }
            }

            mod persona {
                use super::*;
                type E = Persona;

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0() {
                    prudent_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0_assert_correct_intent_hash_is_signed() {
                    prudent_user_single_tx_e0_assert_correct_intent_hash_is_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0_assert_correct_owner_has_signed() {
                    prudent_user_single_tx_e0_assert_correct_owner_has_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0_assert_correct_owner_factor_instance_signed() {
                    prudent_user_single_tx_e0_assert_correct_owner_factor_instance_signed::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p1() {
                    prudent_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p2() {
                    prudent_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p3() {
                    prudent_user_single_tx_e3::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p4() {
                    prudent_user_single_tx_e4::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p5() {
                    prudent_user_single_tx_e5::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p6() {
                    prudent_user_single_tx_e6::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p7() {
                    prudent_user_single_tx_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_p0() {
                    lazy_sign_minimum_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_p1() {
                    lazy_sign_minimum_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_p2() {
                    lazy_sign_minimum_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p3() {
                    lazy_sign_minimum_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p4() {
                    lazy_sign_minimum_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p5() {
                    lazy_sign_minimum_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p6() {
                    lazy_sign_minimum_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p7() {
                    lazy_sign_minimum_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p5_last_factor_used() {
                    lazy_sign_minimum_user_e5_last_factor_used::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_account(
                ) {
                    lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_p0() {
                    lazy_always_skip_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn fail_get_skipped_p0() {
                    fail_get_skipped_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_p1() {
                    lazy_always_skip_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_p2() {
                    lazy_always_skip_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p3() {
                    lazy_always_skip_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p4() {
                    lazy_always_skip_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p5() {
                    lazy_always_skip_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p6() {
                    lazy_always_skip_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p7() {
                    lazy_always_skip_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn failure() {
                    failure_e0::<E>().await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4::<E>().await
                }
            }
        }
    }
}
