//! Question: Is there any difference between BatchSigningDrivers and
//! SingleSigningDrivers other than the fact that BatchSigningDerivers can sign
//! many transactions with many derivations paths at once?

mod types;

pub mod prelude {
    pub use crate::types::*;

    pub use async_trait::async_trait;
    pub use derivative::Derivative;
    pub use indexmap::{IndexMap, IndexSet};
    pub use itertools::Itertools;
    pub use std::cell::RefCell;
    pub use std::time::SystemTime;
    pub use uuid::Uuid;

    pub use std::{
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        sync::Arc,
    };
}

pub use prelude::*;

#[cfg(test)]
impl SignaturesBuildingCoordinator {
    pub fn new_test(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
        simulated_user: SimulatedUser,
    ) -> Self {
        Self::new(
            all_factor_sources_in_profile.into_iter().collect(),
            transactions.into_iter().collect(),
            Arc::new(TestSigningDriversContext::new(simulated_user)),
        )
    }
    pub fn test_prudent_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::new_test(
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::Prudent,
        )
    }

    pub fn test_prudent(transactions: impl IntoIterator<Item = TransactionIntent>) -> Self {
        Self::test_prudent_with_factors(FactorSource::all(), transactions)
    }

    pub fn test_lazy_sign_minimum_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::new_test(
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::Lazy(Laziness::SignMinimum),
        )
    }

    pub fn test_lazy_sign_minimum(
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::test_lazy_sign_minimum_with_factors(FactorSource::all(), transactions)
    }

    pub fn test_lazy_always_skip_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::new_test(
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::Lazy(Laziness::AlwaysSkip),
        )
    }

    pub fn test_lazy_always_skip(
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::test_lazy_always_skip_with_factors(FactorSource::all(), transactions)
    }
}

impl FactorSource {
    /// Device
    pub fn fs0() -> Self {
        Self::device()
    }

    /// Ledger
    pub fn fs1() -> Self {
        Self::ledger()
    }

    /// Ledger
    pub fn fs2() -> Self {
        Self::ledger()
    }

    /// Arculus
    pub fn fs3() -> Self {
        Self::arculus()
    }

    /// Arculus
    pub fn fs4() -> Self {
        Self::arculus()
    }

    /// Yubikey
    pub fn fs5() -> Self {
        Self::yubikey()
    }

    /// Yubikey
    pub fn fs6() -> Self {
        Self::yubikey()
    }

    /// Off Device
    pub fn fs7() -> Self {
        Self::off_device()
    }

    /// Off Device
    pub fn fs8() -> Self {
        Self::off_device()
    }

    /// Security Questions
    pub fn fs9() -> Self {
        Self::security_question()
    }

    pub fn all() -> IndexSet<Self> {
        IndexSet::from_iter(ALL_FACTOR_SOURCES.clone())
    }
}

use once_cell::sync::Lazy;

pub static ALL_FACTOR_SOURCES: Lazy<[FactorSource; 10]> = Lazy::new(|| {
    [
        FactorSource::fs0(),
        FactorSource::fs1(),
        FactorSource::fs2(),
        FactorSource::fs3(),
        FactorSource::fs4(),
        FactorSource::fs5(),
        FactorSource::fs6(),
        FactorSource::fs7(),
        FactorSource::fs8(),
        FactorSource::fs9(),
    ]
});

pub fn fs_at(index: usize) -> FactorSource {
    ALL_FACTOR_SOURCES[index].clone()
}

pub fn fs_id_at(index: usize) -> FactorSourceID {
    fs_at(index).id
}

impl FactorSourceID {
    /// Device
    pub fn fs0() -> Self {
        fs_id_at(0)
    }

    /// Ledger
    pub fn fs1() -> Self {
        fs_id_at(1)
    }

    /// Ledger
    pub fn fs2() -> Self {
        fs_id_at(2)
    }

    /// Arculus
    pub fn fs3() -> Self {
        fs_id_at(3)
    }
    /// Arculus
    pub fn fs4() -> Self {
        fs_id_at(4)
    }
    /// Yubikey
    pub fn fs5() -> Self {
        fs_id_at(5)
    }
    /// Yubikey
    pub fn fs6() -> Self {
        fs_id_at(6)
    }
    /// Off Device
    pub fn fs7() -> Self {
        fs_id_at(7)
    }
    /// Off Device
    pub fn fs8() -> Self {
        fs_id_at(8)
    }
    /// Security Questions
    pub fn fs9() -> Self {
        fs_id_at(9)
    }
}

#[cfg(test)]
impl FactorInstance {
    pub fn f(idx: u32) -> impl Fn(FactorSourceID) -> Self {
        move |id: FactorSourceID| Self::new(idx, id)
    }
}

#[cfg(test)]
impl Entity {
    /// Alice | 0 | Unsecurified { Device }
    pub fn a0() -> Self {
        Self::unsecurified(0, "Alice", FactorSourceID::fs0())
    }

