

use elrond_wasm::Address;
use elrond_wasm::StorageKey;

use crate::big_int_mock::*;
use elrond_wasm::ContractHookApi;
use elrond_wasm::CallableContract;
use elrond_wasm::BigUintApi;

use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

use alloc::boxed::Box;
use alloc::vec::Vec;

use std::collections::HashMap;
use std::fmt;

use core::cell::RefCell;
use alloc::rc::Rc;

const ADDRESS_LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;
const TOPIC_LENGTH: usize = 32;

pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub balance: BigInt,
    pub storage: HashMap<StorageKey, Vec<u8>>,
    pub contract: Option<Box<dyn CallableContract>>,
}

impl fmt::Debug for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccountData {{ address: {:?}, nonce: {}, balance: {:?}, storage: {:?} }}", 
            self.address, 
            self.nonce, 
            self.balance,
            self.storage)
    }
}

pub struct TxData {
    func_name: &'static str,
    new_contract: Option<Box<dyn CallableContract>>,
    args: Vec<Vec<u8>>,
    call_value: BigInt,
    from: Address,
    to: Address,
}

impl fmt::Debug for TxData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxData {{ func: {}, args: {:?}, call_value: {}, from: {:?}, to: {:?} }}", 
            self.func_name, 
            self.args, 
            self.call_value, 
            self.from, 
            self.to)
    }
}

impl TxData {
    pub fn new_create(new_contract: Box<dyn CallableContract>, from: Address, to: Address) -> Self {
        TxData{
            func_name: "init",
            new_contract: Some(new_contract),
            args: Vec::new(),
            call_value: 0.into(),
            from: from,
            to: to,
        }
    }

    pub fn new_call(func_name: &'static str, from: Address, to: Address) -> Self {
        TxData{
            func_name: func_name,
            new_contract: None,
            args: Vec::new(),
            call_value: 0.into(),
            from: from,
            to: to,
        }
    }

    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }
}

#[derive(Debug, Clone)]
pub struct TxResult {
    pub result_status: i32,
    pub result_values: Vec<Vec<u8>>,
}

impl TxResult {
    pub fn empty() -> TxResult {
        TxResult {
            result_status: 0,
            result_values: Vec::new(),
        }
    }
    pub fn print(&self) {
        print!("[{:#?}]\n", self);
    }
}

#[derive(Debug)]
pub struct ArwenMockState {
    current_tx: Option<TxData>,
    current_result: TxResult,
    accounts: HashMap<Address, AccountData>,
}

pub struct ArwenMockRef {
    state_ref: Rc<RefCell<ArwenMockState>>
}

impl Clone for ArwenMockRef {
    fn clone(&self) -> Self {
        ArwenMockRef{ state_ref: Rc::clone(&self.state_ref) }
    }
}

impl ArwenMockState {
    pub fn new() -> ArwenMockRef {
        let state = ArwenMockState{
            current_tx: None,
            current_result: TxResult::empty(),
            accounts: HashMap::new(),
        };
        let state_ref = Rc::new(RefCell::new(state));
        ArwenMockRef{ state_ref: state_ref }
    }

    fn create_account_if_necessary(&mut self, tx: &mut TxData) {
        if let Some(ref tx_contract) = tx.new_contract {
            if self.accounts.contains_key(&tx.to) {
                panic!("Account already exists");
            }
            self.accounts.insert(tx.to.clone(), AccountData{
                address: tx.to.clone(),
                nonce: 0,
                balance: 0.into(),
                storage: HashMap::new(),
                contract: Some(tx_contract.clone_contract()),
            });
        }
    }

    pub fn set_result_status(&mut self, status: i32) {
        self.current_result.result_status = status;
    }
    
    pub fn add_result(&mut self, result: Vec<u8>) {
        self.current_result.result_values.push(result);
    }
    
    fn clear_result(&mut self) {
        self.current_result = TxResult::empty();
    }
    
    fn get_result(&self) -> TxResult {
        self.current_result.clone()
    }
}

impl ArwenMockRef {
    fn get_contract(&self) -> Box<dyn CallableContract> {
        let state = self.state_ref.borrow();
        let tx_ref = &state.current_tx.as_ref().unwrap();
        match state.accounts.get(&tx_ref.to) {
            None => panic!("Account not found"),
            Some(ref account) => {
                match account.contract {
                    None => panic!("Recipient account is not a smart contract"),
                    Some(ref acct_contract) => acct_contract.clone_contract(),
                }
            }
        }
    }

    pub fn execute_tx(&self, mut tx: TxData) -> TxResult {
        {
            let mut state = self.state_ref.borrow_mut();
            state.create_account_if_necessary(&mut tx);    
            state.current_tx = Some(tx);
            state.clear_result();
        }
        
        let func_name = {
            let state = self.state_ref.borrow();
            let tx_ref = &state.current_tx.as_ref().unwrap();
            &tx_ref.func_name.clone()
        };
        
        let contract = self.get_contract();

        // contract call
        // important: state cannot be borrowed at this point
        contract.call(&func_name);
        
        let state = self.state_ref.borrow();
        state.get_result()
    }

    pub fn add_account(&self, acct: AccountData) {
        let mut state = self.state_ref.borrow_mut();
        state.accounts.insert(acct.address.clone(), acct);
    }

    pub fn print_accounts(&self) {
        let state = self.state_ref.borrow();
        print!("{:#?}", state.accounts);
    }
}

