use crate::prelude::*;

impl HDFactorSource {
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

pub static ID_STEPPER: Lazy<UuidStepper> = Lazy::new(UuidStepper::new);

impl UuidStepper {
    pub fn next() -> Uuid {
        ID_STEPPER._next()
    }
}

pub static ALL_FACTOR_SOURCES: Lazy<[HDFactorSource; 10]> = Lazy::new(|| {
    [
        HDFactorSource::fs0(),
        HDFactorSource::fs1(),
        HDFactorSource::fs2(),
        HDFactorSource::fs3(),
        HDFactorSource::fs4(),
        HDFactorSource::fs5(),
        HDFactorSource::fs6(),
        HDFactorSource::fs7(),
        HDFactorSource::fs8(),
        HDFactorSource::fs9(),
    ]
});

pub fn fs_at(index: usize) -> HDFactorSource {
    ALL_FACTOR_SOURCES[index].clone()
}

pub fn fs_id_at(index: usize) -> FactorSourceIDFromHash {
    fs_at(index).factor_source_id()
}

impl FactorSourceIDFromHash {
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

impl HierarchicalDeterministicFactorInstance {
    pub fn f(entity_kind: CAP26EntityKind, idx: u32) -> impl Fn(FactorSourceIDFromHash) -> Self {
        move |id: FactorSourceIDFromHash| {
            Self::mainnet_tx(entity_kind, HDPathComponent::non_hardened(idx), id)
        }
    }
}

impl MatrixOfFactorInstances {
    /// Securified { Single Threshold only }
    pub fn m2<F>(fi: F) -> Self
    where
        F: Fn(FactorSourceIDFromHash) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_threshold(fi(FactorSourceIDFromHash::fs0()))
    }

    /// Securified { Single Override only }
    pub fn m3<F>(fi: F) -> Self
    where
        F: Fn(FactorSourceIDFromHash) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_override(fi(FactorSourceIDFromHash::fs1()))
    }

    /// Securified { Threshold factors only #3 }
    pub fn m4<F>(fi: F) -> Self
    where
        F: Fn(FactorSourceIDFromHash) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::threshold_only([F::fs0(), F::fs3(), F::fs5()].map(fi), 2)
    }

    /// Securified { Override factors only #2 }
    pub fn m5<F>(fi: F) -> Self
    where
        F: Fn(FactorSourceIDFromHash) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::override_only([F::fs1(), F::fs4()].map(&fi))
    }

    /// Securified { Threshold #3 and Override factors #2  }
    pub fn m6<F>(fi: F) -> Self
    where
        F: Fn(FactorSourceIDFromHash) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::new(
            [F::fs0(), F::fs3(), F::fs5()].map(&fi),
            2,
            [F::fs1(), F::fs4()].map(&fi),
        )
    }

    /// Securified { Threshold only # 5/5 }
    pub fn m7<F>(fi: F) -> Self
    where
        F: Fn(FactorSourceIDFromHash) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::threshold_only(
            [F::fs2(), F::fs6(), F::fs7(), F::fs8(), F::fs9()].map(&fi),
            5,
        )
    }
}

impl Account {
    /// Alice | 0 | Unsecurified { Device }
    pub fn a0() -> Self {
        Self::unsecurified_mainnet(0, "Alice", FactorSourceIDFromHash::fs0())
    }

    /// Bob | 1 | Unsecurified { Ledger }
    pub fn a1() -> Self {
        Self::unsecurified_mainnet(1, "Bob", FactorSourceIDFromHash::fs1())
    }

    /// Carla | 2 | Securified { Single Threshold only }
    pub fn a2() -> Self {
        Self::securified_mainnet(2, "Carla", |idx| {
            MatrixOfFactorInstances::m2(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// David | 3 | Securified { Single Override only }
    pub fn a3() -> Self {
        Self::securified_mainnet(3, "David", |idx| {
            MatrixOfFactorInstances::m3(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Emily | 4 | Securified { Threshold factors only #3 }
    pub fn a4() -> Self {
        Self::securified_mainnet(4, "Emily", |idx| {
            MatrixOfFactorInstances::m4(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Frank | 5 | Securified { Override factors only #2 }
    pub fn a5() -> Self {
        Self::securified_mainnet(5, "Frank", |idx| {
            MatrixOfFactorInstances::m5(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Grace | 6 | Securified { Threshold #3 and Override factors #2  }
    pub fn a6() -> Self {
        Self::securified_mainnet(6, "Grace", |idx| {
            MatrixOfFactorInstances::m6(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Ida | 7 | Securified { Threshold only # 5/5 }
    pub fn a7() -> Self {
        Self::securified_mainnet(7, "Ida", |idx| {
            MatrixOfFactorInstances::m7(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }
}

impl Persona {
    /// Satoshi | 0 | Unsecurified { Device }
    pub fn p0() -> Self {
        Self::unsecurified_mainnet(0, "Satoshi", FactorSourceIDFromHash::fs0())
    }

    /// Batman | 1 | Unsecurified { Ledger }
    pub fn p1() -> Self {
        Self::unsecurified_mainnet(1, "Batman", FactorSourceIDFromHash::fs1())
    }

    /// Ziggy | 2 | Securified { Single Threshold only }
    pub fn p2() -> Self {
        Self::securified_mainnet(2, "Ziggy", |idx| {
            MatrixOfFactorInstances::m2(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Superman | 3 | Securified { Single Override only }
    pub fn p3() -> Self {
        Self::securified_mainnet(3, "Superman", |idx| {
            MatrixOfFactorInstances::m3(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Banksy | 4 | Securified { Threshold factors only #3 }
    pub fn p4() -> Self {
        Self::securified_mainnet(4, "Banksy", |idx| {
            MatrixOfFactorInstances::m4(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Voltaire | 5 | Securified { Override factors only #2 }
    pub fn p5() -> Self {
        Self::securified_mainnet(5, "Voltaire", |idx| {
            MatrixOfFactorInstances::m5(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Kasparov | 6 | Securified { Threshold #3 and Override factors #2  }
    pub fn p6() -> Self {
        Self::securified_mainnet(6, "Kasparov", |idx| {
            MatrixOfFactorInstances::m6(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    /// Pelé | 7 | Securified { Threshold only # 5/5 }
    pub fn p7() -> Self {
        Self::securified_mainnet(7, "Pelé", |idx| {
            MatrixOfFactorInstances::m7(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }
}
