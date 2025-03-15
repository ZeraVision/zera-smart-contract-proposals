pub mod zera_treasury_v2 {
    use native_functions::zera::wasmedge_bindgen;
    use native_functions::zera::smart_contracts;
    use native_functions::zera::types::U256;
    use native_functions::zera::types;

    const TREASURY_WALLET: &str = "4Yg2ZeYrzMjVBXvU2YWtuZ7CzWR9atnQCD35TQj1kKcH";
    const ZRA_CONTRACT: &str = "$ZRA+0000";
    const BURN_WALLET: &str = ":fire:";

    #[wasmedge_bindgen]
    pub fn init() {
    }

    #[wasmedge_bindgen]
    pub fn withdraw(amount: String)
    {
        unsafe{
            let sc_wallet_ = smart_contracts::smart_contract_wallet();
            let sc_wallet = sc_wallet_.clone();

            if sc_wallet != TREASURY_WALLET.to_string(){
                smart_contracts::emit("Failed: Unauthorized sender".to_string());
                return;
            }

            if !types::is_valid_u256(amount.clone()){
                smart_contracts::emit("Failed: Invalid amount".to_string());
                return;
            }

            let wallet_address = smart_contracts::wallet_address();
            let wallet_balance = smart_contracts::wallet_balance(ZRA_CONTRACT.to_string(), wallet_address.clone());

            let amount_u256 = types::string_to_u256(amount.clone());

            if wallet_balance < amount_u256{
                smart_contracts::emit("Failed: Insufficient balance".to_string());
                return;
            }   

            let tokens = smart_contracts::wallet_tokens(TREASURY_WALLET.to_string());

            if tokens.is_empty(){
                smart_contracts::emit("Failed: No tokens in treasury wallet".to_string());
                return;
            }

            let (auth, equiv) = smart_contracts::get_ace_data(ZRA_CONTRACT.to_string());

            let token_len = U256::from(tokens.len());

            let fee_value =  token_len * U256::from(200_000_000_000_000_000u64);

            let zra_fees = (fee_value * U256::from(1_000_000_000u64)) / equiv;

            if zra_fees > amount_u256{
                smart_contracts::emit("Failed: Insufficient fees".to_string());
                return;
            }

            if !smart_contracts::hold(ZRA_CONTRACT.to_string(), amount_u256.to_string()){
                smart_contracts::emit("Failed: Withdraw".to_string());
                return;
            }
            
            let burn_amount = amount_u256 - zra_fees;

            let cir_supply = smart_contracts::circulating_supply(ZRA_CONTRACT.to_string());

            if cir_supply == U256::zero(){
                smart_contracts::emit("Failed: Circulating supply is zero".to_string());
                return;
            }

            let percentage = (burn_amount *  U256::from(1_000_000_000_000_000_000u64)) / cir_supply;

            for token in tokens{
                if token == ZRA_CONTRACT.to_string(){
                    continue;
                }

                let token_balance = smart_contracts::wallet_balance(token.clone(), TREASURY_WALLET.to_string());
                let send_amount = (token_balance * percentage) / U256::from(1_000_000_000_000_000_000u64);;

                smart_contracts::send(token.clone(), send_amount.to_string(), wallet_address.clone());
            }

            if !smart_contracts::send(ZRA_CONTRACT.to_string(), burn_amount.to_string(), BURN_WALLET.to_string()){
                smart_contracts::emit("Failed: Burn".to_string());
                return;
            }

            smart_contracts::emit("Success: Tokens sent".to_string());
        }
    }

    #[wasmedge_bindgen]
    pub fn send_all(wallet_address: String){
        unsafe{
             let pub_key_ = smart_contracts::public_key();
             let pub_key = pub_key_.clone();

            if pub_key != "gov_$TREASURY+0000" && pub_key != "gov_$ZRA+0000"{
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            smart_contracts::send_all(wallet_address.clone());
            
            smart_contracts::emit("Success: All tokens sent".to_string());
        }
    }

}