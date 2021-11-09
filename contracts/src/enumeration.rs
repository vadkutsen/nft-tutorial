use crate::*;

#[near_bindgen]
impl Contract {

    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) {
        /*
            FILL THIS IN
        */
    }

    pub fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) {
        /*
            FILL THIS IN
        */
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let tokens_owner = self.tokens_per_owner.get(&account_id);
        let tokens = if let Some(tokens_owner) = tokens_owner {
            tokens_owner
        } else {
            return vec![];
        };
        let keys = tokens.as_vector();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}
