use crate::*;
use near_sdk::json_types::{ValidAccountId};

pub trait NonFungibleTokenCore {
    //calculates the payout for a token given the passed in balance. This is a view method
  	fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32);
    
    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
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

    //calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) {
		/*
            FILL THIS IN
        */
	}

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
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
