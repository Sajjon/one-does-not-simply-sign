use crate::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct StatelessDummyIndices;

impl UsedDerivationIndices for StatelessDummyIndices {
    fn next_derivation_index_with_request(
        &self,
        request: CreateNextDerivationPathRequest,
    ) -> DerivationIndex {
        request.key_space.range().start
    }
}
