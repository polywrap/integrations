//! fetch metadata constant values
use crate::{
    api::Api,
    Error,
};
use frame_metadata::PalletConstantMetadata;
use scale_info::{
    form::PortableForm,
    Type,
};

impl Api {
    pub fn constant_metadata(
        &self,
        module: &str,
        constant_name: &str,
    ) -> Result<&PalletConstantMetadata<PortableForm>, Error> {
        Ok(self.metadata().pallet(module)?.constant(constant_name)?)
    }

    pub fn fetch_constant_type(
        &self,
        module: &str,
        constant_name: &str,
    ) -> Result<std::option::Option<&Type<PortableForm>>, Error> {
        let metadata = self.metadata();
        let ty = metadata.pallet(module)?.constant(constant_name)?.ty;
        Ok(metadata.get_resolve_type(ty.id()))
    }

    pub fn fetch_constant_opaque_value(
        &self,
        module: &str,
        constant_name: &str,
    ) -> Result<Vec<u8>, Error> {
        Ok(self.constant_metadata(module, constant_name)?.value.clone())
    }
}
