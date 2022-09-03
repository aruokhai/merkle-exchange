#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use serde::{ Serialize, Deserialize, };

use sp_core::crypto::KeyTypeId;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"dara");
use scale_info::prelude::string::String;

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct DataProviderId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for DataProviderId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for DataProviderId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: u64,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iss: String,         // Optional. Issuer
    jti: String,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct PostBody {
    scope: String,       
    grant_type: String,         
    client_assertion_type: String,        
	client_assertion: String,         
   
}


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec;
	use sp_std::borrow::ToOwned;
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction,
			Signer,
		},
};
	use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
	use querystring::stringify;
	use sp_runtime::{
			offchain::{
				http,
				storage::{StorageValueRef},
				Duration },
	};
	use sp_runtime::traits::Zero;
	use crate::Claims;
	use crate::PostBody;
	use libaes::Cipher;
	use sp_io::offchain::*;
	
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn providers)]
	pub type Provider<T: Config> = StorageMap<_,Blake2_128Concat,T::AccountId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn patients)]
	pub type Patient<T: Config> = StorageMap<_,Blake2_128Concat,T::AccountId, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Token<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, Vec<u8>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub providers: Vec<T::AccountId>,
		pub patients: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {providers: Vec::new(), patients: Vec::new() }
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProviderAdded,
		ProviderRemoved,
		PatientAdded,
		PatientRemoved,
		NoProvider,
		NoPatient,
		TokenUpdated,
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyProivder,
		NotProvider,
		AlreadyPatient,
		NotPatient,
		NoSmartURL,
		NoAccountId,
		ClientIdError,
		KidError,
		PemError,
		NoNetworkUrl,
		NoPem,
		NoKid,
		NoResp,

	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			if (block_number % 10u32.into()) == Zero::zero() {
				Self::aggregate_tokens(block_number);
			}

		}


	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_provider(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<Provider<T>>::set(who, Vec::new());
			Self::deposit_event(Event::ProviderAdded);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_provider(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<Provider<T>>::remove(who);
			Self::deposit_event(Event::ProviderRemoved);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_patient(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<Patient<T>>::set(who,  Vec::new());
			Self::deposit_event(Event::PatientAdded);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_patient(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<Patient<T>>::remove(who);
			Self::deposit_event(Event::PatientRemoved);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_patients_token(origin: OriginFor<T>, patient_turple: Vec<(T::AccountId, Vec<u8>)>) -> DispatchResult {
			let originator = ensure_signed(origin)?;
			let is_provider = <Provider<T>>::contains_key(originator.clone());
			for (patient, token) in patient_turple {
				let is_patient = <Patient<T>>::contains_key(patient.clone());
				if is_provider && is_patient {
					<Token<T>>::insert(originator.clone(),patient,token);
				}
			}
			Self::deposit_event(Event::TokenUpdated);
			Ok(())
		}
	
	}

	impl <T: Config>  Pallet<T> {
		pub fn aggregate_tokens(block_number: T::BlockNumber) -> Result<(), Error<T>> {		
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			signer.send_signed_transaction(|account| {
				let patients = <Provider<T>>::get(account.clone().id);
				let mut patient_token_turple: Vec<(T::AccountId, Vec<u8>)> = Vec::new();
				for patient in patients {
					let request = Self::get_access_token(patient.clone());
					if let Ok(token) = request {
						patient_token_turple.push((patient,token));
					}
				}
				Call::update_patients_token { patient_turple : patient_token_turple }
			});
			Ok(())
		}

		pub fn get_access_token(patient: T::AccountId) -> Result<Vec<u8>, Error<T>> {
			let token_url = StorageValueRef::persistent(b"token_url").get::<Vec<u8>>()
				.map_err(|_| Error::<T>::NoSmartURL )?;
			let client_id = StorageValueRef::persistent(b"client_id").get::<Vec<u8>>()
				.map_err(|_| Error::<T>::ClientIdError )?;
			let kid = StorageValueRef::persistent(b"kid").get::<Vec<u8>>()
				.map_err(|_| Error::<T>::KidError )?;
			let pem = StorageValueRef::persistent(b"pem").get::<Vec<u8>>()
				.map_err(|_| Error::<T>::PemError )?;
			if let Some(url) = token_url {
				if let Some(id) = client_id {
					if let Some(k) = kid {
						if let Some(private_key) = pem {
								return Self::create_access_token(url, id, k, private_key);

						}else {
							Err(Error::<T>::NoPem)
						}
 					}else {
						Err(Error::<T>::NoKid)
					}
				}else {
					Err(Error::<T>::NoNetworkUrl)
				}
			}else {
				Err(Error::<T>::NoSmartURL)
			}
		}

		pub fn create_access_token(url: Vec<u8>, id: Vec<u8>, k: Vec<u8>, private_key: Vec<u8>) -> Result<Vec<u8>, Error<T>> {
			let duration = timestamp().unix_millis() + 60000u64;
			let random_seed = hex::encode(random_seed());
			let kid_string = sp_std::str::from_utf8(k.as_slice()).map_err(|_| {
				Error::<T>::KidError
			})?;
			let client_id_string = sp_std::str::from_utf8(id.as_slice()).map_err(|_| {
				Error::<T>::KidError
			})?;
			let token_url_string = sp_std::str::from_utf8(url.as_slice()).map_err(|_| {
				Error::<T>::KidError
			})?;
			let mut header = Header::new(Algorithm::ES384);
			header.typ = Some("JWT".to_owned());
			header.kid = Some(kid_string.to_owned());
			let paylod =  Claims {
				iss: client_id_string.to_owned(),
				aud: token_url_string.to_owned(),
				exp: duration,
				jti: random_seed,
				sub: client_id_string.to_owned(),
			};							
			let token = jsonwebtoken::encode(&header, &paylod, &EncodingKey::from_ec_pem(private_key.as_slice()).unwrap()).
				map_err(|_| {
					Error::<T>::KidError
				})?;
			let body = stringify(vec![
				("scope","system/*.*"),
				("grant_type","client_credentials"),
				("client_assertion_type","urn:ietf:params:oauth:client-assertion-type:jwt-bearer"),
				("client_assertion",&token),
			]);
			let request =
				http::Request::post(token_url_string, vec![body])
				.add_header("Content-Type","application/x-www-form-urlencoded")
				.add_header("accept","application/json");
			let deadline = timestamp().add(Duration::from_millis(5_000));
			let pending = request.deadline(deadline).send().map_err(|_|  Error::<T>::NoPem)?;
			let response = pending.try_wait(deadline).map_err(|_| Error::<T>::NoPem)?.ok();
			if let Some(resp_body) = response {
				let result= resp_body.body().collect::<Vec<u8>>();
				let iv = b"plaintext";
				let my_key = b"This is the key!";
				let cipher = Cipher::new_128(my_key);
				let encrypted = cipher.cbc_encrypt(iv, result.as_slice());
				Ok(encrypted)
			} else {
				Err(Error::<T>::NoResp)
			}
		}

	}
}

	
