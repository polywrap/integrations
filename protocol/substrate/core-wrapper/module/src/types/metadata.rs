// Copyright 2019-2021 Parity Technologies (UK) Ltd. and Supercomputing Systems AG
// and Integritee AG.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Handle substrate chain metadata
//!
//! This file is mostly subxt.

use crate::types::storage::GetStorage;
use codec::{
    Encode,
    Error as CodecError,
};
use frame_metadata::{
    v14::StorageEntryType,
    PalletConstantMetadata,
    RuntimeMetadata,
    RuntimeMetadataLastVersion,
    RuntimeMetadataPrefixed,
    StorageEntryMetadata,
    META_RESERVED,
};
use scale_info::{
    form::PortableForm,
    Type,
    Variant,
};
use serde::Serialize;
use sp_core::storage::StorageKey;
use std::{
    collections::HashMap,
    convert::TryFrom,
};

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

/// Metadata error.
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    /// Module is not in metadata.
    #[error("Pallet {0} not found")]
    PalletNotFound(String),
    /// Pallet is not in metadata.
    #[error("Pallet index {0} not found")]
    PalletIndexNotFound(u8),
    /// Call is not in metadata.
    #[error("Call {0} not found")]
    CallNotFound(String),
    /// Event is not in metadata.
    #[error("Pallet {0}, Event {0} not found")]
    EventNotFound(u8, u8),
    /// Event is not in metadata.
    #[error("Pallet {0}, Error {0} not found")]
    ErrorNotFound(u8, u8),
    /// Storage is not in metadata.
    #[error("Storage {0} not found")]
    StorageNotFound(String),
    /// Storage type does not match requested type.
    #[error("Storage type error")]
    StorageTypeError,
    #[error("Map value type error")]
    MapValueTypeError,
    /// Default error.
    #[error("Failed to decode default: {0}")]
    DefaultError(CodecError),
    /// Failure to decode constant value.
    #[error("Failed to decode constant value: {0}")]
    ConstantValueError(CodecError),
    /// Constant is not in metadata.
    #[error("Constant {0} not found")]
    ConstantNotFound(String),
    #[error("Type {0} missing from type registry")]
    TypeNotFound(u32),
}

/// Runtime metadata.
#[derive(Clone, Debug)]
pub struct Metadata {
    pub metadata: RuntimeMetadataLastVersion,
    pub pallets: HashMap<String, PalletMetadata>,
    pub events: HashMap<(u8, u8), EventMetadata>,
    pub errors: HashMap<(u8, u8), ErrorMetadata>,
}

impl Metadata {
    /// Returns a reference to [`PalletMetadata`].
    pub fn pallet(&self, name: &str) -> Result<&PalletMetadata, MetadataError> {
        self.pallets
            .get(name)
            .ok_or_else(|| MetadataError::PalletNotFound(name.to_string()))
    }

    /// Returns the metadata for the event at the given pallet and event indices.
    pub fn get_event(
        &self,
        pallet_index: u8,
        event_index: u8,
    ) -> Result<&EventMetadata, MetadataError> {
        let event = self
            .events
            .get(&(pallet_index, event_index))
            .ok_or(MetadataError::EventNotFound(pallet_index, event_index))?;
        Ok(event)
    }

    /// Returns the metadata for all events of a given pallet
    pub fn get_events(&self, pallet_index: u8) -> Vec<EventMetadata> {
        self.events
            .clone()
            .into_iter()
            .filter(|(k, _v)| k.0 == pallet_index)
            .map(|(_k, v)| v)
            .collect()
    }

    /// Returns the metadata for the error at the given pallet and error indices.
    pub fn get_error(
        &self,
        pallet_index: u8,
        error_index: u8,
    ) -> Result<&ErrorMetadata, MetadataError> {
        let error = self
            .errors
            .get(&(pallet_index, error_index))
            .ok_or(MetadataError::ErrorNotFound(pallet_index, error_index))?;
        Ok(error)
    }

    /// Returns the metadata for all errors of a given pallet
    pub fn get_errors(&self, pallet_index: u8) -> Vec<ErrorMetadata> {
        self.errors
            .clone()
            .into_iter()
            .filter(|(k, _v)| k.0 == pallet_index)
            .map(|(_k, v)| v)
            .collect()
    }

    /// Resolve a type definition.
    pub fn get_resolve_type(&self, id: u32) -> Option<&Type<PortableForm>> {
        self.metadata.types.resolve(id)
    }

    /// Return the runtime metadata.
    pub fn get_runtime_metadata(&self) -> &RuntimeMetadataLastVersion {
        &self.metadata
    }

