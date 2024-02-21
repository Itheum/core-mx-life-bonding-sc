use crate::storage::{self, Bond, Compensation};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {
    #[view(getBond)]
    fn get_bond(&self, bond_id: u64) -> Bond<Self::Api> {
        Bond {
            bond_id,
            address: self.bond_address(bond_id).get(),
            token_identifier: self.bond_token_identifier(bond_id).get(),
            nonce: self.bond_nonce(bond_id).get(),
            lock_period: self.bond_lock_period(bond_id).get(),
            bond_timestamp: self.bond_timestamp(bond_id).get(),
            unbound_timestamp: self.unbound_timestamp(bond_id).get(),
            bond_amount: self.bond_amount(bond_id).get(),
        }
    }

    #[view(getCompensation)]
    fn get_compensation(
        &self,
        token_identifier: TokenIdentifier,
        nonce: u64,
    ) -> Compensation<Self::Api> {
        self.compensations(&token_identifier, nonce).get()
    }

    #[view(getBondsByTokenIdentifierNonce)]
    fn get_bonds_by_token_identifier_nonce(
        &self,
        input: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>,
    ) -> ManagedVec<Bond<Self::Api>> {
        let bonds = input
            .into_iter()
            .map(|value| {
                let (token_identifier, nonce) = value.into_tuple();
                let bond_id = self.object_to_id().get_id((token_identifier, nonce));
                self.get_bond(bond_id)
            })
            .collect::<ManagedVec<Bond<Self::Api>>>();

        bonds
    }
    #[view(getBonds)]
    fn get_bonds(&self, bond_ids: MultiValueEncoded<u64>) -> ManagedVec<Bond<Self::Api>> {
        let bonds = bond_ids
            .into_iter()
            .map(|bond_id| self.get_bond(bond_id))
            .collect::<ManagedVec<Bond<Self::Api>>>();
        bonds
    }

    #[view(getAddressBonds)]
    fn get_address_bonds(&self, address: ManagedAddress<Self::Api>) -> ManagedVec<Bond<Self::Api>> {
        let bonds = self
            .address_bonds(&address)
            .into_iter()
            .map(|bond_id| self.get_bond(bond_id))
            .collect::<ManagedVec<Bond<Self::Api>>>();

        bonds
    }

    #[view(getAllBonds)]
    fn get_all_bonds(&self) -> ManagedVec<Bond<Self::Api>> {
        let bonds = self
            .bonds()
            .into_iter()
            .map(|bond_id| self.get_bond(bond_id))
            .collect();

        bonds
    }
}
