use signing::prelude::*;

#[cfg(test)]
mod tests {

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
            Entity::a0().security_state.all_factor_instances(),
            Entity::a0().security_state.all_factor_instances()
        );
        assert_eq!(
            Entity::a6().security_state.all_factor_instances(),
            Entity::a6().security_state.all_factor_instances()
        );
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a0()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_intent_hash_is_signed() {
        let tx = TransactionIntent::new([Entity::a0()]);
        let collector = SignaturesCollector::test_prudent([tx.clone()]);
        let signature = &collector.collect_signatures().await.all_signatures()[0];
        assert_eq!(signature.intent_hash(), &tx.intent_hash);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_owner_has_signed() {
        let account = Entity::a0();
        let tx = TransactionIntent::new([account.clone()]);
        let collector = SignaturesCollector::test_prudent([tx.clone()]);
        let signature = &collector.collect_signatures().await.all_signatures()[0];
        assert_eq!(signature.owned_factor_instance().owner, account.address);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_owner_factor_instance_signed() {
        let account = Entity::a0();
        let tx = TransactionIntent::new([account.clone()]);
        let collector = SignaturesCollector::test_prudent([tx.clone()]);
        let signature = &collector.collect_signatures().await.all_signatures()[0];

        assert_eq!(
            signature.owned_factor_instance().factor_instance(),
            account
                .security_state
                .all_factor_instances()
                .first()
                .unwrap()
        );
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a1() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a1()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a2() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a2()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a3() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a3()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a4() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a4()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a5() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a5()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a6() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a6()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a7() {
        let collector = SignaturesCollector::test_prudent([TransactionIntent::new([Entity::a7()])]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();

        assert_eq!(signatures.len(), 5);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a0() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a0()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a1() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a1()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a2() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a2()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a3() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a3()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a4() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a4()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a5() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a5()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a6() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a6()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();

        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a7() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::a7()]),
        ]);
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();

        assert_eq!(signatures.len(), 5);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a5_last_factor_used() {
        let entity = Entity::a5();
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([entity.clone()]),
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

    #[actix_rt::test]
    async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device() {
        let collector = SignaturesCollector::test_lazy_sign_minimum_no_failures([
            TransactionIntent::new([Entity::securified(0, "all override", |idx| {
                MatrixOfFactorInstances::override_only(
                    FactorSource::all()
                        .into_iter()
                        .map(|f| FactorInstance::new(idx, f.id)),
                )
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

    #[actix_rt::test]
    async fn lazy_always_skip_user_single_tx_a0() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a0()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn fail_get_skipped() {
        let failing = IndexSet::<_>::from_iter([FactorSourceID::fs0()]);
        let collector = SignaturesCollector::test_prudent_with_failures(
            [TransactionIntent::new([Entity::a0()])],
            SimulatedFailures::with_simulated_failures(failing.clone()),
        );
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let skipped = outcome.skipped_factor_sources();
        assert_eq!(skipped, failing);
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_single_tx_a1() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a1()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_single_tx_a2() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a2()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a3() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a3()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a4() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a4()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a5() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a5()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a6() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a6()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a7() {
        let collector =
            SignaturesCollector::test_lazy_always_skip([TransactionIntent::new([Entity::a7()])]);
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn failure() {
        let collector = SignaturesCollector::test_prudent_with_failures(
            [TransactionIntent::new([Entity::a0()])],
            SimulatedFailures::with_simulated_failures([FactorSourceID::fs0()]),
        );
        let outcome = collector.collect_signatures().await;
        assert!(!outcome.successful());
    }

    #[actix_rt::test]
    async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx() {
        let collector = SignaturesCollector::test_prudent_with_failures(
            [TransactionIntent::new([Entity::a4()])],
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

    #[actix_rt::test]
    async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx() {
        let collector = SignaturesCollector::test_prudent_with_failures(
            [TransactionIntent::new([Entity::a4()])],
            SimulatedFailures::with_simulated_failures([FactorSourceID::fs3()]),
        );
        let outcome = collector.collect_signatures().await;
        assert!(outcome.successful());
        assert_eq!(
            outcome.skipped_factor_sources(),
            IndexSet::<_>::from_iter([FactorSourceID::fs3()])
        );
    }
}
