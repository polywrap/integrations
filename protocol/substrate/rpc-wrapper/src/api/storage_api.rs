//!
//! Constant API
//!
//! Extension to the API for retrieving data from on-chain storage
//!

use crate::{api::Api, utils::FromHexStr, Error};
use codec::{Decode, Encode};
use scale_info::{form::PortableForm, Type};
use sp_core::storage::StorageKey;

impl Api {
    /// Retrieve a value from storage value named `storage_name` in pallet `module`.
    pub fn fetch_storage_value<V>(
        &self,
        module: &str,
        storage_name: &str,
    ) -> Result<Option<V>, Error>
    where
        V: Decode,
    {
        let storage_key =
            self.metadata.storage_value_key(module, storage_name)?;
        self.fetch_storage_by_key_hash(storage_key)
    }

    /// Retrieve a value from a storage map named `storage_name` with a key `key` in pallet `module`
    pub fn fetch_storage_map<K, V>(
        &self,
        module: &str,
        storage_name: &str,
        key: K,
    ) -> Result<Option<V>, Error>
    where
        K: Encode,
        V: Decode,
    {
        let storage_key =
            self.metadata.storage_map_key(module, storage_name, key)?;
        self.fetch_storage_by_key_hash(storage_key)
    }

    /// Retrieve a value from a storage double map named `storage_name` with keys `first` and
    /// `second` in pallet `module`.
    pub fn fetch_storage_double_map<K, Q, V>(
        &self,
        module: &str,
        storage_name: &str,
        first: K,
        second: Q,
    ) -> Result<Option<V>, Error>
    where
        K: Encode,
        Q: Encode,
        V: Decode,
    {
        let storage_key = self.metadata.storage_double_map_key(
            module,
            storage_name,
            first,
            second,
        )?;
        self.fetch_storage_by_key_hash(storage_key)
    }

    pub fn fetch_storage_by_key_hash<V>(
        &self,
        storage_key: StorageKey,
    ) -> Result<Option<V>, Error>
    where
        V: Decode,
    {
        match self.fetch_opaque_storage_by_key_hash(storage_key)? {
            Some(storage) => Ok(Some(Decode::decode(&mut storage.as_slice())?)),
            None => Ok(None),
        }
    }

    /// Retrieve a value in bytes from storage value named `storage_name` in pallet `module`.
    pub fn fetch_opaque_storage_value(
        &self,
        module: &str,
        storage_name: &str,
    ) -> Result<Option<Vec<u8>>, Error> {
        let storage_key =
            self.metadata.storage_value_key(module, storage_name)?;
        self.fetch_opaque_storage_by_key_hash(storage_key)
    }

    /// Retrieve a value in bytes from a storage map named `storage_name` with a key `key` in pallet `module`
    pub fn fetch_opaque_storage_map<K>(
        &self,
        module: &str,
        storage_name: &str,
        key: K,
    ) -> Result<Option<Vec<u8>>, Error>
    where
        K: Encode,
    {
        let storage_key =
            self.metadata.storage_map_key(module, storage_name, key);
        self.fetch_opaque_storage_by_key_hash(storage_key.unwrap())
    }

    fn fetch_opaque_storage_by_key_hash(
        &self,
        storage_key: StorageKey,
    ) -> Result<Option<Vec<u8>>, Error> {
        let value = self
            .base_api
            .json_request_value("state_getStorage", [storage_key])?;

        match value {
            Some(value) => {
                let value_str = value.as_str().expect("must be a str");
                let data = Vec::from_hex(value_str)?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    /// Retrieve a paged list of values in bytes from a storage map named `storage_name`
    /// `module` - the pallet where the storage is created from.
    /// `storage_name` - the given name of the storage in the pallet.
    /// `count` - the number of values to be returned
    /// `start_key` - the key used as an offset marker to start getting the list of records.
    pub fn fetch_opaque_storage_map_paged<K>(
        &self,
        module: &str,
        storage_name: &str,
        count: u32,
        start_key: Option<K>,
    ) -> Result<Option<Vec<Vec<u8>>>, Error>
    where
        K: Encode,
    {
        let storage_keys: Option<Vec<StorageKey>> = self
            .fetch_opaque_storage_keys_paged(
                module,
                storage_name,
                count,
                start_key,
            )?;

        if let Some(storage_keys) = storage_keys {
            let mut storage_values = Vec::with_capacity(storage_keys.len());
            for storage_key in storage_keys.into_iter() {
                if let Some(bytes) =
                    self.fetch_opaque_storage_by_key_hash(storage_key)?
                {
                    storage_values.push(bytes);
                }
            }
            Ok(Some(storage_values))
        } else {
            Ok(None)
        }
    }

    /// return the data type of this storage named `storage_name` from pallet `module`.
    pub fn storage_map_type(
        &self,
        module: &str,
        storage_name: &str,
    ) -> Result<Option<(&Type<PortableForm>, &Type<PortableForm>)>, Error> {
        Ok(self.metadata.storage_map_type(module, storage_name)?)
    }

    /// Retrieve a paged list of the keys from a storage map named `storage_name` in pallet `module`
    /// `module` - the pallet where the storage is created from.
    /// `storage_name` - the given name of the storage in the pallet.
    /// `count` - the number of values to be returned
    /// `start_key` - the key used as an offset marker to start getting the list of records.
    pub fn fetch_opaque_storage_keys_paged<K>(
        &self,
        module: &str,
        storage_name: &str,
        count: u32,
        start_key: Option<K>,
    ) -> Result<Option<Vec<StorageKey>>, Error>
    where
        K: Encode,
    {
        let storage_key =
            self.metadata.storage_map_key_prefix(module, storage_name)?;
        let start_storage_key = if let Some(start_key) = start_key {
            Some(self.metadata.storage_map_key(
                module,
                storage_name,
                start_key,
            )?)
        } else {
            None
        };
        let value = self.base_api.json_request_value(
            "state_getKeysPaged",
            (storage_key, count, start_storage_key),
        )?;

        match value {
            Some(value) => {
                let value_array =
                    value.as_array().expect("must be an array of str");
                let data: Vec<StorageKey> = value_array
                    .into_iter()
                    .map(|v| {
                        let value_str =
                            v.as_str().expect("each item must be a str");
                        let bytes = Vec::from_hex(value_str)
                            .expect("must convert hex value to bytes");
                        StorageKey(bytes)
                    })
                    .collect();
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}
