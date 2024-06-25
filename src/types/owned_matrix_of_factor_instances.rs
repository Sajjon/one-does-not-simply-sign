use crate::prelude::*;

pub type OwnedMatrixOfFactorInstances = Owned<MatrixOfFactorInstances>;

impl OwnedMatrixOfFactorInstances {
    pub fn owned_matrix(
        owner: AccountAddressOrIdentityAddress,
        matrix: MatrixOfFactorInstances,
    ) -> Self {
        Self::new(owner, matrix)
    }
}

impl From<&Entity> for OwnedMatrixOfFactorInstances {
    fn from(value: &Entity) -> Self {
        let matrix = match value.security_state.clone() {
            EntitySecurityState::Securified(matrix) => matrix.clone(),
            EntitySecurityState::Unsecured(instance) => MatrixOfFactorInstances::from(instance),
        };
        OwnedMatrixOfFactorInstances::owned_matrix(value.address.clone(), matrix)
    }
}
