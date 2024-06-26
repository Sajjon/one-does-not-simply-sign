use crate::prelude::*;

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

impl FactorInstance {
    pub fn f(idx: u32) -> impl Fn(FactorSourceID) -> Self {
        move |id: FactorSourceID| Self::new(idx, id)
    }
}

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

    /// Ida | 7 | Securified { Threshold only # 5/5 }
    pub fn a7() -> Self {
        type F = FactorSourceID;
        Self::securified(7, "Ida", |idx| {
            let fi = FactorInstance::f(idx);
            MatrixOfFactorInstances::threshold_only(
                [F::fs2(), F::fs6(), F::fs7(), F::fs8(), F::fs9()].map(&fi),
                5,
            )
        })
    }
}
