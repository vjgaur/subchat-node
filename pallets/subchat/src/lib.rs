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
	#[pallet::getter(fn next_message_id)]
	pub type NextMessageId<T> = StorageValue<_,u64>;
	

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

			<MessageByMessageId<T>>::insert(new_id,a_message);
			let b_id = new_id;
			let b_message = Message{

				id:b_id,
				sender:from.clone(),
				recipient:to.clone(),
				content:Content::Raw(message.clone()),
				created_at:now,
				owner:Owner::Recipient,
			};

			<MessageByMessageId<T>>::insert(b_id,b_message);
			<NextMessageId<T>>::put(new_id+2);
			
			let mut messages_a = <MessageIdsByAccountIds<T>>::get(from.clone(),to.clone()).unwrap_or(Vec::new());
			messages_a.push(a_id);
			<MessageIdsByAccountIds<T>>::insert(from.clone(), to.clone(), messages_a);

			let mut messages_b = <MessageIdsByAccountIds<T>>::get(from.clone(),to.clone()).unwrap_or(Vec::new());
			messages_b.push(b_id);
			<MessageIdsByAccountIds<T>>::insert(from.clone(), to.clone(), messages_b);
	
			let mut recent_a = <ConversationsByAccountId<T>>::get(from.clone()).unwrap_or(Vec::new());

			if !recent_a.contains(&to){
				recent_a.push(to.clone());
				<ConversationsByAccountId<T>>::insert(from.clone(),recent_a);
			}

			let mut recent_b = <conversations_by_account_id<T>>::get(from.clone()).unwrap_or(Vec::new());
			if !recent_b.contains(&to){
				recent_b.push(to.clone());
				<ConversationsByAccountId<T>>::insert(from.clone(),recent_b);
			}

			Self::deposit_event(Event::MessageCreated(NewMessageEvent {
				sender:from.clone(),
				sender_message_id:a_id,
				recipient:to.clone(),
				recipient_message_id:b_id,
			}));
			Ok(().into())
		}
	}
}
