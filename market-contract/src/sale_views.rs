use crate::*;

#[near_bindgen]
impl Contract {

    /// views
    
    //returns the number of sales the marketplace has up (as a string)
    pub fn get_supply_sales(
        &self,
    ) -> U64 {
        //returns the sales object length wrapped as a U64
        U64(self.sales.len())
    }
    
    //returns the number of sales for a given account (result is a string)
    pub fn get_supply_by_owner_id(
        &self,
        account_id: AccountId,
    ) -> U64 {
        //get the set of sales for the given owner Id
        let by_owner_id = self.by_owner_id.get(&account_id);
        
        //if there as some set, we return the length but if there wasn't a set, we return 0
        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated sale objects for a given account. (result is a vector of sales)
    pub fn get_sales_by_owner_id(
        &self,
        account_id: AccountId,
        from_index: U64,
        limit: u64,
    ) -> Vec<Sale> {
        //temporary vector to hold sales
        let mut tmp = vec![];
        
        //get the set of sales for the given account ID
        let by_owner_id = self.by_owner_id.get(&account_id);
        //if there was some set, we set the sales object to that set. If there wasn't, sales is set to an empty vector
        let sales = if let Some(by_owner_id) = by_owner_id {
            by_owner_id
        } else {
            return vec![];
        };
        
        //get all the keys in the vector
        let keys = sales.as_vector();
        //the starting index is a u64 based on the passed in from_index
        let start = u64::from(from_index);
        //the ending index is the result of whatever is smaller: start + limit or the end of the sales vector
        let end = min(start + limit, sales.len());
        
        //iterate through the sales vector and push to the tmp vector so we can return it
        for i in start..end {
            tmp.push(self.sales.get(&keys.get(i).unwrap()).unwrap());
        }
        tmp
    }

    //get the number of sales for an nft contract. (returns a string)
    pub fn get_supply_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
    ) -> U64 {
        //get the set of tokens for associated with the given nft contract
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);
        
        //if there was some set, return it's length. Otherwise return 0
        if let Some(by_nft_contract_id) = by_nft_contract_id {
            U64(by_nft_contract_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated sale objects associated with a given nft contract. (result is a vector of sales)
    pub fn get_sales_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
        from_index: U64,
        limit: u64,
    ) -> Vec<Sale> {

        let mut tmp = vec![];
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);
        let sales = if let Some(by_nft_contract_id) = by_nft_contract_id {
            by_nft_contract_id
        } else {
            return vec![];
        };
        let keys = sales.as_vector();
        let start = u64::from(from_index);
        let end = min(start + limit, sales.len());
        for i in start..end {
            tmp.push(self.sales.get(&format!("{}{}{}", &nft_contract_id, DELIMETER, &keys.get(i).unwrap())).unwrap());
        }
        tmp
    }

    //get a sale information for a given unique sale ID (contract + DELIMITER + token ID)
    pub fn get_sale(&self, nft_contract_token: ContractAndTokenId) -> Option<Sale> {
        //try and get the sale object for the given unique sale ID. Will return an option since
        //we're not guaranteed that the unique sale ID passed in will be valid.
        self.sales.get(&nft_contract_token)
    }
    
}
