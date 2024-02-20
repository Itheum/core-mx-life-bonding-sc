multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub struct BondCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    sc_ref: &'a C,
    pub bond_id: u64,
    pub address: ManagedAddress<C::Api>,
    pub token_identifier: TokenIdentifier<C::Api>,
    pub nonce: u64,
    pub lock_period: u16,
    pub bond_timestamp: u64,
    pub unbound_timestamp: u64,
    pub bond_amount: BigUint<C::Api>,
}

impl<'a, C> BondCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    pub fn new(sc_ref: &'a C, bond_id: u64) -> Self {
        BondCache {
            sc_ref,
            bond_id,
            address: sc_ref.bond_address(bond_id).get(),
            token_identifier: sc_ref.bond_token_identifier(bond_id).get(),
            nonce: sc_ref.bond_nonce(bond_id).get(),
            lock_period: sc_ref.bond_lock_period(bond_id).get(),
            bond_timestamp: sc_ref.bond_timestamp(bond_id).get(),
            unbound_timestamp: sc_ref.unbound_timestamp(bond_id).get(),
            bond_amount: sc_ref.bond_amount(bond_id).get(),
        }
    }
}

impl<'a, C> Drop for BondCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    fn drop(&mut self) {
        self.sc_ref
            .bond_lock_period(self.bond_id)
            .set(self.lock_period);
        self.sc_ref
            .bond_timestamp(self.bond_id)
            .set(self.bond_timestamp);
        self.sc_ref
            .unbound_timestamp(self.bond_id)
            .set(self.unbound_timestamp);
        self.sc_ref.bond_amount(self.bond_id).set(self.bond_amount);
    }
}
