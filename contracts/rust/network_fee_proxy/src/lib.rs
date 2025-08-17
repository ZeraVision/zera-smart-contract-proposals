pub mod network_fee_proxy {
    use native_functions::zera::wasmedge_bindgen;
    use native_functions::zera::smart_contracts;
    use native_functions::zera::types;
    use native_functions::zera::types::U256;

    const SMART_CONTRACT_KEY: &str = "SMART_CONTRACT_";
    const INSTANCE_KEY: &str = "INSTANCE_";
    const NETWORK_FEE_KEY: &str = "NETWORK_SC";
    const ZRA_CONTRACT: &str = "$ZRA+0000";

    #[wasmedge_bindgen]
    pub fn init() {
        unsafe{
            let (authorized, rate) = smart_contracts::get_ace_data(ZRA_CONTRACT.to_string());
            let denomination = smart_contracts::contract_denomination(ZRA_CONTRACT.to_string());
            let one_dolla = types::string_to_u256("1000000000000000000".to_string()); //change to 10 $
            let one_dolla_zera = (one_dolla * denomination) / rate;

            smart_contracts::hold(ZRA_CONTRACT.to_string(), one_dolla_zera.to_string());

            smart_contracts::store_state(SMART_CONTRACT_KEY.to_string(), "network_fees_v1".to_string());
            smart_contracts::store_state(INSTANCE_KEY.to_string(), "1".to_string());
            smart_contracts::store_state(NETWORK_FEE_KEY.to_string(), "network_fees_v1_1".to_string());
        }
    }

    fn get_instance() -> String {
        unsafe{
            let instance = smart_contracts::retrieve_state(INSTANCE_KEY.to_string());
            return instance;
        }
    }

    fn get_smart_contract() -> String {
        unsafe{
            let smart_contract = smart_contracts::retrieve_state(SMART_CONTRACT_KEY.to_string());
            return smart_contract;
        }
    }

    //change to update multiple fees at once
    #[wasmedge_bindgen]
    pub fn execute(function: String, parameters: String) {
        unsafe{

            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != "gov_$ZIP+0000" && pub_key != "gov_$ZRA+0000"{
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
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


            let parameters_vec: Vec<String> = parameters.clone().split(",").map(|s| s.to_string()).collect();

            smart_contracts::delegatecall(smart_contract.clone(), instance.clone(), function.clone(), parameters_vec.clone());
        }
    
    }

    #[wasmedge_bindgen]
    pub fn update(smart_contract: String, instance: String) {
        unsafe{
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != "gov_$ZIP+0000" && pub_key != "gov_$ZRA+0000"{
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            smart_contracts::store_state(SMART_CONTRACT_KEY.to_string(), smart_contract.clone());
            smart_contracts::store_state(INSTANCE_KEY.to_string(), instance.clone());

            let network_sc = format!("{}_{}", smart_contract.clone(), instance.clone());
            smart_contracts::store_state(NETWORK_FEE_KEY.to_string(), network_sc.clone());
        }
    }
}