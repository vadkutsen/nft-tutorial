use crate::*;
use near_sdk::json_types::{ValidAccountId};
use near_sdk::{ext_contract, Gas};

const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    fn nft_approve(&mut self, token_id: TokenId, account_id: ValidAccountId, msg: Option<String>);

	fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    fn nft_revoke(&mut self, token_id: TokenId, account_id: ValidAccountId);

    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: ValidAccountId, msg: Option<String>) {
        /*
            FILL THIS IN
        */
    }

	fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        /*
            FILL THIS IN
        */
    }

    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: ValidAccountId) {
        /*
            FILL THIS IN
        */
    }

    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        /*
            FILL THIS IN
        */
    }
}
