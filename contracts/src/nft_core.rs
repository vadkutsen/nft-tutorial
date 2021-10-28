use crate::*;
use near_sdk::json_types::{ValidAccountId};
use near_sdk::{ext_contract, log, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
    );

    /// Returns `true` if the token was transferred from the sender's account.
    fn nft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> Promise;

    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}

#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool;
}

trait NonFungibleTokenResolver {
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
    ) {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let previous_token = self.internal_transfer(
            &sender_id,
            receiver_id.as_ref(),
            &token_id,
            Some(approval_id),
            memo,
        );
        refund_approved_account_ids(
            previous_token.owner_id.clone(),
            &previous_token.approved_account_ids,
        );
    }

    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> Promise {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let previous_token = self.internal_transfer(
            &sender_id,
            receiver_id.as_ref(),
            &token_id,
            Some(approval_id),
            memo,
        );
        // Initiating receiver's call and the callback
        ext_non_fungible_token_receiver::nft_on_transfer(
            sender_id,
            previous_token.owner_id.clone(),
            token_id.clone(),
            msg,
            receiver_id.as_ref(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
        )
        .then(ext_self::nft_resolve_transfer(
            previous_token.owner_id,
            receiver_id.into(),
            token_id,
            previous_token.approved_account_ids,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
    }

    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            Some(JsonToken {
                token_id,
                owner_id: token.owner_id,
                metadata,
                royalty: token.royalty,
                approved_account_ids: token.approved_account_ids,
            })
        } else {
            None
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: HashMap<AccountId, u64>,
    ) -> bool {
        // Whether receiver wants to return token back to the sender, based on `nft_on_transfer`
        // call result.
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                if !return_token {
                    // Token was successfully received.
                    refund_approved_account_ids(owner_id, &approved_account_ids);
                    return true;
                }
            }
        }

        let mut token = if let Some(token) = self.tokens_by_id.get(&token_id) {
            if token.owner_id != receiver_id {
                // The token is not owner by the receiver anymore. Can't return it.
                refund_approved_account_ids(owner_id, &approved_account_ids);
                return true;
            }
            token
        } else {
            // The token was burned and doesn't exist anymore.
            refund_approved_account_ids(owner_id, &approved_account_ids);
            return true;
        };

        log!("Return {} from @{} to @{}", token_id, receiver_id, owner_id);

        self.internal_remove_token_from_owner(&receiver_id, &token_id);
        self.internal_add_token_to_owner(&owner_id, &token_id);
        token.owner_id = owner_id;
        refund_approved_account_ids(receiver_id, &token.approved_account_ids);
        token.approved_account_ids = approved_account_ids;
        self.tokens_by_id.insert(&token_id, &token);

        false
    }
}
