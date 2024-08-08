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
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a0()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_intent_hash_is_signed() {
        let tx = TransactionIntent::new([Entity::a0()]);
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([tx.clone()]);
        let signature = &coordinator.use_factor_sources().await.all_signatures()[0];
        assert_eq!(signature.intent_hash(), &tx.intent_hash);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_owner_has_signed() {
        let account = Entity::a0();
        let tx = TransactionIntent::new([account.clone()]);
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([tx.clone()]);
        let signature = &coordinator.use_factor_sources().await.all_signatures()[0];
        assert_eq!(signature.owned_factor_instance().owner, account.address);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_owner_factor_instance_signed() {
        let account = Entity::a0();
        let tx = TransactionIntent::new([account.clone()]);
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([tx.clone()]);
        let signature = &coordinator.use_factor_sources().await.all_signatures()[0];

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
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a1()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a2() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a2()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a3() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a3()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a4() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a4()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a5() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a5()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a6() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a6()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a7() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent([TransactionIntent::new(
            [Entity::a7()],
        )]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();

        assert_eq!(signatures.len(), 5);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a0() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a0()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a1() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a1()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a2() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a2()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a3() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a3()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a4() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a4()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a5() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a5()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a6() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a6()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();

        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a7() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::a7()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
        let signatures = outcome.all_signatures();

        assert_eq!(signatures.len(), 5);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a5_last_factor_used() {
        let entity = Entity::a5();
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([entity.clone()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
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
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_sign_minimum_no_retry([
            TransactionIntent::new([Entity::securified(0, "all override", |idx| {
                MatrixOfFactorInstances::override_only(
                    FactorSource::all()
                        .into_iter()
                        .map(|f| FactorInstance::new(idx, f.id)),
                )
            })]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
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
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a0()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_single_tx_a1() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a1()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_single_tx_a2() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a2()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a3() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a3()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a4() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a4()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a5() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a5()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a6() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a6()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn lazy_always_skip_user_a7() {
        let coordinator = FactorResultsBuildingCoordinator::test_lazy_always_skip([
            TransactionIntent::new([Entity::a7()]),
        ]);
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
        let signatures = outcome.all_signatures();
        assert!(signatures.is_empty());
    }

    #[actix_rt::test]
    async fn failure_user_does_not_retry() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent_with_retry(
            [TransactionIntent::new([Entity::a0()])],
            SimulatedUserRetries::with_simulated_failures(0, [(FactorSourceID::fs0(), usize::MAX)]),
        );
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
    }

    #[actix_rt::test]
    async fn failure_user_does_not_retry_enough() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent_with_retry(
            [TransactionIntent::new([Entity::a0()])],
            SimulatedUserRetries::with_simulated_failures(1, [(FactorSourceID::fs0(), usize::MAX)]),
        );
        let outcome = coordinator.use_factor_sources().await;
        assert!(!outcome.successful());
    }

    async fn failure_user_succeeds_after_nth_retry(n: usize) {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent_with_retry(
            [TransactionIntent::new([Entity::a0()])],
            SimulatedUserRetries::with_simulated_failures(n, [(FactorSourceID::fs0(), n)]),
        );
        let outcome = coordinator.use_factor_sources().await;
        assert!(outcome.successful());
    }

    #[actix_rt::test]
    async fn test_failure_user_succeeds_after_nth_retry() {
        failure_user_succeeds_after_nth_retry(1).await;
        failure_user_succeeds_after_nth_retry(2).await;
        failure_user_succeeds_after_nth_retry(3).await;
        failure_user_succeeds_after_nth_retry(10).await;
    }

    #[actix_rt::test]
    async fn building_can_succeed_even_if_one_factor_source_fails_ids_of_successful_tx() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent_with_retry(
            [TransactionIntent::new([Entity::a4()])],
            SimulatedUserRetries::with_simulated_failures(2, [(FactorSourceID::fs3(), 99)]),
        );
        let outcome = coordinator.use_factor_sources().await;
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
    async fn building_can_succeed_even_if_one_factor_source_fails_ids_of_failed_tx() {
        let coordinator = FactorResultsBuildingCoordinator::test_prudent_with_retry(
            [TransactionIntent::new([Entity::a4()])],
            SimulatedUserRetries::with_simulated_failures(2, [(FactorSourceID::fs3(), 99)]),
        );
        let outcome = coordinator.use_factor_sources().await;
        assert_eq!(
            outcome.skipped_factor_sources(),
            IndexSet::<_>::from_iter([FactorSourceID::fs3()])
        );
    }
}
