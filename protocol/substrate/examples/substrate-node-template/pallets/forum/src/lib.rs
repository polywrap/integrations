#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

mod types;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::types::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The maximum length of the post content
		#[pallet::constant]
		type MaxContentLength: Get<u32>;

		/// The maximum allowed children comments
		#[pallet::constant]
		type MaxComments: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn post)]
	pub type AllPosts<T: Config> = StorageMap<_, Twox64Concat, u32, Post<T>>;

	/// The comment (post_id, comment_id, (content, author, parent_comment))
	#[pallet::storage]
	#[pallet::getter(fn comment)]
	pub type AllComments<T: Config> = StorageMap<_, Twox64Concat, u32, Comment<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kids)]
	/// The heirarchy of comments (item_id, Vec<item_id>)
	pub type Kids<T: Config> = StorageMap<_, Twox64Concat, u32, BoundedVec<u32, T::MaxComments>>;

	/// Keeps track of the item added into the system
	/// increments as more post or item is added
	#[pallet::storage]
	#[pallet::getter(fn item_counter)]
	pub(super) type ItemCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A post is submitted with post_id, and the author
		PostSubmitted(u32, T::AccountId, BoundedVec<u8, T::MaxContentLength>),
		/// A comment is submmited with comment_id and the author)
		CommentSubmitted(u32, T::AccountId, BoundedVec<u8, T::MaxContentLength>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		ContentTooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add a post content to the storage, emit and event
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn post_content(
			origin: OriginFor<T>,
			content: BoundedVec<u8, T::MaxContentLength>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;

			// use the total number of items as post_id
			let post_id = ItemCounter::<T>::get();

			let post = Post {
				post_id,
				content: content.clone(),
				timestamp: Self::timestamp(),
				author: who.clone(),
				block_number: Self::block_number(),
			};

			AllPosts::<T>::insert(post_id, post);
			// increment the item counter
			Self::increment_item_counter();
			// Emit a PostSubmitted event
			Self::deposit_event(Event::PostSubmitted(post_id, who, content));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn comment_on(
			origin: OriginFor<T>,
			parent_item: u32,
			content: BoundedVec<u8, T::MaxContentLength>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::add_comment_to(who.clone(), parent_item, content)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn timestamp() -> T::Moment {
			pallet_timestamp::Pallet::<T>::get()
		}

		pub fn block_number() -> T::BlockNumber {
			<frame_system::Pallet<T>>::block_number()
		}
		/// increment th ItemCounter storage value
		fn increment_item_counter() {
			ItemCounter::<T>::mutate(|i| {
				*i = i.saturating_add(1);
			});
		}

		pub fn add_comment_to(
			who: T::AccountId,
			parent_item: u32,
			content: BoundedVec<u8, T::MaxContentLength>,
		) -> DispatchResult {
			log::warn!("adding comment to..");
			let comment_id = ItemCounter::<T>::get();
			AllComments::<T>::insert(
				comment_id,
				Comment {
					comment_id,
					content: content.clone(),
					author: who.clone(),
					parent_item,
					timestamp: Self::timestamp(),
					block_number: Self::block_number(),
				},
			);
			Self::increment_item_counter();

			if Kids::<T>::contains_key(parent_item) {
				log::info!("adding comment: {} to parent: {}", comment_id, parent_item);
				Kids::<T>::mutate(parent_item, |i| {
					if let Some(i) = i {
						i.try_push(comment_id).unwrap();
					}
				});
			} else {
				log::info!("inserting as a new kid entry: {} -> {}", parent_item, comment_id);
				Kids::<T>::insert(parent_item, BoundedVec::try_from(vec![comment_id]).unwrap());
			}
			Self::deposit_event(Event::CommentSubmitted(comment_id, who, content));
			Ok(())
		}

		pub fn get_post(post_id: u32) -> Option<Post<T>> {
			log::info!("getting post_id: {}", post_id);
			AllPosts::<T>::get(post_id)
		}
	}
}
