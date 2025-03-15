pub mod zera_supply_management_proxy {
    use native_functions::zera::wasmedge_bindgen;
    use native_functions::zera::smart_contracts;

    const SMART_CONTRACT_KEY: &str = "SMART_CONTRACT_";
    const INSTANCE_KEY: &str = "INSTANCE_";

    #[wasmedge_bindgen]
    pub fn init() {
        unsafe{
            smart_contracts::store_state(SMART_CONTRACT_KEY.to_string(), "zera_supply_management_v2".to_string());
            smart_contracts::store_state(INSTANCE_KEY.to_string(), "1".to_string());
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

    #[wasmedge_bindgen]
    pub fn execute(function: String, parameters: String) {
        unsafe{

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


            let parameters_vec: Vec<String> = parameters.clone().split("##").map(|s| s.to_string()).collect();

            smart_contracts::delegatecall(smart_contract.clone(), instance.clone(), function.clone(), parameters_vec.clone());
        }
    
    }

    #[wasmedge_bindgen]
    pub fn update(smart_contract: String, instance: String) {
        unsafe{
            let pub_key_ = smart_contracts::public_key();
            let pub_key = pub_key_.clone();

            if pub_key != "gov_$ZRA+0000" {
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            smart_contracts::store_state(SMART_CONTRACT_KEY.to_string(), smart_contract.clone());
            smart_contracts::store_state(INSTANCE_KEY.to_string(), instance.clone());
        }
    }
}