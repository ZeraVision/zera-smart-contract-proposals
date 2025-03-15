pub mod zera {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    //use serde::{Serialize, Deserialize};
    pub use wasmedge_bindgen_macro::wasmedge_bindgen; // 'use' is used to import an item into the current module, 'pub use' allows us to (not only import but,) re-export the item.
  
    pub mod types{
      use uint::construct_uint;
  
      construct_uint! {
          pub struct U256(4);
        }
  
        pub fn string_to_u256(input: String) -> U256 {
          let input_str = input.as_str();
          match U256::from_dec_str(input_str) {
              Ok(value) => value,
              Err(_) => {
                  println!("Error parsing string to U256");
                  U256::zero() // Return a default value, such as zero, in case of an error
              }
          }
        }
  
        pub fn is_valid_u256(input: String) -> bool {
          let input_str = input.as_str();
          U256::from_dec_str(input_str).is_ok()
        }   
    }
  
    pub mod smart_contracts {
      use crate::zera; 
      use crate::zera::types::U256;
      use crate::zera::types;
  
      pub unsafe fn store_state(key_string: String, value_string: String) -> bool {
        let key = key_string.as_str();
        let value = value_string.as_str();
  
        let key_pointer = key.as_bytes().as_ptr();
        let key_length = key.len() as i32;
  
        let value_pointer = value.as_bytes().as_ptr();
        let value_length = value.len() as i32;
    
        let result = zera::store_state(key_pointer, key_length, value_pointer, value_length);
        if result == 0 {
          return false;
        }
  
        return true;
      }
  
      pub unsafe fn clear_state(key_string: String) {
        let key = key_string.as_str();
  
        let key_pointer = key.as_bytes().as_ptr();
        let key_length = key.len() as i32;
    
        zera::clear_state(key_pointer, key_length);
      }
  
      pub unsafe fn retrieve_state(key_string: String) -> String {
        let key = key_string.as_str();
  
        let key_pointer = key.as_bytes().as_ptr();
        let key_length = key.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::retrieve_state(key_pointer, key_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
  
        return res;
      }
  
      pub unsafe fn get_ace_data(contract: String) -> (bool, U256)  {
    
        let contract_pointer = contract.as_bytes().as_ptr();
        let contract_length = contract.len() as i32;
    
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
    
        let result_len = zera::get_ace_data(contract_pointer, contract_length, target_pointer) as usize;
    
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        // Split the result string into an array of strings using comma as delimiter
        let string_array: Vec<String> = res.split(',').map(|s| s.to_string()).collect();
        let auth_string = string_array[0].clone();
        let mut auth = false;
  
        if auth_string == "true" {
          auth = true;
        }
  
        let amount = types::string_to_u256(string_array[1].clone());
    
        return (auth, amount);
     }
  
  
      pub unsafe fn db_get_data(key_string: String) -> String {
        let key = key_string.as_str();
  
        let key_pointer = key.as_bytes().as_ptr();
        let key_length = key.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::db_get_data(key_pointer, key_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        return res;
      }
  
      pub unsafe fn db_get_any_data(key_string: String, db_key_string: String) -> String
      {
        let key_pointer = key_string.as_bytes().as_ptr();
        let key_length = key_string.len() as i32;
  
        let db_key_pointer = db_key_string.as_bytes().as_ptr();
        let db_key_length = db_key_string.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::db_get_any_data(key_pointer, key_length, db_key_pointer, db_key_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn call(contract_name: String, nonce: String, function_name: String, parameters: Vec<String>) -> Vec<String> {
        let contract_name_pointer = contract_name.as_bytes().as_ptr();
        let contract_name_length = contract_name.len() as i32;
  
        let nonce_pointer = nonce.as_bytes().as_ptr();
        let nonce_length = nonce.len() as i32;
  
        let function_name_pointer = function_name.as_bytes().as_ptr();
        let function_name_length = function_name.len() as i32;
        let binding = parameters.join("##");
        let parameters_combined_in_string = binding.as_str();
        let parameters_pointer = parameters_combined_in_string.as_ptr();
        let parameters_length = parameters_combined_in_string.len() as i32;
  
        // a struct for storing result
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::call(contract_name_pointer, contract_name_length, nonce_pointer, nonce_length, function_name_pointer, function_name_length, parameters_pointer, parameters_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let results: Vec<String> = res
          .split("[res]")
          .filter(|s| !s.is_empty())
          .map(|s| s.trim_end_matches("[end]").to_string())
          .collect();
  
        return results;
      }
  
      pub unsafe fn delegatecall(contract_name: String, nonce: String, function_name: String, parameters: Vec<String>) -> Vec<String> {
        let contract_name_pointer = contract_name.as_bytes().as_ptr();
        let contract_name_length = contract_name.len() as i32;
  
        let nonce_pointer = nonce.as_bytes().as_ptr();
        let nonce_length = nonce.len() as i32;
  
        let function_name_pointer = function_name.as_bytes().as_ptr();
        let function_name_length = function_name.len() as i32;
        let binding = parameters.join("##");
        let parameters_combined_in_string = binding.as_str();
        let parameters_pointer = parameters_combined_in_string.as_ptr();
        let parameters_length = parameters_combined_in_string.len() as i32;
  
        // a struct for storing result
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::delegatecall(contract_name_pointer, contract_name_length, nonce_pointer, nonce_length, function_name_pointer, function_name_length, parameters_pointer, parameters_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let results: Vec<String> = res
          .split("[res]")
          .filter(|s| !s.is_empty())
          .map(|s| s.trim_end_matches("[end]").to_string())
          .collect();
  
        return results;
      }
  
      // pub unsafe fn randomish() -> String {
      //   let mut buffer = Vec::with_capacity(0);
      //   let target_pointer = buffer.as_mut_ptr();
  
      //   let result_len = zera::randomish(target_pointer) as usize;
  
      //   buffer.set_len(result_len);
      //   let res = String::from_utf8(buffer).unwrap();
    
      //   return res;
      // }
  
  
      // pub unsafe fn balance() -> f32 {
      //   return zera::balance();
      // }
  
      pub unsafe fn version() -> i32 {
        return zera::version();
      }
  
      pub unsafe fn emit(value: String) -> bool{
        //let value = value_string.as_str();
  
        let value_pointer = value.as_bytes().as_ptr();
        let value_length = value.len() as i32;
    
        let result = zera::emit(value_pointer, value_length);
  
        if result == 0 {
          return false;
        }
  
        return true;
      }
  
      pub unsafe fn transfer(contract_id: String, amount: String, address: String) -> bool {
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
        
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::transfer_v2(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
      pub unsafe fn current_hold(contract_id: String, amount: String) -> bool{
  
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::current_hold(contract_id_pointer, contract_id_length, amount_pointer, amount_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
  
      }
  
      pub unsafe fn delegate_send(contract_id: String, amount: String, address: String, sc_wallet: String) -> bool{
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let sc_wallet_pointer = sc_wallet.as_bytes().as_ptr();
        let sc_wallet_length = sc_wallet.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::delegate_send(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, sc_wallet_pointer, sc_wallet_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
  
      pub unsafe fn current_send(contract_id: String, amount: String, address: String) -> bool{
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::current_send(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
  
      pub unsafe fn current_send_all(wallet_address: String) -> String{
        let address_pointer = wallet_address.as_bytes().as_ptr();
        let address_length = wallet_address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::current_send_all(address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn current_mint(contract_id: String, amount: String, address: String) -> bool{
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::current_mint(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
  
      pub unsafe fn delegate_send_all(wallet_address: String, sc_wallet: String) -> String{
        let address_pointer = wallet_address.as_bytes().as_ptr();
        let address_length = wallet_address.len() as i32;
  
        let sc_wallet_pointer = sc_wallet.as_bytes().as_ptr();
        let sc_wallet_length = sc_wallet.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::delegate_send_all(address_pointer, address_length, sc_wallet_pointer, sc_wallet_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn delegate_mint(contract_id: String, amount: String, address: String, sc_wallet: String) -> bool{
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let sc_wallet_pointer = sc_wallet.as_bytes().as_ptr();
        let sc_wallet_length = sc_wallet.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::delegate_mint(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, sc_wallet_pointer, sc_wallet_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
  
      pub unsafe fn hold(contract_id: String, amount: String) -> bool{
  
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::hold(contract_id_pointer, contract_id_length, amount_pointer, amount_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
  
      }
  
      pub unsafe fn send(contract_id: String, amount: String, address: String) -> bool{
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::send(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
  
      pub unsafe fn send_all(wallet_address: String) -> String{
        let address_pointer = wallet_address.as_bytes().as_ptr();
        let address_length = wallet_address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::send_all(address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn mint(contract_id: String, amount: String, address: String) -> bool{
        let amount_pointer = amount.as_bytes().as_ptr();
        let amount_length = amount.len() as i32;
  
        let contract_id_pointer = contract_id.as_bytes().as_ptr();
        let contract_id_length = contract_id.len() as i32;
  
        let address_pointer = address.as_bytes().as_ptr();
        let address_length = address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::mint(contract_id_pointer, contract_id_length, amount_pointer, amount_length, address_pointer, address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
  
        match String::from_utf8(buffer) {
          Ok(res) => {
              if res == "OK" {
                  return true;
              }
          }
          Err(e) => {
              println!("Error converting buffer to String: {}", e);
              return false;
          }
        }
        return false;
      }
  
      pub unsafe fn last_block_time() -> u64{
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::last_block_time(target_pointer) as usize;
        buffer.set_len(result_len);
  
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let time = res.parse::<u64>().unwrap(); 
  
        return time;
      }
  
      pub unsafe fn wallet_address() -> String {
        let mut buffer = Vec::with_capacity(1024);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::wallet_address(target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn public_key() -> String {
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::public_key(target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn txn_hash() -> String {
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::txn_hash(target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
    
        return res;
      }
  
      pub unsafe fn contract_exists(contract_id: String) -> bool {
        let contract_name_pointer = contract_id.as_bytes().as_ptr();
        let contract_name_length = contract_id.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::contract_exists(contract_name_pointer, contract_name_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let mut exists = false;
  
        if res == "true" {
          exists = true;
        }
  
        return exists;
      }
  
      pub unsafe fn contract_denomination(contract_id: String) -> U256 {
        let contract_name_pointer = contract_id.as_bytes().as_ptr();
        let contract_name_length = contract_id.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::contract_denomination(contract_name_pointer, contract_name_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let denomination = types::string_to_u256(res);
  
        return denomination;
      }
  
      pub unsafe fn circulating_supply(contract_id: String) -> U256 {
        let contract_name_pointer = contract_id.as_bytes().as_ptr();
        let contract_name_length = contract_id.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::circulating_supply(contract_name_pointer, contract_name_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let circ = types::string_to_u256(res);
  
        return circ;
      }
  
      pub unsafe fn wallet_tokens(wallet_address: String) -> Vec<String>  {
        let wallet_address_pointer = wallet_address.as_bytes().as_ptr();
        let wallet_address_length = wallet_address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::wallet_tokens(wallet_address_pointer, wallet_address_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let tokens: Vec<String> = res.split(',')
                                  .map(|s| s.to_string())
                                  .collect();
  
        return tokens;
      }
      pub unsafe fn smart_contract_balance(contract_id: String) -> U256 {
        let contract_name_pointer = contract_id.as_bytes().as_ptr();
        let contract_name_length = contract_id.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::smart_contract_balance(contract_name_pointer, contract_name_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let balance = types::string_to_u256(res);
  
        return balance;
      }
  
      pub unsafe fn wallet_balance(contract_id: String, wallet_address: String) -> U256 {
        let contract_name_pointer = contract_id.as_bytes().as_ptr();
        let contract_name_length = contract_id.len() as i32;
  
        let wallet_pointer = wallet_address.as_bytes().as_ptr();
        let wallet_length = wallet_address.len() as i32;
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::wallet_balance(contract_name_pointer, contract_name_length, wallet_pointer, wallet_length, target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        let balance = types::string_to_u256(res);
  
        return balance;
      }
      pub unsafe fn smart_contract_wallet() -> String {
  
        let mut buffer = Vec::with_capacity(0);
        let target_pointer = buffer.as_mut_ptr();
  
        let result_len = zera::smart_contract_wallet(target_pointer) as usize;
  
        buffer.set_len(result_len);
        let res_ = String::from_utf8(buffer).unwrap();
        let res = res_.clone();
  
        return res;
      }
  
    }
  
    // #[repr(C)]
    // #[derive(Serialize, Deserialize, Debug)]
    // pub struct ZeraStatus {
    //   pub code: i32,
    //   pub txn_status: i32,
    // }
  
    #[link(wasm_import_module = "native_functions")]
    extern "C" {
        //*****************************
        // Zera txn functions
        //*****************************
        //original sc functions (these will take the value of the original smart contract)
        pub fn transfer_v2(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        pub fn hold(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, target_pointer: *const u8) -> i32;
        pub fn send(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        pub fn send_all(address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        pub fn mint(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        
        //any sc functions (user can specify any smart contract value, will be verified by function if it is in stack)
        pub fn delegate_send(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, sc_wallet: *const u8, sc_wallet_length: i32,target_pointer: *const u8) -> i32;
        pub fn delegate_send_all(address_pointer: *const u8, address_length: i32, sc_wallet: *const u8, sc_wallet_length: i32,target_pointer: *const u8) -> i32;
        pub fn delegate_mint(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, sc_wallet: *const u8, sc_wallet_length: i32,target_pointer: *const u8) -> i32;
  
        //current sc functions (this will take the values of the latest sc on the stack)
        pub fn current_hold(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, target_pointer: *const u8) -> i32;
        pub fn current_send(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        pub fn current_send_all(address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        pub fn current_mint(contract_id: *const u8, contract_id_length: i32, amount_pointer: *const u8, amount_length: i32, address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        
        //***************************** 
        //utils
        //*****************************
        pub fn public_key(target_pointer: *const u8) -> i32;
        pub fn txn_hash(target_pointer: *const u8) -> i32;
        pub fn wallet_address(target_pointer: *const u8) -> i32;
        pub fn last_block_time(target_pointer: *const u8) -> i32;
        pub fn wallet_tokens(address_pointer: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
        pub fn contract_exists(contract_name_pointer: *const u8, contract_name_length: i32, target_pointer: *const u8) -> i32;
        pub fn contract_denomination(contract_name_pointer: *const u8, contract_name_length: i32, target_pointer: *const u8) -> i32;
        pub fn wallet_balance(contract_name_pointer: *const u8, contract_name_length: i32, wallet_pointer: *const u8, wallet_length: i32, target_pointer: *const u8) ->i32;
        pub fn circulating_supply(contract_name_pointer: *const u8, contract_name_length: i32, target_pointer: *const u8) ->i32;
        pub fn supply_data(contract_name_pointer: *const u8, contract_name_length: i32, target_pointer: *const u8) -> i32;
        
        //original sc functions (these will take the value of the original smart contract)
        pub fn smart_contract_balance(contract_name_pointer: *const u8, contract_name_length: i32, target_pointer: *const u8) -> i32;
        pub fn smart_contract_wallet(target_pointer: *const u8) -> i32;
  
        //current sc functions (this will take the values of the latest sc on the stack)
        pub fn current_smart_contract_balance(contract_name_pointer: *const u8, contract_name_length: i32, target_pointer: *const u8) -> i32;
        pub fn current_smart_contract_wallet(target_pointer: *const u8) -> i32;
        pub fn version() -> i32;
  
        //needed for state management
        pub fn call(contract_name_pointer: *const u8, contract_name_length: i32, nonce_pointer: *const u8, nonce_length: i32, function_name_pointer: *const u8, function_name_length: i32, parameters_pointer: *const u8, parameters_length: i32, target_pointer: *const u8) -> i32;
        pub fn delegatecall(contract_name_pointer: *const u8, contract_name_length: i32, nonce_pointer: *const u8, nonce_length: i32, function_name_pointer: *const u8, function_name_length: i32, parameters_pointer: *const u8, parameters_length: i32, target_pointer: *const u8) -> i32;
        pub fn store_state(key_pointer: *const u8, key_length: i32, value_pointer: *const u8, value_length: i32) -> i32;
        pub fn retrieve_state(key_pointer: *const u8, key_length: i32, target_pointer: *const u8) -> i32;
        pub fn clear_state(key_pointer: *const u8, key_length: i32) -> i32;
        pub fn db_get_any_data(key_pointer: *const u8, key_length: i32, db_key_pointer: *const u8, db_key_length: i32, target_pointer: *const u8) -> i32;
        pub fn db_get_data(key_pointer: *const u8, key_length: i32, target_pointer: *const u8) -> i32;
        pub fn get_ace_data(contract_pointer: *const u8, contract_length: i32, target_pointer: *const u8) -> i32;
        pub fn emit(value_pointer: *const u8, value_length: i32) -> i32;
    }
  }