    pub fn storage_value_type(
        &self,
        pallet_name: &str,
        storage_name: &str,
    ) -> Result<Option<&Type<PortableForm>>, MetadataError> {
        let pallet = self.pallet(pallet_name)?;
        let storage_metadata = pallet.storage(storage_name)?;
        let ty_id = match storage_metadata.ty {
            StorageEntryType::Plain(plain) => Some(plain.id()),
            _ => None,
        };
        let portable_form =
            ty_id.map(|ty_id| self.get_resolve_type(ty_id)).flatten();
        Ok(portable_form)
    }

    pub fn pallet_call_index(
        &self,
        pallet_name: &str,
        call_name: &str,
    ) -> Result<[u8; 2], MetadataError> {
        let pallet = self.pallet(pallet_name)?;
        let call_index = pallet.calls.get(call_name).ok_or_else(|| {
            MetadataError::CallNotFound(call_name.to_string())
        })?;
        Ok([pallet.index, *call_index])
    }

    pub fn storage_map_type(
        &self,
        pallet_name: &str,
        storage_name: &str,
    ) -> Result<Option<(&Type<PortableForm>, &Type<PortableForm>)>, MetadataError>
    {
        let pallet = self.pallet(pallet_name)?;
        let storage_metadata = pallet.storage(storage_name)?;
        match storage_metadata.ty {
            StorageEntryType::Map { key, value, .. } => {
                let ty_key = self.get_resolve_type(key.id());
                let ty_value = self.get_resolve_type(value.id());
                match (ty_key, ty_value) {
                    (Some(ty_key), Some(ty_value)) => {
                        Ok(Some((ty_key, ty_value)))
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PalletMetadata {
    pub index: u8,
    pub name: String,
    pub calls: HashMap<String, u8>,
    pub storage: HashMap<String, StorageEntryMetadata<PortableForm>>,
    pub constants: HashMap<String, PalletConstantMetadata<PortableForm>>,
}

impl PalletMetadata {
    pub fn encode_call<C>(
        &self,
        call_name: &str,
        args: C,
    ) -> Result<Encoded, MetadataError>
    where
        C: Encode,
    {
        let fn_index = self.calls.get(call_name).ok_or_else(|| {
            MetadataError::CallNotFound(call_name.to_string())
        })?;
        let mut bytes = vec![self.index, *fn_index];
        bytes.extend(args.encode());
        Ok(Encoded(bytes))
    }

    pub fn storage(
        &self,
        key: &str,
    ) -> Result<&StorageEntryMetadata<PortableForm>, MetadataError> {
        self.storage
            .get(key)
            .ok_or_else(|| MetadataError::StorageNotFound(key.to_string()))
    }

    /// Get a constant's metadata by name
    pub fn constant(
        &self,
        key: &str,
    ) -> Result<&PalletConstantMetadata<PortableForm>, MetadataError> {
        self.constants
            .get(key)
            .ok_or_else(|| MetadataError::ConstantNotFound(key.to_string()))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct EventMetadata {
    pallet: String,
    event: String,
    variant: Variant<PortableForm>,
}

impl EventMetadata {
    /// Get the name of the pallet from which the event was emitted.
    pub fn pallet(&self) -> &str {
        &self.pallet
    }

    /// Get the name of the pallet event which was emitted.
    pub fn event(&self) -> &str {
        &self.event
    }

    /// Get the type def variant for the pallet event.
    pub fn variant(&self) -> &Variant<PortableForm> {
        &self.variant
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ErrorMetadata {
    pallet: String,
    error: String,
    variant: Variant<PortableForm>,
}

impl ErrorMetadata {
    /// Get the name of the pallet from which the error originates.
    pub fn pallet(&self) -> &str {
        &self.pallet
    }

    /// Get the name of the specific pallet error.
    pub fn error(&self) -> &str {
        &self.error
    }

    /// Get the description of the specific pallet error.
    pub fn description(&self) -> &[String] {
        self.variant.docs()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidMetadataError {
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Invalid version")]
    InvalidVersion,
    #[error("Type {0} missing from type registry")]
    MissingType(u32),
    #[error("Type {0} was not a variant/enum type")]
    TypeDefNotVariant(u32),
}

impl TryFrom<RuntimeMetadataPrefixed> for Metadata {
    type Error = InvalidMetadataError;

    fn try_from(
        metadata: RuntimeMetadataPrefixed,
    ) -> Result<Self, Self::Error> {
        if metadata.0 != META_RESERVED {
            return Err(InvalidMetadataError::InvalidPrefix);
        }
        let metadata = match metadata.1 {
            RuntimeMetadata::V14(meta) => meta,
            _ => return Err(InvalidMetadataError::InvalidVersion),
        };

        let get_type_def_variant = |type_id: u32| {
            let ty = metadata
                .types
                .resolve(type_id)
                .ok_or(InvalidMetadataError::MissingType(type_id))?;
            if let scale_info::TypeDef::Variant(var) = ty.type_def() {
                Ok(var)
            } else {
                Err(InvalidMetadataError::TypeDefNotVariant(type_id))
            }
        };

        let pallets = metadata
            .pallets
            .iter()
            .map(|pallet| {
                let calls = pallet.calls.as_ref().map_or(
                    Ok(HashMap::new()),
                    |call| {
                        let type_def_variant =
                            get_type_def_variant(call.ty.id())?;
                        let calls = type_def_variant
                            .variants()
                            .iter()
                            .map(|v| (v.name().clone(), v.index()))
                            .collect();
                        Ok(calls)
                    },
                )?;

                let storage =
                    pallet.storage.as_ref().map_or(HashMap::new(), |storage| {
                        storage
                            .entries
                            .iter()
                            .map(|entry| (entry.name.clone(), entry.clone()))
                            .collect()
                    });

                let constants = pallet
                    .constants
                    .iter()
                    .map(|constant| (constant.name.clone(), constant.clone()))
                    .collect();

                let pallet_metadata = PalletMetadata {
                    index: pallet.index,
                    name: pallet.name.to_string(),
                    calls,
                    storage,
                    constants,
                };

                Ok((pallet.name.to_string(), pallet_metadata))
            })
            .collect::<Result<_, _>>()?;

        let pallet_events = metadata
            .pallets
            .iter()
            .filter_map(|pallet| {
                pallet.event.as_ref().map(|event| {
                    let type_def_variant = get_type_def_variant(event.ty.id())?;
                    Ok((pallet, type_def_variant))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let events = pallet_events
            .iter()
            .flat_map(|(pallet, type_def_variant)| {
                type_def_variant.variants().iter().map(move |var| {
                    let key = (pallet.index, var.index());
                    let value = EventMetadata {
                        pallet: pallet.name.clone(),
                        event: var.name().clone(),
                        variant: var.clone(),
                    };
                    (key, value)
                })
            })
            .collect();

        let pallet_errors = metadata
            .pallets
            .iter()
            .filter_map(|pallet| {
                pallet.error.as_ref().map(|error| {
                    let type_def_variant = get_type_def_variant(error.ty.id())?;
                    Ok((pallet, type_def_variant))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let errors = pallet_errors
            .iter()
            .flat_map(|(pallet, type_def_variant)| {
                type_def_variant.variants().iter().map(move |var| {
                    let key = (pallet.index, var.index());
                    let value = ErrorMetadata {
                        pallet: pallet.name.clone(),
                        error: var.name().clone(),
                        variant: var.clone(),
                    };
                    (key, value)
                })
            })
            .collect();

        Ok(Self {
            metadata,
            pallets,
            events,
            errors,
        })
    }
}

/// Get the storage keys corresponding to a storage
///
/// This is **not** part of subxt.
impl Metadata {
    pub fn storage_value_key(
        &self,
        storage_prefix: &str,
        storage_key_name: &str,
    ) -> Result<StorageKey, MetadataError> {
        Ok(self
            .pallet(storage_prefix)?
            .storage(storage_key_name)?
            .get_value(storage_prefix)?
            .key())
    }

    pub fn storage_map_key<K: Encode>(
        &self,
        storage_prefix: &str,
        storage_key_name: &str,
        map_key: K,
    ) -> Result<StorageKey, MetadataError> {
        Ok(self
            .pallet(storage_prefix)?
            .storage(storage_key_name)?
            .get_map::<K>(storage_prefix)?
            .key(map_key))
    }

    pub fn storage_map_key_prefix(
        &self,
        storage_prefix: &str,
        storage_key_name: &str,
    ) -> Result<StorageKey, MetadataError> {
        self.pallet(storage_prefix)?
            .storage(storage_key_name)?
            .get_map_prefix(storage_prefix)
    }

    pub fn storage_double_map_key<K: Encode, Q: Encode>(
        &self,
        storage_prefix: &str,
        storage_key_name: &str,
        first: K,
        second: Q,
    ) -> Result<StorageKey, MetadataError> {
        Ok(self
            .pallet(storage_prefix)?
            .storage(storage_key_name)?
            .get_double_map::<K, Q>(storage_prefix)?
            .key(first, second))
    }
}
