use crate::*;
use near_sdk::json_types::{ValidAccountId};

pub trait NonFungibleTokenCore {
  	fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32);

    fn nft_transfer_payout(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) {
		/*
            FILL THIS IN
        */
	}

    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) {
        /*
            FILL THIS IN
        */
    }
}
