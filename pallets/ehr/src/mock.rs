use crate as pallet_ehr;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup}, offchain::{DbExternalities, StorageKind},
};
use sp_runtime::{
	testing:: {TestXt},
	traits::{ Extrinsic as ExtrinsicT, IdentifyAccount, Verify},
};
use parking_lot::RwLock;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_core::{
	offchain::{testing:: {self, OffchainState, PoolState}
	, OffchainWorkerExt, TransactionPoolExt },
	sr25519::Signature,
};
use codec::alloc::sync::Arc;
type Extrinsic = TestXt<Call, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Index = u64;
use pallet_sudo;
use frame_support::{
	parameter_types, traits::{GenesisBuild, ConstU64, ConstU16},
};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Sudo:	pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},	
		Ehr: pallet_ehr::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaxMembers: u32 = 10;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl pallet_sudo::Config for Test {
	type Event = Event;
	type Call = Call;
}
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		index: Index,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (index,())))
	}
}

impl pallet_ehr::Config for Test {
	type Event = Event;
	type AuthorityId = pallet_ehr::crypto::DataProviderId; 

}

pub fn test_pub(number: u8) -> sp_core::sr25519::Public {
	sp_core::sr25519::Public::from_raw([number; 32])
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_sudo ::GenesisConfig::<Test> {
		key : Some(test_pub(1)),
	}
	.assimilate_storage(&mut t)
	.unwrap();
	// We use default for brevity, but you can configure as desired if needed.
	pallet_ehr::GenesisConfig::<Test> {
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	
	t.into()
}
pub struct ExternalityBuilder;
impl ExternalityBuilder {
	pub fn build() -> (
		TestExternalities,
		Arc<RwLock<PoolState>>,
		Arc<RwLock<OffchainState>>,
	) {
		const PHRASE: &str =
			"expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		let (mut offchain, offchain_state) = testing::TestOffchainExt::new();
		offchain.local_storage_set(StorageKind::PERSISTENT, "token_url".as_bytes(), "http://localhost:8001".as_bytes());
		offchain.local_storage_set(StorageKind::PERSISTENT, "client_id".as_bytes(), "http://localhost:8003".as_bytes());
		offchain.local_storage_set(StorageKind::PERSISTENT, "kid".as_bytes(), "09f1fa89-c4ea-4d81-aa21-2ad3c781435c".as_bytes());
		offchain.local_storage_set(StorageKind::PERSISTENT, "pem".as_bytes(), "-----BEGIN EC PRIVATE KEY-----
		MIGkAgEBBDBahrYNyrPLiWSGVQykhp4RAo0Z8swFNRSClT9UPdAOIpSIEKrEabYa
		+pb17wxynhOgBwYFK4EEACKhZANiAARfAku9SbA5AfyPWxLTIcSHgZC0uTxwJXX1
		3/JHVjoGdilgc/BzdE2OcsYkgtdCufdVaIG5Hdzlk8rKKzQ5d0vAy8CtvXW5R8rl
		xAZZq779qywgqMkzPbOxLzdVzuQAo3Y=
		-----END EC PRIVATE KEY-----
		".as_bytes());

		let (pool, pool_state) = testing::TestTransactionPoolExt::new();
		let keystore = KeyStore::new();
		keystore
			.sr25519_generate_new(crate::KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
			.unwrap();
		let mut t = new_test_ext();
		t.register_extension(OffchainWorkerExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(Arc::new(keystore)));
		t.execute_with(|| System::set_block_number(1));
		(t, pool_state, offchain_state)
	}
}