impl elrond_wasm::ContractHookApi<RustBigInt> for ArwenMockRef {
    #[inline]
    fn get_owner(&self) -> Address {
        let state = self.state_ref.borrow();
        match &state.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.to.clone(),
        }
    }

    #[inline]
    fn get_caller(&self) -> Address {
        let state = self.state_ref.borrow();
        match &state.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.from.clone(),
        }
    }

    #[inline]
    fn signal_error(&self) {
        panic!("signal_error was called");
    }

    fn signal_exit(&self, exit_code: i32) {
        let mut state = self.state_ref.borrow_mut();
        state.set_result_status(exit_code);
    }

    fn write_log(&self, _topics: &[[u8;32]], _data: &[u8]) {
        print!("write_log not yet implemented\n");
    }

    fn storage_store(&self, key: &StorageKey, value: &Vec<u8>) {
        let sc_address = self.get_owner();
        let mut state = self.state_ref.borrow_mut();
        match state.accounts.get_mut(&sc_address) {
            None => panic!("Account not found!"),
            Some(acct) => {
                acct.storage.insert(key.clone(), value.clone());
            }
        }
    }

    fn storage_load(&self, key: &StorageKey) -> Vec<u8> {
        let state = self.state_ref.borrow();
        match &state.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => {
                match state.accounts.get(&tx.to) {
                    None => panic!("Account not found!"),
                    Some(acct) => {
                        match acct.storage.get(key) {
                            None => Vec::with_capacity(0),
                            Some(value) => {
                                value.clone()
                            },
                        }
                    }
                }
            }
        }
    }

    fn storage_store_bytes32(&self, key: &StorageKey, value: &[u8; 32]) {
        let mut vector = Vec::with_capacity(32);
        for i in value.iter() {
            vector.push(*i);
        }
        self.storage_store(key, &vector);
    }
    
    fn storage_load_bytes32(&self, key: &StorageKey) -> [u8; 32] {
        let value = self.storage_load(key);
        let mut res = [0u8; 32];
        let offset = 32 - value.len();
        for i in 0..value.len()-1 {
            res[offset+i] = value[i];
        }
        res
    }

    fn storage_store_big_int(&self, key: &StorageKey, value: &RustBigInt) {
        self.storage_store(key, &value.to_signed_bytes_be());
    }

    fn storage_load_big_int(&self, key: &StorageKey) -> RustBigInt {
        let value = self.storage_load(key);
        let bi = BigInt::from_signed_bytes_be(value.as_slice());
        bi.into()
    }

    #[inline]
    fn get_call_value_big_int(&self) -> RustBigInt {
        let state = self.state_ref.borrow();
        match &state.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.call_value.clone().into(),
        }
    }

    fn send_tx(&self, to: &Address, amount: &RustBigInt, _message: &str) {
        let owner = self.get_owner();
        let mut state = self.state_ref.borrow_mut();
        match state.accounts.get_mut(&owner) {
            None => panic!("Account not found!"),
            Some(acct) => {
                acct.balance -= amount.value();
            }
        }
        match state.accounts.get_mut(to) {
            None => panic!("Account not found!"),
            Some(acct) => {
                acct.balance += amount.value();
            }
        }
    }

    fn get_gas_left(&self) -> i64 {
        0
    }
}

impl ArwenMockState {
    fn get_argument(&self, arg_index: i32) -> Vec<u8> {
        let arg_idx_usize: usize = arg_index as usize;
        match &self.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => {
                if arg_idx_usize >= tx.args.len() {
                    panic!("Tx arg index out of range");
                }
                tx.args[arg_idx_usize].clone()
            },
        }
    }
}

impl elrond_wasm::ContractIOApi<RustBigInt, RustBigUint> for ArwenMockRef {

    fn check_num_arguments(&self, expected: i32) -> bool {
        let state = self.state_ref.borrow();
        let nr_args = match &state.current_tx {
            None => panic!("Tx not initialized!"),
            Some(tx) => tx.args.len(),
        };
        if nr_args != expected as usize {
            self.signal_error();
            return false;
        }
        return true;
    }

    fn check_not_payable(&self) -> bool {
        if &self.get_call_value_big_int() > &0.into() {
            self.signal_error();
            return false;
        }
        return true;
    }

    #[inline]
    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32] {
        let state = self.state_ref.borrow();
        let arg = state.get_argument(arg_index);
        let mut res = [0u8; 32];
        let offset = 32 - arg.len();
        for i in 0..arg.len()-1 {
            res[offset+i] = arg[i];
        }
        res
    }
    
    #[inline]
    fn get_argument_address(&self, arg_index: i32) -> Address {
        let bytes = self.get_argument_bytes32(arg_index);
        bytes.into()
    }
    
    #[inline]
    fn get_argument_big_int_signed(&self, arg_index: i32) -> RustBigInt {
        let state = self.state_ref.borrow();
        let bytes = state.get_argument(arg_index);
        BigInt::from_signed_bytes_be(&bytes).into()
    }

    #[inline]
    fn get_argument_big_int_unsigned(&self, arg_index: i32) -> RustBigUint {
        let signed = self.get_argument_big_int_signed(arg_index);
        signed.into()
    }

    #[inline]
    fn get_argument_i64(&self, arg_index: i32) -> i64 {
        let state = self.state_ref.borrow();
        let bytes = state.get_argument(arg_index);
        let bi = BigInt::from_signed_bytes_be(&bytes);
        if let Some(v) = bi.to_i64() {
            v
        } else {
            panic!("Argument does not fit in an i64.")
        }
    }

    #[inline]
    fn finish_big_int_signed(&self, bi: RustBigInt) {
        let mut state = self.state_ref.borrow_mut();
        state.add_result(bi.to_signed_bytes_be());
    }

    #[inline]
    fn finish_big_int_unsigned(&self, bu: RustBigUint) {
        self.finish_big_int_signed(bu.into_signed());
    }
    
    #[inline]
    fn finish_i64(&self, value: i64) {
        self.finish_big_int_signed(value.into());
    }
}