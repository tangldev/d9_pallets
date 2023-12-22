use frame_support::pallet_prelude::*;
use frame_support::RuntimeDebugNoBound;
use codec::MaxEncodedLen;
use crate::pallet::Config;
use crate::BalanceOf;
#[derive(
    PartialEqNoBound,
    EqNoBound,
    CloneNoBound,
    Encode,
    Decode,
    RuntimeDebugNoBound,
    TypeInfo,
    MaxEncodedLen
)]
pub struct VotingInterest {
    #[codec(compact)]
    pub total: u64,
    #[codec(compact)]
    pub delegated: u64,
}

impl Default for VotingInterest {
    fn default() -> Self {
        VotingInterest {
            total: 0, // Default value for total
            delegated: 0, // Default value for delegated
        }
    }
}

impl VotingInterest {
    pub fn new(total: u64) -> Self {
        VotingInterest {
            total,
            delegated: 0,
        }
    }
}

#[derive(
    PartialEqNoBound,
    EqNoBound,
    CloneNoBound,
    Encode,
    Decode,
    RuntimeDebugNoBound,
    TypeInfo,
    MaxEncodedLen
)]

/// defines how a user will delegate their votes among a particular candidate
#[scale_info(skip_type_params(T))]
pub struct ValidatorDelegations<T: Config> {
    pub candidate: T::AccountId,
    #[codec(compact)]
    pub votes: u64,
}
#[derive(
    PartialEqNoBound,
    EqNoBound,
    CloneNoBound,
    Encode,
    Decode,
    RuntimeDebugNoBound,
    TypeInfo,
    MaxEncodedLen
)]
#[scale_info(skip_type_params(T))]
pub struct Candidate<T: Config> {
    account_id: T::AccountId,
    #[codec(compact)]
    total_votes: BalanceOf<T>,
}
