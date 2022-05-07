/// Tạo một contract có thể  Read, Update, Delete đơn giản bằng Rust

/// 0. Import một số module sử dụng use declaration 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// set up global allocator để có thể sử dụng bộ nhớ từ hệ thống trong khi runtime 
near_sdk::setup_alloc!();

/// 1. Main Struct

// Khi thêm macro #[near_bindgen], ta tạo ra một bản mẫu của Line Struct khiến nó phù hợp với NEAR blockchain 
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Line {
    val: String,
}

/// 2. Core Logic

#[near_bindgen]
impl Line {
    //thay đổi giá trị của val
    pub fn change_string(&mut self, s: String) {
        env::log(b"changed");
        self.val = s.clone();
    }

    //đọc giá trị từ val
    pub fn read(&self) -> String {
        env::log(b"read");
        return self.val.clone();
    }

    //xóa val
    pub fn delete(&mut self) {
        env::log(b"delete");
        self.val = "".to_string();
    }
}

/// 4. Test the contract

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {

    // setup môi trường testing và mocked blockchain
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "thang.testnet".to_string(),
            signer_account_id: "duc.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // Test 1: Thay đổi giá trị của val
    #[test]
    fn create_read_line() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Line {val: "HelloWorld".to_string()};
        println!("The data of line is {}", contract.read());
        contract.change_string("Hi".to_string());
        println!("The data of line is {}", contract.read());
    }

    // Test 2: Xoa gia tri cua val
    #[test]
    fn read_nonexistent_pair() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Line {val: "HelloWorld".to_string()};
        println!("The data of line is {}", contract.read());
        contract.delete();
        println!("The data of line is {}", contract.read());
    }
}