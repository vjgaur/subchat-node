#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_std::collections::vec_deque::VecDeque;

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
		pub content: Content,
		pub nonce: Vec<u8>,
		pub created_at: Moment,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type MaxMessageLength:Get<u32>;
		#[pallet::constant]
		type MaxNonceLength:Get<u32>;
		#[pallet::constant]
		type MaxRecentConversations: Get<u32>;
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
	#[pallet::getter(fn next_channel_id)]
	pub type NextChannelId<T> = StorageValue<_,u64>;

	#[pallet::storage]
	#[pallet::getter(fn message_by_message_id)]
	pub type MessageByMessageId<T: Config> =
		StorageMap<_, Blake2_128Concat, MessageId, Message<T::AccountId, T::Moment>>;

	#[pallet::storage]
	#[pallet::getter(fn message_ids_by_channel_ids)]
	pub type MessageIdsByChannelId<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		ChannelId,
		Vec<MessageId>,
	>;
	#[pallet::storage]
	#[pallet::getter(fn account_ids_by_account_id)]
	pub type AccountIdsByAccountId<T: Config> = StorageMap<
		_, Blake2_128Concat, 
		T::AccountId, 
		VecDeque<T::AccountId>
	>;

	#[pallet::storage]
	#[pallet::getter(fn conversations_by_account_id)]
	pub type ConversationsByAccountId<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::AccountId>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MessageCreated(ChannelId,MessageId),
		NewChannelCreated(T::AccountId, T::AccountId, ChannelId),
		CommonKeyUpdated(ChannelId),
	}

	#[pallet::error]
	pub enum Error<T> {
		CommonKeyRequired,
		MaxMessageLengthExceeded,
		MaxNounceLengthExceeded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn new_message(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			message: Vec<u8>,
			nonce:Vec<u8>,
			encrypted_key_of_from:Option<Vec<u8>>,
			encrypted_key_of_to:Option<Vec<u8>>
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let to = T::Lookup::lookup(dest)?;

			log::debug!("new_message from {:?} to {:?}", from, to);

			let max_message_length = T::MaxMessageLength::get() as usize;
			ensure!(message.len()<= max_message_length, Error::<T>::MaxMessageLengthExceeded);

			let max_nounce_length = T::MaxNonceLength::get() as usize;
			ensure!(message.len() <= max_nounce_length,Error::<T>::MaxNounceLengthExceeded)

			let channel_id = match <ChannelIdByAccountIds<T>>::get(from.clone(),to.clone()){
				Some(id)=> {
					if encrypted_key_of_from.is_some() && encrypted_key_of_to.is_some(){
						let next_channel_id = <NextChannelId>::get().unwrap_or(0);
					}
				}
			

				<CommonKeyByAccounIdChannelId<T>>::insert(from.clone(),next_channel_id,encrypted_key_of_from.unwrap());
				<CommonKeyByAccounIdChannelId<T>>::insert(from.clone(),next_channel_id,encrypted_key_of_to.unwrap());
				Self::deposit_event(Event::CommonUpdated(id));
			}

			let next_channel_id = <NextChannelId<T>::get().unwrap_or(0)>
			

			<ChannelIdByAccountIds<T>>::insert(from.clone(), to.clone(), next_channel_id);
			<ChannelIdByAccountIds<T>>::insert(to.clone(), from.clone(), next_channel_id);

			<CommonKeyByAccounIdChannelId<T>>::insert(from.clone(),next_channel_id,encrypted_key_of_from.unwrap());
			<CommonKeyByAccounIdChannelId<T>>::insert(from.clone(),next_channel_id,encrypted_key_of_to.unwrap());
		
			<NextChannelId<T>>::put(next_channel_id + 1);
			Self::deposit_event(Event::NewChannelCreated(from.clone(), to.clone(), next_channel_id));

			next_channel_id
		}
	};

	let next_message_id = <NextMessageId<T>>::get().unwrap_or(0);
	let now = <pallet_timestamp::Pallet<T>>::now();
	let new_message = Message {
				id:next_message_id,
				sender:from.clone(),
				sender_message_id:a_id,
				recipient:to.clone(),
				recipient_message_id:b_id,
			}));
			content:Content::Encrypted(message.clone()),
			nounce,
			created_at::now,	
		};
		<MessageByMessageId<T>>::insert(next_message_id,new_message);

		let mut message_ids = <MessageIdsByChannelId<T>>::get(channel_id).unwrap_or(Vec::new());
		message_ids.push(next_message_id);
		<MessageIdsByChannelId<T>>::insert(channel_id,message_ids);
		
		let max_recent_conversations = T::MaxRecentConversations::get() as usize;
		Self::update_recent_conversations(from.clone(), to.clone(), max_recent_conversations);
		Self::update_recent_conversations(to.clone(), from.clone(), max_recent_conversations);
		
		<NextMessageId<T>>::put(next_message_id+1);
		Self::deposit_event(Event::MessageCreated(channel_id,next_message_id));
		Ok(().into())
		
	}
}
}

impl<T: Config> Pallet<T> {
	fn update_recent_conversations(who: T::AccountId, partner: T::AccountId, max_recent_conversations: usize) {
		<AccountIdsByAccountId<T>>::mutate(who, |maybe_account_ids| {
			let mut recent_account_ids: VecDeque<T::AccountId> = match maybe_account_ids {
				Some(channel_ids) => {
					let recent_account_ids: VecDeque<T::AccountId> = channel_ids
						.iter()
						.filter(|id| **id != partner)
						.cloned()
						.collect();

					recent_account_ids
				},
				None => VecDeque::new()
			};

			recent_account_ids.push_front(partner);
			while recent_account_ids.len() > max_recent_conversations {
				recent_account_ids.pop_back();
			}

			*maybe_account_ids = Some(recent_account_ids);
		});
	}
}