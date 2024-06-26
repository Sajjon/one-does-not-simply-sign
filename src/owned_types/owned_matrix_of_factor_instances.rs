use crate::prelude::*;

/// A `MatrixOfFactorInstances` with a known owner - an account or persona.
pub type OwnedMatrixOfFactorInstances = Owned<MatrixOfFactorInstances>;

impl OwnedMatrixOfFactorInstances {
    /// Constructs a new `OwnedFactorInstance`.
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
