use crate::Config;
use frame_support::pallet_prelude::*;
use sp_std::prelude::*;

#[derive(Encode, Decode, TypeInfo, RuntimeDebug)]
#[scale_info(skip_type_params(T))]
pub struct Post<T: Config> {
	pub post_id: u32,
	pub content: BoundedVec<u8, T::MaxContentLength>,
	pub author: T::AccountId,
	pub timestamp: T::Moment,
	pub block_number: T::BlockNumber,
}

impl<T: Config> MaxEncodedLen for Post<T> {
	fn max_encoded_len() -> usize {
		<(u32, BoundedVec<u8, T::MaxContentLength>, T::AccountId, T::Moment, T::BlockNumber)>::max_encoded_len()
	}
}

impl<T: Config> PartialEq for Post<T> {
	fn eq(&self, other: &Self) -> bool {
		self.post_id == other.post_id
			&& self.content == other.content
			&& self.author == other.author
	}
}

#[derive(Encode, Decode, TypeInfo, RuntimeDebug)]
#[scale_info(skip_type_params(T))]
pub struct Comment<T: Config> {
	pub comment_id: u32,
	pub content: BoundedVec<u8, T::MaxContentLength>,
	pub author: T::AccountId,
	pub parent_item: u32,
	pub timestamp: T::Moment,
	pub block_number: T::BlockNumber,
}

impl<T: Config> MaxEncodedLen for Comment<T> {
	fn max_encoded_len() -> usize {
		<(u32, BoundedVec<u8, T::MaxContentLength>, T::AccountId, u32, T::Moment, T::BlockNumber)>::max_encoded_len(
		)
	}
}

impl<T: Config> PartialEq for Comment<T> {
	fn eq(&self, other: &Self) -> bool {
		self.comment_id == other.comment_id
			&& self.content == other.content
			&& self.author == other.author
			&& self.parent_item == other.parent_item
	}
}
