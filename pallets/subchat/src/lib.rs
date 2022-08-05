#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod palletsubchat {

	use super::*;
	use frame_support::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;
	pub type MessageId = u64;
	pub type ChannelId = u64;

	//Content Enum Type
	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum Content {
		None,
		Raw(Vec<u8>),
		Encrypted(Vec<u8>),
		IPFS(Vec<u8>),
	}
	//Message Struct
	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct Message<AccountId, Moment> {
		pub id: MessageId,
		pub sender: AccountId,
		pub recipient: AccountId,
		pub message_content: Content,
		pub nonce: Vec<u8>,
		pub created_at: Moment,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum Owner {
		Sender,
		Recipient,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct NewMessage<AccountId> {
		sender: AccountId,
		sender_message_id: MessageId,
		recipient: AccountId,
		recipient_message_id: MessageId,
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn message_by_message_id)]
	pub type MessageByMessageId<T: Config> =
		StorageMap<_, Blake2_128Concat, MessageId, Message<T::AccountId, T::Moment>>;

	#[pallet::storage]
	#[pallet::getter(fn message_ids_by_account_ids)]
	pub type MessageIdsByAccountIds<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		Vec<MessageId>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn conversations_by_account_id)]
	pub type ConversationsByAccountId<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::AccountId>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MessageCreated(NewMessage<T::AccountId>),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn new_message(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			message: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let to = T::Lookup::lookup(dest)?;

			log::debug!("random_message from {:?} to {:?}", from, to);
			let new_id = <NewMessageId<T>>::get().unwrap_or(0);
			let now = <pallet_timestamp>::Pallet < T >> ::now();

			let a_id = new_id;
			let a_message = Message {
				id: a_id,
				sender: from.clone(),
				recipient: to.clone(),
				content: Content::Raw(message.clone()),
				created_at: now,
				owner: Owner::Sender,
			};
		}
	}
}
