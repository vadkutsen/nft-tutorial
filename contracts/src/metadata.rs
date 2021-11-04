use crate::*;
pub type TokenId = String;
pub type Payout = HashMap<AccountId, U128>;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTMetadata {
    /*
        FILL THIS IN
    */
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    /*
        FILL THIS IN
    */
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    /*
        FILL THIS IN
    */
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    /*
        FILL THIS IN
    */
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self);
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) {
        /*
            FILL THIS IN
        */
    }
}
