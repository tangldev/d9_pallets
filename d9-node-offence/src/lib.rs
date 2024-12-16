#![cfg_attr(not(feature = "std"), no_std)]

use pallet_d9_node_voting::ValidatorVoteStats;
use pallet_im_online::ValidatorId;
use pallet_im_online::{IdentificationTuple, UnresponsivenessOffence};
use sp_staking::offence::{Offence, OffenceError, ReportOffence};
use sp_std::prelude::*;
mod types;
pub use pallet::*;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{inherent::Vec, pallet_prelude::*, traits::ValidatorSet};
    use frame_system::{ensure_signed, pallet_prelude::*};
    use pallet_d9_node_voting::Pallet as NodeVoting;
    use sp_staking::offence::ReportOffence;
    const STORAGE_VERSION: frame_support::traits::StorageVersion =
        frame_support::traits::StorageVersion::new(1);
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_im_online::Config + pallet_d9_node_voting::Config
    {
        // Add pallet_im_online::Config
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type NodeId: Member
            + Parameter
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TryFrom<Self::AccountId>;
        // type IdentificationTuple: Parameter;  // No longer needed here
    }

    #[pallet::storage]
    #[pallet::getter(fn node_votes)]
    pub type NodeAccumulativeVotes<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u64, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomeEvent,
    }

    #[pallet::error]
    pub enum Error<T> {
        SomeErrors,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().reads_writes(2, 2))]
        pub fn submit_candidacy(origin: OriginFor<T>) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            Ok(())
        }
    }

    impl<T: Config>
        ReportOffence<
            T::AccountId,
            IdentificationTuple<T>,
            UnresponsivenessOffence<IdentificationTuple<T>>,
        > for Pallet<T>
    {
        fn report_offence(
            reporters: Vec<T::AccountId>,
            offence: UnresponsivenessOffence<IdentificationTuple<T>>,
        ) -> Result<(), OffenceError> {
            //note check to see if this is the right way to identify offenders
            let offenders = offence.offenders();
            for id_tuple in offenders.iter() {
                let (validator_id, _) = id_tuple;
                let encoded_validator_id = validator_id.encode();
                let account_id = T::AccountId::decode(&mut &encoded_validator_id[..]).unwrap();
                //note verify that validator_id is the same as account_id
            }

            Ok(())
        }
        fn is_known_offence(
            offenders: &[IdentificationTuple<T>],
            time_slot: &<UnresponsivenessOffence<IdentificationTuple<T>> as Offence<
                IdentificationTuple<T>,
            >>::TimeSlot, // Specify Offender type
        ) -> bool {
            // Your check logic
            false
        }
    }
}
