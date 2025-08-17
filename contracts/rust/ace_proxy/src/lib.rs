pub mod authorize_tokens_proxy {
    use native_functions::zera::smart_contracts;
    use native_functions::zera::types;
    use native_functions::zera::wasmedge_bindgen;

    const SMART_CONTRACT_KEY: &str = "SMART_CONTRACT_";
    const INSTANCE_KEY: &str = "INSTANCE_";
    const AUTHORIZED_KEY: &str = "AUTHORIZED_";
    const UPDATE_KEY: &str = "UPDATE_KEY_";
    const ZRA_CONTRACT: &str = "$ZRA+0000";

    #[wasmedge_bindgen]
    pub fn init() {
        unsafe {
            let (authorized, rate) = smart_contracts::get_ace_data(ZRA_CONTRACT.to_string());
            let denomination = smart_contracts::contract_denomination(ZRA_CONTRACT.to_string());
            let one_dolla = types::string_to_u256("1000000000000000000".to_string()); //change to 10 $
            let one_dolla_zera = (one_dolla * denomination) / rate;

            smart_contracts::hold(ZRA_CONTRACT.to_string(), one_dolla_zera.to_string());

            smart_contracts::store_state(
                SMART_CONTRACT_KEY.to_string(),
                "authorize_tokens_v1".to_string(),
            );
            smart_contracts::store_state(INSTANCE_KEY.to_string(), "1".to_string());
        }
    }

    fn get_instance() -> String {
        unsafe {
            let instance = smart_contracts::retrieve_state(INSTANCE_KEY.to_string());
            return instance;
        }
    }

    fn get_smart_contract() -> String {
        unsafe {
            let smart_contract = smart_contracts::retrieve_state(SMART_CONTRACT_KEY.to_string());
            return smart_contract;
        }
    }

    fn check_auth_keys() -> bool {
        unsafe {
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key == "gov_$ZRA+0000"
                || pub_key == "gov_$ACE+0000"
                || pub_key == "gov_$ZIP+0000"
            {
                return true;
            }

            let wallet = smart_contracts::wallet_address();

            let key = format!("{}{}", AUTHORIZED_KEY, wallet);

            let authorized = smart_contracts::retrieve_state(key);

            if authorized == "true" {
                return true;
            }

            return false;
        }
    }

    #[wasmedge_bindgen]
    pub fn execute(function: String, parameters: String) {
        unsafe {
            if (!check_auth_keys()) {
                let emit1 = format!("Failed: Unauthorized sender key");
                smart_contracts::emit(emit1.clone());
                return;
            }

            let smart_contract = get_smart_contract();

            if smart_contract == "" {
                let emit1 = format!("Failed: No smart contract set");
                smart_contracts::emit(emit1.clone());
                return;
            }

            let instance = get_instance();

            if instance == "" {
                let emit1 = format!("Failed: No instance set");
                smart_contracts::emit(emit1.clone());
                return;
            }

            let parameters_vec: Vec<String> = parameters
                .clone()
                .split(",")
                .map(|s| s.to_string())
                .collect();

            smart_contracts::delegatecall(
                smart_contract.clone(),
                instance.clone(),
                function.clone(),
                parameters_vec.clone(),
            );
        }
    }

    #[wasmedge_bindgen]
    pub fn update(smart_contract: String, instance: String) {
        unsafe {
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != "gov_$ZIP+0000"
                && pub_key != "gov_$ZRA+0000"
                && pub_key != "gov_$ACE+0000"
            {
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            smart_contracts::store_state(SMART_CONTRACT_KEY.to_string(), smart_contract.clone());
            smart_contracts::store_state(INSTANCE_KEY.to_string(), instance.clone());
        }
    }

    #[wasmedge_bindgen]
    pub fn remove_auth(key: String, update_key: String) {
        unsafe {
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != "gov_$ZIP+0000"
                && pub_key != "gov_$ZRA+0000"
                && pub_key != "gov_$ACE+0000"
            {
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            let state_key = format!("{}{}", AUTHORIZED_KEY, key);
            let state_key2 = format!("{}{}", UPDATE_KEY, update_key);

            smart_contracts::clear_state(state_key);
            smart_contracts::clear_state(state_key2);

            let emit1 = format!("Success: Removed key: {}", key.clone());
            let emit2 = format!("Success: Removed update key: {}", update_key.clone());

            smart_contracts::emit(emit1.clone());
            smart_contracts::emit(emit2.clone());
        }
    }

    #[wasmedge_bindgen]
    pub fn add_auth(auth_key: String, update_key: String) {
        unsafe {
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != "gov_$ZIP+0000"
                && pub_key != "gov_$ZRA+0000"
                && pub_key != "gov_$ACE+0000"
            {
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            let state_key = format!("{}{}", AUTHORIZED_KEY, auth_key);
            smart_contracts::store_state(state_key, "true".to_string());

            let state_key2 = format!("{}{}", UPDATE_KEY, update_key);
            smart_contracts::store_state(state_key2, auth_key.clone());

            let emit1 = format!("Success: Authorized key: {}", auth_key);
            smart_contracts::emit(emit1.clone());
        }
    }

    #[wasmedge_bindgen]
    pub fn update_auth(auth_key: String, update_key: String) {
        unsafe {
            let check_auth_key = format!("{}{}", AUTHORIZED_KEY, auth_key);

            let authorized_key = smart_contracts::retrieve_state(check_auth_key);

            if authorized_key != "" {
                let emit1 = format!("Failed: Key already authorized");
                smart_contracts::emit(emit1.clone());
                return;
            }

            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            let state_key = format!("{}{}", UPDATE_KEY, pub_key);

            let authorized_key = smart_contracts::retrieve_state(state_key.clone());

            if authorized_key == "" {
                let emit1 = format!("Failed: Key not authorized");
                smart_contracts::emit(emit1.clone());
                return;
            }

            if (update_key != pub_key) {
                smart_contracts::clear_state(state_key.clone());
            }

            let new_auth_key = format!("{}{}", AUTHORIZED_KEY, auth_key);
            let new_update_key = format!("{}{}", UPDATE_KEY, update_key);

            smart_contracts::store_state(new_auth_key.clone(), "true".to_string());
            smart_contracts::store_state(new_update_key.clone(), auth_key.clone());

            let emit1 = format!("Success: Updated auth key: {}", auth_key.clone());
            let emit2 = format!("Success: Updated update key: {}", update_key.clone());

            smart_contracts::emit(emit1.clone());
            smart_contracts::emit(emit2.clone());
        }
    }
}
