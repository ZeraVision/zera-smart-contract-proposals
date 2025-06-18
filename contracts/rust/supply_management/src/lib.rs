pub mod zera_supply_management_v4 {
    use native_functions::zera::wasmedge_bindgen;
    use native_functions::zera::smart_contracts;
    use native_functions::zera::types::U256;
    use native_functions::zera::types;

    const REFUND_PERIOD: u64 = 7776000; // 3 months in seconds
    const REFUND_PERIOD_WEEKS: u64 = 604800; // 1 week in seconds
    const MAX_REFUND_PERIOD: u64 = 11491200; // 19 weeks in seconds


    const PROXY_WALLET: &str = "5Bq2FibBdw3N664ybFbAumFhi5gpeq29FZRHq4rxkwo3";
    const TREASURY_WALLET: &str = "4Yg2ZeYrzMjVBXvU2YWtuZ7CzWR9atnQCD35TQj1kKcH";
    const ZRA_CONTRACT: &str = "$ZRA+0000";
    const BURN_WALLET: &str = ":fire:";
    const START_STATE: &str = "START";
    const REFUND_STATE: &str = "REFUND";
    const DONE_STATE: &str = "DONE";
    const OPTION_A: &str = "A";
    const OPTION_B: &str = "B";

    const TOTAL_BURNED_KEY: &str = "TOTAL_BURNED_";
    const UNALLOCATED_SUPPLY_KEY: &str = "UNALLOCATED_SUPPLY_";
    const MINT_SELF_KEY: &str = "MINT_SELF_";
    const STARTING_TIME_KEY: &str = "STARTING_TIME_";
    const BURN_TIME_KEY: &str = "BURN_TIME_";
    const BUCKET_NUMBER_KEY: &str = "BUCKET_NUMBER_";
    const TAKEN_FROM_KEY: &str = "TAKEN_FROM_";
    const SWAP_TIME_KEY: &str = "SWAP_TIME_";
    const REFUND_AMOUNT_KEY: &str = "REFUND_AMOUNT_";
    const REFUND_SENT_KEY: &str = "REFUND_SENT_";
    const REFUND_TOKEN_SENT_KEY: &str = "REFUND_TOKEN_SENT_";
    const REFUND_WALLET_ADDRESS_KEY: &str = "REFUND_WALLET_ADDRESS_";
    const REFUND_STATE_KEY: &str = "REFUND_STATE_";
    const REFUND_TIME_KEY: &str = "REFUND_TIME_";
    const REFUND_LATER_KEY: &str = "REFUND_LATER_";
    const ACE_DATA_KEY: &str = "ACE_DATA_";
    const INITIAL_RATE_KEY: &str = "INITIAL_RATE_";
    const ALL_TXNS_KEY: &str = "ALL_TXNS_";
    const TOKEN_PURCHASE_POWER_KEY: &str = "TOKEN_PURCHASE_POWER_";
    const SEND_ALL_KEY: &str = "SEND_ALL_";
    const PAUSE_BURN_KEY: &str = "PAUSE_BURN_";


    #[wasmedge_bindgen]
    pub fn init() {
        unsafe{

            let (authorized, rate) = smart_contracts::get_ace_data(ZRA_CONTRACT.to_string());
            let denomination = smart_contracts::contract_denomination(ZRA_CONTRACT.to_string());
            let one_dolla = types::string_to_u256("1000000000000000000".to_string());
            let one_dolla_zera = (one_dolla * denomination) / rate;

            smart_contracts::hold(ZRA_CONTRACT.to_string(), one_dolla_zera.to_string());

          let key_ace_data1 = format!("{}{}", ACE_DATA_KEY.to_string(), "$IIT+0000".to_string());
          let key_ace_data2 = format!("{}{}", ACE_DATA_KEY.to_string(), "$RUBY+0000".to_string());
          let key_ace_data3 = format!("{}{}", ACE_DATA_KEY.to_string(), "$RUBY+0001".to_string());
          let key_ace_data4 = format!("{}{}", ACE_DATA_KEY.to_string(), "$RUBY+0002".to_string());
          let key_ace_data5 = format!("{}{}", ACE_DATA_KEY.to_string(), "$RUBY+0003".to_string());

          smart_contracts::store_state(key_ace_data1.clone(), "1040000000000000000,1000000000000000000000000000000".to_string()); //done
          smart_contracts::store_state(key_ace_data2.clone(), "333333333333333333,1000000000000000000000000000".to_string()); //done
          smart_contracts::store_state(key_ace_data3.clone(), "333333333333333333,1000000000000000000000000000".to_string()); //done
          smart_contracts::store_state(key_ace_data4.clone(), "333333333333333333,1000000000000000000000000000".to_string()); //done
          smart_contracts::store_state(key_ace_data5.clone(), "333333333333333333,1000000000000000000000000000".to_string()); //done

          let purchase_power1 = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), "$RUBY+0000".to_string());
          let purchase_power2 = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), "$RUBY+0001".to_string());
          let purchase_power3 = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), "$RUBY+0002".to_string());
          let purchase_power4 = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), "$RUBY+0003".to_string());
          let purchase_power5 = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), "$IIT+0000".to_string());

          smart_contracts::store_state(purchase_power1.clone(), "935557874884000000064442128".to_string()); //done
          smart_contracts::store_state(purchase_power2.clone(), "932782750000000000067217250".to_string()); //done
          smart_contracts::store_state(purchase_power3.clone(), "934051000000000000065949000".to_string()); //done
          smart_contracts::store_state(purchase_power4.clone(), "935636312500000000064363688".to_string()); //done
          smart_contracts::store_state(purchase_power5.clone(), "1000000000000000000000000000000".to_string()); //done
          
          smart_contracts::store_state(STARTING_TIME_KEY.to_string(), "1733966306".to_string()); //done
          smart_contracts::store_state(BUCKET_NUMBER_KEY.to_string(), "586".to_string()); //done
          smart_contracts::store_state(MINT_SELF_KEY.to_string(), "997000000000000000000000".to_string()); //done
          smart_contracts::store_state(BURN_TIME_KEY.to_string(), "1748189345".to_string()); //done
          smart_contracts::store_state(INITIAL_RATE_KEY.to_string(), "3179880259022495537".to_string()); //done
          smart_contracts::store_state(TAKEN_FROM_KEY.to_string(), "73191728042407".to_string()); //done
          smart_contracts::store_state(UNALLOCATED_SUPPLY_KEY.to_string(), "579519531651778015".to_string()); //done
          smart_contracts::store_state(TOTAL_BURNED_KEY.to_string(), "53545294308641906".to_string()); //done
        }
    }

    fn get_unallocated_supply() -> U256 {
        unsafe{
            let state_unallocated_supply_ = smart_contracts::retrieve_state(UNALLOCATED_SUPPLY_KEY.to_string());
            let mut unallocated_supply = types::string_to_u256(state_unallocated_supply_.clone());

            return unallocated_supply;
        }
    }

    fn get_total_burned() -> U256 {
        unsafe{
            let state_total_burned_ = smart_contracts::retrieve_state(TOTAL_BURNED_KEY.to_string());
            let mut total_burned = types::string_to_u256(state_total_burned_.clone());

            return total_burned;
        }
    }
    fn update_total_burned(burn_amount: U256) {
        unsafe{
            let total_burned_state = smart_contracts::retrieve_state(TOTAL_BURNED_KEY.to_string());
            let mut total_burned = U256::zero();

            if !total_burned_state.is_empty(){
                total_burned = types::string_to_u256(total_burned_state.clone());
            }

            total_burned += burn_amount;
            smart_contracts::store_state(TOTAL_BURNED_KEY.to_string(), total_burned.to_string());
        }
    }
    fn update_unallocated_supply(mint_amount: U256, burn_amount: U256) {
        unsafe{
            let unallocated_supply = get_unallocated_supply();
            let new_unallocated_supply = unallocated_supply - mint_amount - burn_amount;
            smart_contracts::store_state(UNALLOCATED_SUPPLY_KEY.to_string(), new_unallocated_supply.to_string());
        }
    }
    fn get_burned_time() -> u64 {
        unsafe{
            let current_time = smart_contracts::last_block_time();
            let state_burn_time_ = smart_contracts::retrieve_state(BURN_TIME_KEY.to_string());
            let state_burn_time = state_burn_time_.clone();
            let mut burn_time = 0 as u64;

            if state_burn_time.is_empty() {
                burn_time = current_time;
                smart_contracts::store_state(BURN_TIME_KEY.to_string(), current_time.to_string());
            }
            else {
                burn_time = match state_burn_time.parse::<u64>() {
                    Ok(time) => time,
                    Err(e) => {
                        smart_contracts::emit(format!("Failed: Error parsing state_starting_time: {}", e));
                        return 0; // or handle the error appropriately
                    }
                };
            }

            let elapsed_burn_time = current_time - burn_time;
            return elapsed_burn_time;
        }
    }
    fn get_elapsed_time() -> u64 {
        unsafe {
            let current_time = smart_contracts::last_block_time();
            let state_starting_time = smart_contracts::retrieve_state(STARTING_TIME_KEY.to_string());
            let mut start_time = 0 as u64; // Get from storage

            if state_starting_time.is_empty() {
                start_time = current_time;
                smart_contracts::store_state(STARTING_TIME_KEY.to_string(), current_time.to_string());
            }
            else {
                start_time = match state_starting_time.parse::<u64>() {
                    Ok(time) => time,
                    Err(e) => {
                        smart_contracts::emit(format!("Failed: Error parsing state_starting_time: {}", e));
                        return 0; // or handle the error appropriately
                    }
                };
            }
            
            let elapsed_time = current_time - start_time;
            return elapsed_time;
        }
    }

    fn get_mint_amount(purchase_power: U256, initial_rate: U256) -> (U256, U256, i32, U256, U256, u64)
    {
        unsafe
        {
        // Setup variables
        let precision_scale = U256::from(1_000_000_000_000_000_000u64); // Scale by 10^18 for higher precision
        let initial_amount = U256::from(800_000_000) * precision_scale; // 800 ZRA, scaled
        let initial_percent = U256::from(40) * precision_scale / U256::from(100_000); // 0.040%, scaled
        let cut_rate_per_bucket = U256::from(40) * precision_scale / U256::from(100_000); // 0.040%, scaled
        let rate_increment = precision_scale * U256::from(2) / U256::from(1_000); // 0.0020, scaled

        //RETRIEVE FROM STORAGE!
        let mut bucket_number = 1; // last bucket number we were in, must be > 0, start at 1
        let mut amount_taken_from_current_bucket = U256::from(0) * precision_scale; // scaled number of coins pulled //! pull from storage

        let bucket_number_state = smart_contracts::retrieve_state(BUCKET_NUMBER_KEY.to_string());
        if !bucket_number_state.is_empty(){
            bucket_number = match bucket_number_state.parse::<i32>() {
                Ok(value) => value,
                Err(e) => {
                    println!("Error parsing string to i32: {}", e);
                    return (U256::zero(), U256::zero(), 0, U256::zero(), U256::zero(), 0); // Handle the error appropriately
                }
            };
        }

        if bucket_number > 10000 {
            return (U256::zero(), U256::zero(), 0, U256::zero(), U256::zero(), 0);// max bucket, no more...
        }

        let amount_taken_from_current_bucket_state = smart_contracts::retrieve_state(TAKEN_FROM_KEY.to_string());

        if !amount_taken_from_current_bucket_state.is_empty(){
            amount_taken_from_current_bucket = types::string_to_u256(amount_taken_from_current_bucket_state);
        }
        amount_taken_from_current_bucket *= U256::from(1_000_000_000);
        // Scale the purchase value
        let scaled_value = purchase_power; // //! calculations will differ on network as this is accomodating for a hard coded float


        // Call purchase_coins
            // mint (parts)     // store          // store (parts)             // emit (1e18)
        let (total_coins, max_bucket, taken_from_last_bucket, last_bucket_rate, amount_to_burn, months_burned) = purchase_coins(
            bucket_number, //input //!
            amount_taken_from_current_bucket, // taken from current bucket by users //!
            scaled_value, //input - USD 1e18 //!
            initial_rate, //input
            rate_increment,
            precision_scale,
            initial_amount,
            initial_percent,
            cut_rate_per_bucket,
        );

        return (total_coins, last_bucket_rate, max_bucket, taken_from_last_bucket, amount_to_burn, months_burned);
    }
    }
    fn purchase_coins(
        start_bucket: i32,
        amount_taken_from_current_bucket: U256,
        scaled_value: U256, // Already scaled value
        initial_rate: U256,
        rate_increment: U256,
        precision_scale: U256,
        initial_amount: U256,
        initial_percent: U256,
        cut_rate_per_bucket: U256,
    ) -> (U256, i32, U256, U256, U256, u64) { // Added U256 to return last bucket rate
        let mut remaining_value = scaled_value; // The amount left to spend
        let mut total_coins = U256::zero(); // The total coins purchased
        let mut current_bucket = start_bucket; // The current bucket we're processing
        let mut current_bucket_rate = initial_rate; // Track the rate of the last bucket processed
        let mut total_amount_taken_from_current_bucket: U256 = U256::zero();
        let mut next_bucket:bool = false;
    
        // Get the burn scale for the current month upfront
        let (burn_scale, amount_to_burn, months_burned) = calculate_burn(precision_scale);
    
        while remaining_value > U256::zero() {
    
            // Get the total coins available in the current bucket
            let raw_available_coins = get_coins_for_bucket(
                current_bucket,
                cut_rate_per_bucket,
                initial_amount,
                initial_percent,
                precision_scale,
            );
    
            // Apply the burn scale to adjust available coins
            let adjusted_available_coins = (raw_available_coins * burn_scale) / precision_scale;
    
            // Subtract the amount already taken from the current bucket
            let remaining_coins_in_bucket = if current_bucket == start_bucket {
                adjusted_available_coins.saturating_sub(amount_taken_from_current_bucket)
            } else {
                adjusted_available_coins
            };
    
            // Calculate the value per coin in the current bucket (rate)
            let value_per_coin = current_bucket_rate;
    
            // Max value that can be taken from the current bucket
            let max_value_from_bucket = remaining_coins_in_bucket * value_per_coin / precision_scale;
    
            if remaining_value >= max_value_from_bucket {
                // If we can afford all coins in this bucket
                total_coins += remaining_coins_in_bucket; // Add all coins from this bucket
                remaining_value -= max_value_from_bucket; // Reduce remaining value
    
                current_bucket += 1; // Move to the next bucket
    
                let current_bucket_addition = current_bucket_rate * rate_increment / precision_scale;
    
                current_bucket_rate = current_bucket_rate + current_bucket_addition; // Update the last rate

                next_bucket = true; 

                if remaining_value == U256::zero() {
                    total_amount_taken_from_current_bucket = U256::zero();
                }
            } else {
                // If we can't afford all coins in this bucket
                let coins_to_take = remaining_value * precision_scale / value_per_coin;
                total_coins += coins_to_take; // Add fraction of coins
                remaining_value = U256::zero(); // Exhaust the remaining value
                
                if next_bucket {
                    total_amount_taken_from_current_bucket = coins_to_take;
                } else {
                    total_amount_taken_from_current_bucket = amount_taken_from_current_bucket + coins_to_take;
                }
            }
    
        }
    
        // Convert to easy to use data
        //* Convert from 1e18 to 1e9 (1e9 part per zra)
        let zra_relative_scale_factor = U256::from(1_000_000_000);
        total_coins /= zra_relative_scale_factor;
        total_amount_taken_from_current_bucket /= zra_relative_scale_factor;
    
        // Return the total coins purchased, the last bucket processed, the remaining coins, and the rate of the last bucket
        return (total_coins, current_bucket, total_amount_taken_from_current_bucket, current_bucket_rate, amount_to_burn, months_burned)
    }

    // Optimized using integer-based geometric progression
