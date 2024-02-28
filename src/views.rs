use crate::storage::{self, Bond, Compensation, Refund};

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
    fn get_compensation(&self, compensation_id: u64) -> Compensation<Self::Api> {
        Compensation {
            compensation_id,
            token_identifier: self.compensation_token_identifer(compensation_id).get(),
            nonce: self.compensation_nonce(compensation_id).get(),
            accumulated_amount: self.compensation_accumulated_amount(compensation_id).get(),
            proof_amount: self.compensation_proof_amount(compensation_id).get(),
            end_date: self.compensation_end_date(compensation_id).get(),
        }
    }

    #[view(getCompensations)]
    fn get_compensations(
        &self,
        input: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>,
    ) -> ManagedVec<Compensation<Self::Api>> {
        let compensations = input
            .into_iter()
            .filter_map(|value| {
                let (token_identifier, nonce) = value.into_tuple();
                let compensation_id = self.compensations_ids().get_id((token_identifier, nonce));
                if compensation_id != 0 {
                    Some(self.get_compensation(compensation_id))
                } else {
                    None
                }
            })
            .collect::<ManagedVec<Compensation<Self::Api>>>();

        compensations
    }

    #[view(getPagedCompensations)]
    fn get_paged_compensations(
        &self,
        start_index: u64,
        end_index: u64,
    ) -> ManagedVec<Compensation<Self::Api>> {
        let compensations = self
            .compensations()
            .into_iter()
            .skip(start_index as usize)
            .take((end_index - start_index + 1) as usize)
            .map(|compensation_id| self.get_compensation(compensation_id))
            .collect();

        compensations
    }

    #[view(getAddressRefundForCompensation)]
    fn get_address_compensation(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
    ) -> Option<(Compensation<Self::Api>, Option<Refund<Self::Api>>)> {
        let compensation_id = self.compensations_ids().get_id((token_identifier, nonce));
        if compensation_id == 0 {
            None
        } else {
            let compensation = Compensation {
                compensation_id,
                token_identifier: self.compensation_token_identifer(compensation_id).get(),
                nonce: self.compensation_nonce(compensation_id).get(),
                accumulated_amount: self.compensation_accumulated_amount(compensation_id).get(),
                proof_amount: self.compensation_proof_amount(compensation_id).get(),
                end_date: self.compensation_end_date(compensation_id).get(),
            };

            let refund = self.address_refund(&address, compensation_id).get();

            if self.address_refund(&address, compensation_id).is_empty() {
                Some((compensation, None))
            } else {
                Some((compensation, Some(refund)))
            }
        }
    }

    #[view(getBondsByTokenIdentifierNonce)]
    fn get_bonds_by_token_identifier_nonce(
        &self,
        input: MultiValueEncoded<MultiValue2<TokenIdentifier, u64>>,
    ) -> ManagedVec<Bond<Self::Api>> {
        let bonds = input
            .into_iter()
            .filter_map(|value| {
                let (token_identifier, nonce) = value.into_tuple();
                let bond_id = self.bonds_ids().get_id((token_identifier, nonce));
                if bond_id != 0 {
                    Some(self.get_bond(bond_id))
                } else {
                    None
                }
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

    #[view(getPagedBonds)]
    fn get_paged_bonds(&self, start_index: u64, end_index: u64) -> ManagedVec<Bond<Self::Api>> {
        let bonds = self
            .bonds()
            .into_iter()
            .skip(start_index as usize)
            .take((end_index - start_index + 1) as usize)
            .map(|bond_id| self.get_bond(bond_id))
            .collect();

        bonds
    }

    #[view(getBondsLen)]
    fn get_bonds_len(&self) -> usize {
        self.bonds().len() as usize
    }

    #[view(getLockPeriodsBonds)]
    fn get_lock_periods_bonds(&self) -> (ManagedVec<u64>, ManagedVec<BigUint>) {
        let lock_periods = self.lock_periods().into_iter().collect::<ManagedVec<u64>>();
        let bond_amounts = self
            .lock_periods()
            .into_iter()
            .map(|lock_period| self.lock_period_bond_amount(lock_period).get())
            .collect::<ManagedVec<BigUint>>();

        (lock_periods, bond_amounts)
    }
}