    /// Bob | 1 | Unsecurified { Ledger }
    pub fn a1() -> Self {
        Self::unsecurified(1, "Bob", FactorSourceID::fs1())
    }

    /// Carla | 2 | Securified { Single Threshold only }
    pub fn a2() -> Self {
        Self::securified(2, "Carla", |idx| {
            MatrixOfFactorInstances::single_threshold(FactorInstance::new(
                idx,
                FactorSourceID::fs0(),
            ))
        })
    }

    /// David | 3 | Securified { Single Override only }
    pub fn a3() -> Self {
        Self::securified(3, "David", |idx| {
            MatrixOfFactorInstances::single_override(FactorInstance::new(
                idx,
                FactorSourceID::fs1(),
            ))
        })
    }

    /// Emily | 4 | Securified { Threshold factors only #3 }
    pub fn a4() -> Self {
        type F = FactorSourceID;
        Self::securified(4, "Emily", |idx| {
            MatrixOfFactorInstances::threshold_only(
                [F::fs0(), F::fs3(), F::fs5()].map(FactorInstance::f(idx)),
                2,
            )
        })
    }

    /// Frank | 5 | Securified { Override factors only #2 }
    pub fn a5() -> Self {
        type F = FactorSourceID;
        Self::securified(5, "Frank", |idx| {
            MatrixOfFactorInstances::override_only([F::fs1(), F::fs4()].map(FactorInstance::f(idx)))
        })
    }

    /// Grace | 6 | Securified { Threshold #3 and Override factors #2  }
    pub fn a6() -> Self {
        type F = FactorSourceID;
        Self::securified(6, "Grace", |idx| {
            let fi = FactorInstance::f(idx);
            MatrixOfFactorInstances::new(
                [F::fs0(), F::fs3(), F::fs5()].map(&fi),
                2,
                [F::fs1(), F::fs4()].map(&fi),
            )
        })
    }

    pub fn all() -> IndexSet<Self> {
        IndexSet::from_iter([
            Entity::a0(),
            Entity::a1(),
            Entity::a2(),
            Entity::a3(),
            Entity::a4(),
            Entity::a5(),
            Entity::a6(),
        ])
    }
}

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
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a0()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_intent_hash_is_signed() {
        let tx = TransactionIntent::new([Entity::a0()]);
        let context = SignaturesBuildingCoordinator::test_prudent([tx.clone()]);
        let signature = &context.sign().await.unwrap().all_signatures()[0];
        assert_eq!(signature.intent_hash, tx.intent_hash);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_owner_has_signed() {
        let account = Entity::a0();
        let tx = TransactionIntent::new([account.clone()]);
        let context = SignaturesBuildingCoordinator::test_prudent([tx.clone()]);
        let signature = &context.sign().await.unwrap().all_signatures()[0];
        assert_eq!(signature.owned_factor_instance.owner, account.address);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a0_assert_correct_owner_factor_instance_signed() {
        let account = Entity::a0();
        let tx = TransactionIntent::new([account.clone()]);
        let context = SignaturesBuildingCoordinator::test_prudent([tx.clone()]);
        let signature = &context.sign().await.unwrap().all_signatures()[0];

        assert_eq!(
            &signature.owned_factor_instance.factor_instance,
            account
                .security_state
                .all_factor_instances()
                .first()
                .unwrap()
        );
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a1() {
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a1()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a2() {
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a2()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a3() {
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a3()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a4() {
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a4()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 3);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a5() {
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a5()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn prudent_user_single_tx_a6() {
        let context =
            SignaturesBuildingCoordinator::test_prudent([TransactionIntent::new([Entity::a6()])]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 5);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a0() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a0()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a1() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a1()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_single_tx_a2() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a2()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a3() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a3()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a4() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a4()]),
        ]);
        let outcome = context.sign().await;
        let signatures = outcome.unwrap().all_signatures();
        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a5() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a5()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a6() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::a6()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();

        assert_eq!(signatures.len(), 2);
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_user_a5_last_factor_used() {
        let entity = Entity::a5();
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([entity.clone()]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);

        let signature = &signatures[0];

        assert_eq!(
            signature
                .owned_factor_instance
                .factor_instance
                .factor_source_id,
            FactorSourceID::fs4()
        );
    }

    #[actix_rt::test]
    async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device() {
        let context = SignaturesBuildingCoordinator::test_lazy_sign_minimum([
            TransactionIntent::new([Entity::securified(0, "all override", |idx| {
                MatrixOfFactorInstances::override_only(
                    FactorSource::all()
                        .into_iter()
                        .map(|f| FactorInstance::new(idx, f.id.clone())),
                )
            })]),
        ]);
        let signatures = context.sign().await.unwrap().all_signatures();
        assert_eq!(signatures.len(), 1);
        let signature = &signatures[0];
        assert_eq!(
            signature
                .owned_factor_instance
                .factor_instance
                .factor_source_id
                .kind,
            FactorSourceKind::Device
        );
    }
}
