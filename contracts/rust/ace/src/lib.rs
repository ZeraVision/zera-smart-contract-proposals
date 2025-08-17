pub mod authorize_tokens_v1 {
    use native_functions::zera::smart_contracts;
    use native_functions::zera::wasmedge_bindgen;

    const PROXY_WALLET: &str = "6yKBAKXCXo16BxW1r5XYq7PNDJUiiszePAt5r38fjGzq";

    #[wasmedge_bindgen]
    pub fn init() {
        unsafe {
        }
    }

    #[wasmedge_bindgen]
    pub fn send_auth(contracts: String, rates: String, authorized: String, max_stakes: String) {
        unsafe {
            let sc_wallet_ = smart_contracts::called_smart_contract_wallet();
            let sc_wallet = sc_wallet_.clone();

            if sc_wallet != PROXY_WALLET.to_string() {
                let emit1 = format!("Failed: Unauthorized sender key: {}", sc_wallet.clone());
                return;
            }

            let res = smart_contracts::authorized_currency_equiv(contracts.clone(), rates.clone(), authorized.clone(), max_stakes.clone());
        }
    }
}
