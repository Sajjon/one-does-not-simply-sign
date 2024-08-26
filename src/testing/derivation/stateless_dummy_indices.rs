use crate::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct StatelessDummyIndices;

impl UsedDerivationIndices for StatelessDummyIndices {
    fn next_derivation_index_with_request(
        &self,
        request: CreateNextDerivationPathRequest,
    ) -> HDPathComponent {
        match request.key_space {
            KeySpace::Securified => HDPathComponent::non_hardened(BIP32_SECURIFIED_HALF),
            KeySpace::Unsecurified => HDPathComponent::non_hardened(0),
        }
    }
}
