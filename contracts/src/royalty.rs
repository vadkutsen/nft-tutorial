use crate::*;
use near_sdk::json_types::{ValidAccountId};

pub trait NonFungibleTokenCore {
  	fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout;

    fn nft_transfer_payout(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
		let token = self.tokens_by_id.get(&token_id).expect("No token");

        // compute payouts based on balance option
        // adds in contract_royalty and computes previous owner royalty from remainder
        let owner_id = token.owner_id;
        let mut total_perpetual = 0;
        let balance_u128 = u128::from(balance);
		let mut payout: Payout = HashMap::new();
		let royalty = token.royalty;

		assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

		for (k, v) in royalty.iter() {
			let key = k.clone();
			if key != owner_id {
				payout.insert(key, royalty_to_payout(*v, balance_u128));
				total_perpetual += *v;
			}
		}
		
		// payout to contract owner - may be previous token owner, they get remainder of balance
		if self.contract_royalty > 0 && self.owner_id != owner_id {
			payout.insert(self.owner_id.clone(), royalty_to_payout(self.contract_royalty, balance_u128));
			total_perpetual += self.contract_royalty;
		}
		assert!(total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP, "Royalties should not be more than caps");
		// payout to previous owner
		payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

		payout
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
    ) -> Payout {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let previous_token = self.internal_transfer(
            &sender_id,
            receiver_id.as_ref(),
            &token_id,
            Some(approval_id),
            Some(memo),
        );
        refund_approved_account_ids(
            previous_token.owner_id.clone(),
            &previous_token.approved_account_ids,
        );

        // compute payouts based on balance option
        // adds in contract_royalty and computes previous owner royalty from remainder
        let owner_id = previous_token.owner_id;
        let mut total_perpetual = 0;
        let balance_u128 = u128::from(balance);
		let mut payout: Payout = HashMap::new();
		let royalty = self.tokens_by_id.get(&token_id).expect("No token").royalty;

		assert!(royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

		for (k, v) in royalty.iter() {
			let key = k.clone();
			if key != owner_id {
				payout.insert(key, royalty_to_payout(*v, balance_u128));
				total_perpetual += *v;
			}
		}
		
		// payout to contract owner - may be previous token owner, they get remainder of balance
		if self.contract_royalty > 0 && self.owner_id != owner_id {
			payout.insert(self.owner_id.clone(), royalty_to_payout(self.contract_royalty, balance_u128));
			total_perpetual += self.contract_royalty;
		}
		assert!(total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP, "Royalties should not be more than caps");
		// payout to previous owner
		payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

		payout
    }
}