fn get_coins_for_bucket(
    bucket_number: i32,
    cut_rate_per_bucket: U256,
    initial_amount: U256,
    initial_percent: U256,
    precision_scale: U256,
) -> U256 {
    let initial_amount_scaled = initial_amount * initial_percent / precision_scale;
    let decay_factor_scaled = precision_scale - cut_rate_per_bucket;

    // Use exponentiation by squaring to compute decay_factor_scaled^(bucket_number - 1)
    let decay_power = pow(decay_factor_scaled, (bucket_number - 1) as u32, precision_scale);

    // Compute the amount for the given bucket
    return initial_amount_scaled * decay_power / precision_scale
}

// Integer exponentiation with scaling
fn pow(base: U256, exp: u32, scale: U256) -> U256 {
    let mut result = scale; // Start with 1 in scaled precision
    let mut base = base;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base / scale; // Scale-adjusted multiplication
        }
        base = base * base / scale; // Square the base, scale-adjusted
        exp /= 2;
    }

    return result
}


fn calculate_burn(precision_scale: U256) -> (U256, U256, u64) {

    unsafe{
        let pause = smart_contracts::retrieve_state(PAUSE_BURN_KEY.to_string());
        let mut burn_amount: U256 = U256::zero();
        let mut burn_scale = precision_scale; // Start with 100% represented as precision_scale
        let mut last_burn_periods_elapsed = 0;
        
    if pause != "1" {  
        let seconds_elapsed: u64 = get_elapsed_time(); // burn period is once every 2.592M
        let last_burn_seconds_elapsed: u64 = get_burned_time(); // burn period is once every 2.592M
        let mut unallocated_supply: U256 = get_unallocated_supply(); 


        let burn_periods_elapsed = seconds_elapsed / 2_592_000; // one period per 2.592M seconds
        last_burn_periods_elapsed = last_burn_seconds_elapsed / 2_592_000;

        let mut x = 0;
        let mut month =  burn_periods_elapsed - last_burn_periods_elapsed + 1;

        while x < last_burn_periods_elapsed{
            let burn_rate = if month <= 3 {
                U256::from(125) // 1.25% burn per month
            } else if month <= 9 {
                U256::from(250) // 2.5% burn per month
            } else if month <= 21 {
                U256::from(500) // 5% burn per month
            } else if month <= 45 {
                U256::from(1000) // 10% burn per month
            } else if month <= 93 {
                U256::from(2000) // 20% burn per month
            } else {
                U256::from(4000) // 40% burn per month
            };

            let temp_burn_amount = (unallocated_supply * burn_rate) / U256::from(10000);
            unallocated_supply -= temp_burn_amount;
            burn_amount += temp_burn_amount;
            x += 1;
            month += 1;
        }
    }

    let mut total_burned: U256 = get_total_burned(); 
    total_burned += burn_amount;
    let initial_amount = U256::from(800_000_000_000_000_000u64);

    let remaining_amount = initial_amount - total_burned;

    burn_scale = remaining_amount * precision_scale / initial_amount;

    (burn_scale, burn_amount, last_burn_periods_elapsed) // Return the correct tuple
    }
    
}


    fn refund_states(amount_sent : String, token_sent : String, amount_minted : String, wallet_address : String)
    {
        unsafe{
        let txn_hash_ = smart_contracts::txn_hash();
        let txn_hash = txn_hash_.clone();
        let current_time_ = smart_contracts::last_block_time();
        let current_time = current_time_.to_string();

        let key_time = format!("{}{}", SWAP_TIME_KEY.to_string(), txn_hash.clone());
        let key_amount_minted = format!("{}{}", REFUND_AMOUNT_KEY.to_string(), txn_hash.clone());
        let key_amount_sent = format!("{}{}", REFUND_SENT_KEY.to_string(), txn_hash.clone());
        let key_token_sent = format!("{}{}", REFUND_TOKEN_SENT_KEY.to_string(), txn_hash.clone());
        let key_wallet_address = format!("{}{}", REFUND_WALLET_ADDRESS_KEY.to_string(), txn_hash.clone());
        let key_current_state = format!("{}{}", REFUND_STATE_KEY.to_string(), txn_hash.clone());

        smart_contracts::store_state(key_time.clone(), current_time.to_string());
        smart_contracts::store_state(key_amount_minted.clone(), amount_minted.clone());
        smart_contracts::store_state(key_amount_sent.clone(), amount_sent.clone());
        smart_contracts::store_state(key_token_sent.clone(), token_sent.clone());
        smart_contracts::store_state(key_wallet_address.clone(), wallet_address.clone());
        smart_contracts::store_state(key_current_state.clone(), START_STATE.to_string());  
        
        let mut all_txn_hash = smart_contracts::retrieve_state(ALL_TXNS_KEY.to_string());
        all_txn_hash = all_txn_hash + &txn_hash + ",";
        smart_contracts::store_state(ALL_TXNS_KEY.to_string(), all_txn_hash.clone());
        }
    }

    fn check_sc_wallet()
    {
        unsafe{
            let (auto, rate) = smart_contracts::get_ace_data(ZRA_CONTRACT.to_string());

            let balance = smart_contracts::wallet_balance(ZRA_CONTRACT.to_string(), PROXY_WALLET.to_string());

            let mint_state = smart_contracts::retrieve_state(MINT_SELF_KEY.to_string());

            if !types::is_valid_u256(mint_state.clone()){
                return;
            }

            let mint_left = types::string_to_u256(mint_state.clone());

            if mint_left == U256::zero(){
                return;
            }
            let denomination = U256::from(1_000_000_000u64);
            let value_multiplier = U256::from(1_000_000_000_000_000_000u64); //multiplier for value
            let value = (balance * rate) / denomination;  //find how much zra value sc_wallet has
            let lowest_value = value_multiplier * U256::from(1_000u64); //1000$ value

            if value < lowest_value{
                let mut mint_amount = (lowest_value * denomination) / rate;
                
                let mut new_mint_left = U256::zero();

                if lowest_value > mint_left{
                    mint_amount = (mint_left* denomination) / rate;
                }
                else{
                    new_mint_left = mint_left - lowest_value;
                }
                
                smart_contracts::delegate_mint(ZRA_CONTRACT.to_string(), mint_amount.to_string(), PROXY_WALLET.to_string(), PROXY_WALLET.to_string());


                smart_contracts::store_state(MINT_SELF_KEY.to_string(), new_mint_left.to_string());
                return;
            }  
        }
    }

    #[wasmedge_bindgen]
    pub fn swap(contract_id: String, amount: String, min_zra: String){
      unsafe {

        let sc_wallet_ = smart_contracts::called_smart_contract_wallet();
        let sc_wallet = sc_wallet_.clone();

        if sc_wallet != PROXY_WALLET.to_string(){
            let emit1 = format!("Failed: Unauthorized sender key: {}", sc_wallet.clone());
            smart_contracts::emit(emit1.clone());
            return;
        }

        let key_ace_data = format!("{}{}", ACE_DATA_KEY.to_string(), contract_id);
        let ace_data = smart_contracts::retrieve_state(key_ace_data.clone());

        if ace_data == "\0"{
            smart_contracts::emit(format!("Failed: Token is not authorized for supply management. {}", contract_id.clone()));
            return;
        }

        let split_ace_data: Vec<&str> = ace_data.split(',').collect();

        if split_ace_data.len() < 2 {
            smart_contracts::emit("Failed: Invalid ACE data format.".to_string());
            return;
        }

        let rate_str = split_ace_data[0].to_string();
        let contract_purchase_power = split_ace_data[1].to_string();
        let mut rate = types::string_to_u256(rate_str.clone());


        if !smart_contracts::contract_exists(contract_id.clone())
        {
            let message = format!("Failed: Token does not exist: {}", contract_id.clone());
            smart_contracts::emit(message.clone());
            return;
        }

        let purchase_power_key = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), contract_id.clone());

        let purchase_power_string = smart_contracts::retrieve_state(purchase_power_key.clone());

        if(!types::is_valid_u256(purchase_power_string.clone()))
        {
            smart_contracts::emit("Failed: Invalid purchase power.".to_string());
            return;
        }

        let purchase_power = types::string_to_u256(purchase_power_string.clone());

        if rate_str == "0"
        {
            let (authorized, rate_temp) = smart_contracts::get_ace_data(contract_id.clone());
            rate = rate_temp;
        }
        
        let initial_rate_state = smart_contracts::retrieve_state(INITIAL_RATE_KEY.to_string());
        let mut initial_rate = U256::from(1_000_000_000_000_000_000u64);

        if !initial_rate_state.is_empty(){
            initial_rate = types::string_to_u256(initial_rate_state);
        }

        check_sc_wallet();

        let denomination = smart_contracts::contract_denomination(contract_id.clone());
        let u256_amount = types::string_to_u256(amount.clone());
        let purchase_power_user = (rate * u256_amount) / denomination;

        if(purchase_power_user > purchase_power){
            let message_str = format!("Failed: Purchase power exceeded: {}, purchase power user: {}", purchase_power.clone(), purchase_power_user.to_string());
            smart_contracts::emit(message_str.clone());
            return;
        }
        
        let (mint_amount, last_bucket_rate, max_bucket, taken_from_last_bucket, amount_to_burn, months_burned) = get_mint_amount(purchase_power_user, initial_rate);

        if(mint_amount == U256::zero()){
            smart_contracts::emit("Failed: No more tokens available for swap.".to_string());
            return;
        }

        let zra_min = types::string_to_u256(min_zra);
        
        if mint_amount < zra_min {
            let message_str = format!("Failed: Swap amount is below minimum: {}, mint amount: {}", zra_min.clone(), mint_amount.to_string());
            smart_contracts::emit(message_str.clone());
            return;
        }

        if !smart_contracts::transfer(contract_id.clone(), amount.clone(), PROXY_WALLET.to_string())
        {
            smart_contracts::emit("Failed: Hold.".to_string());
            return;
        }
        
        let wallet_address = smart_contracts::wallet_address();

        let mint_amount_str = mint_amount.to_string();

        if !smart_contracts::delegate_mint(ZRA_CONTRACT.to_string(), mint_amount_str.clone(), wallet_address.clone(), PROXY_WALLET.to_string())
        {
            smart_contracts::delegate_send(contract_id.clone(), amount.clone(), wallet_address.clone(), PROXY_WALLET.to_string());
            smart_contracts::emit("Failed: Minting.".to_string());
            return;
        }

        // Store the new bucket number, amount taken from current bucket, and initial rate
        smart_contracts::store_state(BUCKET_NUMBER_KEY.to_string(), max_bucket.to_string());
        smart_contracts::store_state(TAKEN_FROM_KEY.to_string(), taken_from_last_bucket.to_string());
        smart_contracts::store_state(INITIAL_RATE_KEY.to_string(), last_bucket_rate.to_string());

        //Update new purchase power of token used to swap
        let purchase_power_stats = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), contract_id.clone());
        let token_purchase_power_ = smart_contracts::retrieve_state(purchase_power_stats.clone());
        let token_purchase_power = types::string_to_u256(token_purchase_power_.clone());
        let new_purchase_power = token_purchase_power - purchase_power;
        smart_contracts::store_state(purchase_power_stats.clone(), new_purchase_power.to_string());

        if amount_to_burn > U256::zero()
        {
            let burn_str = amount_to_burn.to_string();
            smart_contracts::delegate_mint(ZRA_CONTRACT.to_string(), burn_str.clone(), BURN_WALLET.to_string(), PROXY_WALLET.to_string());

            let state_burn_time_ = smart_contracts::retrieve_state(BURN_TIME_KEY.to_string());
            let state_burn_time = state_burn_time_.clone();
            let mut burn_time = 0 as u64;
        
            burn_time = match state_burn_time.parse::<u64>() {
                Ok(time) => time,
                Err(e) => {
                    smart_contracts::emit(format!("Failed: Error parsing state_starting_time: {}", e));
                    return; // or handle the error appropriately
                }
            };
            
            burn_time = burn_time + (months_burned * 2_592_000); // 2.592M seconds in a month

            smart_contracts::store_state(BURN_TIME_KEY.to_string(), burn_time.to_string());
            update_total_burned(amount_to_burn);
        }

        update_unallocated_supply(mint_amount, amount_to_burn);

    
        refund_states(amount.clone(), contract_id.clone(), mint_amount.to_string(), wallet_address.clone());
        
        let last_bucket_rate_string = "Success: Last rate: ".to_string() + &last_bucket_rate.to_string();
    
        smart_contracts::emit(last_bucket_rate_string.clone());
    }
    }

    //function to set the accepted tokens for supply management
    #[wasmedge_bindgen]
    pub fn set_auth_tokens(contract_id: String, rate: String, purchase_power: String){

        unsafe{
        
        let pub_key = smart_contracts::public_key();
        
        if pub_key != "gov_$ZRA+0000" {
            let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
            smart_contracts::emit(format!("Unauthorized sender key: {}", pub_key.clone()));
            return;
        }

        if !smart_contracts::contract_exists(contract_id.clone())
        {
            smart_contracts::emit(format!("Failed: Contract does not exist: {}", contract_id.clone()));
            return;
        }


        if !types::is_valid_u256(rate.clone()) || !types::is_valid_u256(purchase_power.clone()) {
            let emit2 = "Failed: Rate and purchase power must be valid numbers.".to_string();
            smart_contracts::emit(emit2.clone());
            return;
        }

        let value_contract_stats = format!("{},{}", rate, purchase_power.clone());
        let key_contract_stats = format!("{}{}", ACE_DATA_KEY.to_string(), contract_id.clone());
        let purchase_power_stats = format!("{}{}", TOKEN_PURCHASE_POWER_KEY.to_string(), contract_id.clone());

        smart_contracts::store_state(purchase_power_stats.clone(), purchase_power.clone());
        smart_contracts::store_state(key_contract_stats.clone(), value_contract_stats.clone());
        smart_contracts::emit(format!("Success: Token added for supply management. {}", contract_id.clone()));

        }
    }

    #[wasmedge_bindgen]
    pub fn remove_auth_tokens(contract_id: String){
        unsafe{
        let pub_key = smart_contracts::public_key();
        
        if pub_key != "gov_$ZRA+0000" {
            let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
            smart_contracts::emit(emit1.clone());
            return;
        }

        let key_contracts_states = format!("{}{}", ACE_DATA_KEY.to_string(), contract_id);
        smart_contracts::clear_state(key_contracts_states.clone());
        }
    }

    #[wasmedge_bindgen]
    pub fn send_all(wallet_address: String){
        unsafe{
             let pub_key = smart_contracts::public_key();
            
            if pub_key != "gov_$ZRA+0000"{
                let emit1 = format!("Failed: Unauthorized sender key: {}", pub_key.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }
            
            let send_all = smart_contracts::retrieve_state(SEND_ALL_KEY.to_string());

            let current_time = smart_contracts::last_block_time();

            if send_all.is_empty(){
                smart_contracts::store_state(SEND_ALL_KEY.to_string(), current_time.to_string());
                smart_contracts::emit("Success: Send all initiated.".to_string());
                return;
            }
            else{
                let mut send_all_time = match send_all.parse::<u64>() {
                    Ok(value) => value,
                    Err(e) => {
                        smart_contracts::emit(format!("Failed: Error parsing string to u64: {}", e));
                        return; // Handle the error appropriately
                    }
                };

                send_all_time = send_all_time + MAX_REFUND_PERIOD;

                if current_time > send_all_time{
                    smart_contracts::clear_state(SEND_ALL_KEY.to_string());
                    smart_contracts::delegate_send_all(wallet_address.clone(), PROXY_WALLET.to_string());
                    smart_contracts::emit("Success: Sent all coins".to_string());
                    return;
                }
                else
                {
                    smart_contracts::emit("Failed: Send all time not reached.".to_string());
                }

            }

        }
    }

    #[wasmedge_bindgen]
    pub fn refund(txn_hash: String, option: String)
    {
    unsafe{

        let sc_wallet_ = smart_contracts::called_smart_contract_wallet();
        let sc_wallet = sc_wallet_.clone();

        if sc_wallet != PROXY_WALLET.to_string() {
            let emit1 = format!("Failed: Unauthorized sender key: {}", sc_wallet.clone());
            smart_contracts::emit(emit1.clone());
            return;
        }
        
        let wallet_address = smart_contracts::wallet_address();

        let key_time = format!("{}{}", SWAP_TIME_KEY.to_string(), txn_hash.clone());
        let key_amount_minted = format!("{}{}", REFUND_AMOUNT_KEY.to_string(), txn_hash.clone());
        let key_amount_sent = format!("{}{}", REFUND_SENT_KEY.to_string(), txn_hash.clone());
        let key_token_sent = format!("{}{}", REFUND_TOKEN_SENT_KEY.to_string(), txn_hash.clone());
        let key_wallet_address = format!("{}{}", REFUND_WALLET_ADDRESS_KEY.to_string(), txn_hash.clone());
        let key_current_state = format!("{}{}", REFUND_STATE_KEY.to_string(), txn_hash.clone());
        let key_refund_time = format!("{}{}", REFUND_TIME_KEY.to_string(), txn_hash.clone());

        let state_wallet_address = smart_contracts::retrieve_state(key_wallet_address.clone());
        
        if state_wallet_address != wallet_address {
            let emit1 = format!("Failed: Unauthorized wallet: {}", wallet_address.clone());
            smart_contracts::emit(emit1.clone());
            return;
        }

        let state_time = smart_contracts::retrieve_state(key_time.clone());
        let amount_minted = smart_contracts::retrieve_state(key_amount_minted.clone());
        let amount_sent = smart_contracts::retrieve_state(key_amount_sent.clone());
        let token_sent = smart_contracts::retrieve_state(key_token_sent.clone());
        let current_state = smart_contracts::retrieve_state(key_current_state.clone());
        
        if state_time.is_empty() || amount_minted.is_empty() || amount_sent.is_empty() || token_sent.is_empty() || current_state.is_empty() {
            let emit2 = "Failed: No refund data found.".to_string();
            smart_contracts::emit(emit2.clone());
            return;
        }

        let mint_amount = types::string_to_u256(amount_minted.clone());
        let sent_amount = types::string_to_u256(amount_sent.clone());
        let current_time = smart_contracts::last_block_time();

        let mint_time = match state_time.parse::<u64>() {
            Ok(value) => value,
            Err(e) => {
                smart_contracts::emit(format!("Failed: Error parsing string to u64: {}", e));
                return; // Handle the error appropriately
            }
        };

        let time_elapsed = current_time - mint_time;
        let key_refund_later = format!("{}{}", REFUND_LATER_KEY.to_string(), txn_hash.clone());

        if option == OPTION_A{ 
             if current_state == START_STATE {
                
                let periods = time_elapsed / REFUND_PERIOD_WEEKS; // one period per 604800 seconds
                let percent_drop = periods * 7; // 7% drop per week
                let percent = 80 - percent_drop; // 80% - 7% per week

                let refund_amount = sent_amount * percent / 100;
                let refund_later = sent_amount - refund_amount;

                if refund_amount > U256::zero() {
                
                if !smart_contracts::transfer(ZRA_CONTRACT.to_string(), amount_minted.clone(), PROXY_WALLET.to_string()){
                    let emit3 = "Failed: Hold.".to_string();
                    smart_contracts::emit(emit3.clone());
                    return;
                }

                if !smart_contracts::delegate_send(token_sent.clone(), refund_amount.to_string(), wallet_address.clone(), PROXY_WALLET.to_string()) { 

                    smart_contracts::delegate_send(ZRA_CONTRACT.to_string(), amount_minted.clone(), wallet_address.clone(), PROXY_WALLET.to_string());

                    let emit4 = "Failed: Refund.".to_string();
                    smart_contracts::emit(emit4.clone());
                    return;
                }

                smart_contracts::store_state(key_refund_later.clone(), refund_later.to_string());
                smart_contracts::store_state(key_current_state.clone(), REFUND_STATE.to_string());
                smart_contracts::store_state(key_refund_time.clone(), current_time.to_string());

                let emit7 = "Success: Redemption in process".to_string();
                smart_contracts::emit(emit7.clone());
                return;
                }
            else{
                    smart_contracts::clear_state(key_time.clone());
                    smart_contracts::clear_state(key_amount_minted.clone());
                    smart_contracts::clear_state(key_amount_sent.clone());
                    smart_contracts::clear_state(key_token_sent.clone());
                    smart_contracts::clear_state(key_wallet_address.clone());
                    smart_contracts::clear_state(key_refund_later.clone());
                    smart_contracts::clear_state(key_refund_time.clone());

                    smart_contracts::store_state(key_current_state.clone(), DONE_STATE.to_string());
                    let emit7 = "Failed: Past Refund duration.".to_string();
                    smart_contracts::emit(emit7.clone());
                    return;
                }
             }
            else if current_state == REFUND_STATE {
                let refund_time_state = smart_contracts::retrieve_state(key_refund_time.clone());

                let refund_time = match refund_time_state.parse::<u64>() {
                    Ok(value) => value,
                    Err(e) => {
                        smart_contracts::emit(format!("Failed: Error parsing string to u64: {}", e));
                        return; // Handle the error appropriately
                    }
                };

                let refund_time_elapsed = current_time - refund_time;
                if time_elapsed < REFUND_PERIOD as u64 {         //3 months
                    let emit5 = "Failed: Redemption period not reached.".to_string();
                    smart_contracts::emit(emit5.clone());
                    return;
                }

                let refund_later = smart_contracts::retrieve_state(key_refund_later.clone());
                if refund_later.is_empty() {
                    let emit5 = "Failed: Invalid state data.".to_string();
                    smart_contracts::emit(emit5.clone());
                    return;
                }

                if !smart_contracts::delegate_send(token_sent.clone(), refund_later.clone(), wallet_address.clone(), PROXY_WALLET.to_string()) {
                    let emit6 = "Failed: Refund.".to_string();
                    smart_contracts::emit(emit6.clone());
                    return;
                }

                smart_contracts::clear_state(key_time.clone());
                smart_contracts::clear_state(key_amount_minted.clone());
                smart_contracts::clear_state(key_amount_sent.clone());
                smart_contracts::clear_state(key_token_sent.clone());
                smart_contracts::clear_state(key_wallet_address.clone());
                smart_contracts::clear_state(key_refund_later.clone());
                smart_contracts::clear_state(key_refund_time.clone());

                smart_contracts::store_state(key_current_state.clone(), DONE_STATE.to_string());
            }
        }
        else if option == OPTION_B {
            if current_state == START_STATE {

                let periods = time_elapsed / REFUND_PERIOD_WEEKS; // one period per 604800 seconds
                let percent_drop = periods * 5; // 5% drop per week
                let percent = 90 - percent_drop; // 90% - 5% per week

                let refund_amount = (sent_amount * percent) / 100;
                let treasury_amount = sent_amount - refund_amount;

                if refund_amount > U256::zero() {

                if !smart_contracts::transfer(ZRA_CONTRACT.to_string(), amount_minted.clone(), PROXY_WALLET.to_string()){
                    let emit3 = "Failed: Hold.".to_string();
                    smart_contracts::emit(emit3.clone());
                    return;
                }

                if !smart_contracts::delegate_send(token_sent.clone(), refund_amount.to_string(), wallet_address.clone(), PROXY_WALLET.to_string()) {

                    smart_contracts::delegate_send(ZRA_CONTRACT.to_string(), amount_minted.clone(), wallet_address.clone(), PROXY_WALLET.to_string());

                    let emit4 = "Failed: Refund.".to_string();
                    smart_contracts::emit(emit4.clone());
                    return;
                }
                }
                
                smart_contracts::clear_state(key_time.clone()); 
                smart_contracts::clear_state(key_amount_minted.clone());
                smart_contracts::clear_state(key_amount_sent.clone());
                smart_contracts::clear_state(key_token_sent.clone());
                smart_contracts::clear_state(key_wallet_address.clone());
                smart_contracts::clear_state(key_refund_later.clone());

                smart_contracts::store_state(key_current_state.clone(), DONE_STATE.to_string());

                let emit7 = "Success: ".to_string();
                smart_contracts::emit(emit7.clone());

                smart_contracts::delegate_send(ZRA_CONTRACT.to_string(), amount_minted.clone(), TREASURY_WALLET.to_string(), PROXY_WALLET.to_string());
                smart_contracts::delegate_send(token_sent.to_string(), treasury_amount.to_string(), TREASURY_WALLET.to_string(), PROXY_WALLET.to_string());
            }
            else{
                let emit5 = "Failed: Invalid state data.".to_string();
                smart_contracts::emit(emit5.clone());
                return;
            }
        }
        else {
            let emit8 = "Failed: Invalid option.".to_string();
            smart_contracts::emit(emit8.clone());
            return;
        }
    }

    }

    #[wasmedge_bindgen]
    pub fn treasury_send()
    {
        unsafe{

        let mut txns_string = smart_contracts::retrieve_state(ALL_TXNS_KEY.to_string());
        
        if txns_string.is_empty(){
            return;
        }

        txns_string.pop();
        let txns: Vec<&str> = txns_string.split(',').collect();
        let mut new_txns: Vec<&str> = Vec::new();

        for txn in txns{
            let hash = txn.clone();
            let key_current_state = format!("{}{}", REFUND_STATE_KEY.to_string(), hash.clone());
            let current_state = smart_contracts::retrieve_state(key_current_state.clone());

            if current_state == START_STATE{
                let hash1 = txn.clone();
                let key_time = smart_contracts::retrieve_state(format!("{}{}", SWAP_TIME_KEY.to_string(), hash1.clone()));

                let parsed_value = match key_time.parse::<u64>() {
                    Ok(value) => value,
                    Err(e) => {
                        return; // Handle the error appropriately
                    }
                };

                let current_time = smart_contracts::last_block_time();
                let time_elapsed = current_time - parsed_value;

                if time_elapsed >= MAX_REFUND_PERIOD
                {
                    let key_amount_minted = format!("{}{}", REFUND_AMOUNT_KEY.to_string(), hash.clone());
                    let key_amount_sent = format!("{}{}", REFUND_SENT_KEY.to_string(), hash.clone());
                    let key_token_sent = format!("{}{}", REFUND_TOKEN_SENT_KEY.to_string(), hash.clone());
                    let key_wallet_address = format!("{}{}", REFUND_WALLET_ADDRESS_KEY.to_string(), hash.clone());
                    let key_current_state = format!("{}{}", REFUND_STATE_KEY.to_string(), hash.clone());

                    let amount_sent = smart_contracts::retrieve_state(key_amount_sent.clone());
                    let token_sent = smart_contracts::retrieve_state(key_token_sent.clone());

                    smart_contracts::delegate_send(token_sent.clone(), amount_sent.clone(), TREASURY_WALLET.to_string(), PROXY_WALLET.to_string());

                    smart_contracts::clear_state(key_time.clone());
                    smart_contracts::clear_state(key_amount_minted.clone());
                    smart_contracts::clear_state(key_amount_sent.clone());
                    smart_contracts::clear_state(key_token_sent.clone());
                    smart_contracts::clear_state(key_wallet_address.clone());
                    smart_contracts::clear_state(key_current_state.clone());    
                }
                else
                {
                    let hash2 = txn.clone();
                    new_txns.push(hash2);
                }

            }
            else if current_state == DONE_STATE
            {
                let key_current_state = format!("{}{}", REFUND_STATE_KEY.to_string(), txn.clone());
                smart_contracts::clear_state(key_current_state.clone()); 
            }
        }
        let mut new_txns_string = "".to_string();

        if !new_txns.is_empty()
        {
            new_txns_string = new_txns.join(",");

            if !new_txns_string.is_empty(){
                new_txns_string = new_txns_string + ",";
            }
        }

        if !new_txns_string.is_empty()
        {
            smart_contracts::store_state(ALL_TXNS_KEY.to_string(), new_txns_string.clone());
        }
        else
        {
            smart_contracts::clear_state(ALL_TXNS_KEY.to_string());
        }
        }
    }

    #[wasmedge_bindgen]
    pub fn burn(){
        unsafe{

            let sc_wallet_ = smart_contracts::called_smart_contract_wallet();
            let sc_wallet = sc_wallet_.clone();

            if sc_wallet != PROXY_WALLET.to_string() {
                let emit1 = format!("Failed: Unauthorized sender key: {}", sc_wallet.clone());
                smart_contracts::emit(emit1.clone());
                return;
            }

            let precision_scale = U256::from(1_000_000_000_000_000_000u64); // Scale by 10^18 for higher precision
            let (burn_scale, amount_to_burn, months_burned) = calculate_burn( precision_scale);

            if amount_to_burn > U256::zero()
            {
                let mint_amount: U256 = U256::zero();
                let burn_str = amount_to_burn.to_string();

                if(!smart_contracts::delegate_mint(ZRA_CONTRACT.to_string(), burn_str.clone(), BURN_WALLET.to_string(), PROXY_WALLET.to_string()))
                {
                    return;
                }

                let state_burn_time_ = smart_contracts::retrieve_state(BURN_TIME_KEY.to_string());
                let state_burn_time = state_burn_time_.clone();
                let mut burn_time = 0 as u64;
        
                burn_time = match state_burn_time.parse::<u64>() {
                    Ok(time) => time,
                    Err(e) => {
                        smart_contracts::emit(format!("Failed: Error parsing state_starting_time: {}", e));
                        return; // or handle the error appropriately
                    }
                };
            
                burn_time = burn_time + (months_burned * 2_592_000); // 2.592M seconds in a month

                smart_contracts::store_state(BURN_TIME_KEY.to_string(), burn_time.to_string());
                update_total_burned(amount_to_burn);
                update_unallocated_supply(mint_amount, amount_to_burn);

            }

        } 
    }

    #[wasmedge_bindgen]
    pub fn allowed_mint(amount: String)
    {
        unsafe{
            let pub_key = smart_contracts::public_key();

            if pub_key != "gov_$ZRA+0000" {
                return;
            }

            smart_contracts::store_state(MINT_SELF_KEY.to_string(), amount.clone());
            smart_contracts::emit("Success: Minting allowed.".to_string());
        }
    }

    #[wasmedge_bindgen]
    pub fn pause_burn()
    {
        unsafe{
            let pub_key = smart_contracts::public_key();

            if pub_key != "gov_$ZRA+0000" {
                return;
            }

            smart_contracts::store_state(PAUSE_BURN_KEY.to_string(), "1".to_string());
            smart_contracts::emit("Success: Burn is paused.".to_string());
        }
    }

    #[wasmedge_bindgen]
    pub fn resume_burn()
    {
        unsafe{
            let pub_key = smart_contracts::public_key();

            if pub_key != "gov_$ZRA+0000" {
                return;
            }

            smart_contracts::clear_state(PAUSE_BURN_KEY.to_string());
            let current_time = smart_contracts::last_block_time();
            smart_contracts::store_state(BURN_TIME_KEY.to_string(), current_time.to_string());
            smart_contracts::emit("Success: Burn will resume.".to_string());
        }
    }

}