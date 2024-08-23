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
        assert_eq!(FactorSourceID::fs0(), FactorSourceID::fs0());
        assert_eq!(FactorSourceID::fs1(), FactorSourceID::fs1());
        assert_ne!(FactorSourceID::fs0(), FactorSourceID::fs1());
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
            .map(|i| DerivationPath::new(Mainnet, Account, T9n, i))
            .collect::<IndexSet<_>>();
        let collector = KeysCollector::new(
            FactorSource::all(),
            [(factor_source.factor_source_id(), paths.clone())]
                .into_iter()
                .collect::<IndexMap<FactorSourceID, IndexSet<DerivationPath>>>(),
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
                .map(|i| DerivationPath::new(Mainnet, Account, T9n, i))
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
            let path = DerivationPath::account_tx(Mainnet, 0);
            let paths = IndexSet::from_iter([path]);
            let factor_sources = FactorSource::all();

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
                .map(|i| DerivationPath::new(Mainnet, Account, T9n, i))
                .collect::<IndexSet<_>>();

            let factor_sources = FactorSource::all();

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
                    .map(|i| DerivationPath::new(Mainnet, Account, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Stokenet, Account, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Mainnet, Identity, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Stokenet, Identity, T9n, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Mainnet, Account, Rola, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Stokenet, Account, Rola, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Mainnet, Identity, Rola, i)),
            );

            paths.extend(
                [0, 1, 2]
                    .into_iter()
                    .map(|i| DerivationPath::new(Stokenet, Identity, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Mainnet, Account, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Stokenet, Account, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Mainnet, Identity, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Stokenet, Identity, T9n, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Mainnet, Account, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Stokenet, Account, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Mainnet, Identity, Rola, i)),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    KeySpace::SPLIT,
                    KeySpace::SPLIT + 1,
                    KeySpace::SPLIT + 2,
                ]
                .into_iter()
                .map(|i| DerivationPath::new(Stokenet, Identity, Rola, i)),
            );

            let factor_sources = FactorSource::all();

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
            index: DerivationIndex,
        }

        async fn do_test(
            key_space: KeySpace,
            factor_source: &FactorSource,
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
                factor_source: &FactorSource,
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
                        index: KeySpace::SPLIT,
                    },
                )
                .await
            }

            mod account {
                use super::*;

                async fn each_factor(network_id: NetworkID, key_kind: CAP26KeyKind) {
                    for factor_source in FactorSource::all().iter() {
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
                factor_source: &FactorSource,
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
                    Expected { index: 0 },
                )
                .await
            }

            mod account {
                use super::*;

                async fn each_factor(network_id: NetworkID, key_kind: CAP26KeyKind) {
                    for factor_source in FactorSource::all().iter() {
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
                    for factor_source in FactorSource::all().iter() {
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

        #[actix_rt::test]
        async fn multi_accounts_multi_personas_all_single_factor_controlled() {
            let factor_sources = &FactorSource::all();
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
                Arc::new(TestSignatureCollectingInteractors::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &profile,
            )
            .unwrap();

            let outcome = collector.collect_signatures().await;
            assert!(outcome.signatures_of_failed_transactions().is_empty());
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
        }

        #[actix_rt::test]
        async fn multi_securified_entities() {}
    }

    mod single_tx {
        use super::*;

        mod multiple_entities {
            use super::*;

            #[actix_rt::test]
            async fn prudent_user_single_tx_two_accounts_same_factor_source() {
                let collector = SignaturesCollector::test_prudent([TXToSign::new([
                    Account::unsecurified_mainnet(0, "A0", FactorSourceID::fs0()),
                    Account::unsecurified_mainnet(1, "A1", FactorSourceID::fs0()),
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
                        DerivationPath::account_tx(NetworkID::Mainnet, 0),
                        DerivationPath::account_tx(NetworkID::Mainnet, 1),
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
                    FactorSourceID::fs4()
                );

                assert_eq!(
                    outcome.skipped_factor_sources(),
                    IndexSet::just(FactorSourceID::fs1())
                )
            }

            async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity<
                E: IsEntity,
            >() {
                let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
                    TXToSign::new([E::securified_mainnet(0, "all override", |idx| {
                        MatrixOfFactorInstances::override_only(FactorSource::all().into_iter().map(
                            |f| {
                                HierarchicalDeterministicFactorInstance::mainnet_tx_account(
                                    idx,
                                    f.factor_source_id(),
                                )
                            },
                        ))
                    })]),
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
                let failing = IndexSet::<_>::from_iter([FactorSourceID::fs0()]);
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
                    SimulatedFailures::with_simulated_failures([FactorSourceID::fs0()]),
                );
                let outcome = collector.collect_signatures().await;
                assert!(!outcome.successful());
            }

            async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4<
                E: IsEntity,
            >() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [TXToSign::new([E::e4()])],
                    SimulatedFailures::with_simulated_failures([FactorSourceID::fs3()]),
                );
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                assert_eq!(
                    outcome
                        .signatures_of_successful_transactions()
                        .into_iter()
                        .map(|f| f.factor_source_id())
                        .collect::<IndexSet<_>>(),
                    IndexSet::<_>::from_iter([FactorSourceID::fs0(), FactorSourceID::fs5()])
                );
            }

            async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4<
                E: IsEntity,
            >() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [TXToSign::new([E::e4()])],
                    SimulatedFailures::with_simulated_failures([FactorSourceID::fs3()]),
                );
                let outcome = collector.collect_signatures().await;
                assert!(outcome.successful());
                assert_eq!(
                    outcome.skipped_factor_sources(),
                    IndexSet::<_>::from_iter([FactorSourceID::fs3()])
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
