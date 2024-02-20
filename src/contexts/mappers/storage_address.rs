use multiversx_sc::{
    api::StorageMapperApi, codec::TopDecode, storage::StorageKey, storage_get, storage_get_len,
    types::ManagedRef,
};

pub trait StorageAddress<SA>
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T;
    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize;
}

pub struct CurrentStorage;

impl<SA> StorageAddress<SA> for CurrentStorage
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T {
        storage_get(key)
    }

    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize {
        storage_get_len(key)
    }
}
