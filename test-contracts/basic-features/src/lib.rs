
#![no_std]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::redundant_clone)]

imports!();

mod ser_ex1;
mod ser_ex2;

use ser_ex1::*;
use ser_ex2::*;

#[elrond_wasm_derive::contract(BasicFeaturesImpl)]
pub trait BasicFeatures {

    #[init]
    fn init(&self) {
    }

    #[endpoint(panicWithMessage)]
    fn panic_with_message(&self) {
        panic!("example panic message");
    }

    // TEST ARGUMENT AND RETURN TYPE SERIALIZATION

    #[endpoint]
    fn echo_big_uint(&self, bi: BigUint) -> BigUint {
        bi
    }

    #[endpoint]
    fn echo_big_int(&self, bi: BigInt) -> BigInt {
        bi
    }

    #[endpoint]
    fn echo_u64(&self, i: u64) -> u64 {
        i
    }

    #[endpoint]
    fn echo_i64(&self, i: i64) -> i64 {
        i
    }

    #[endpoint]
    fn echo_i32(&self, i: i32) -> i32 {
        i
    }

    #[endpoint]
    fn echo_u32(&self, i: u32) -> u32 {
        i
    }

    #[endpoint]
    fn echo_isize(&self, i: isize) -> isize {
        i
    }

    #[endpoint]
    fn echo_usize(&self, i: usize) -> usize {
        i
    }
    
    #[endpoint]
    fn echo_i8(&self, i: i8) -> i8 {
        i
    }

    #[endpoint]
    fn echo_u8(&self, i: u8) -> u8 {
        i
    }

    #[endpoint]
    fn echo_bool(&self, i: bool) -> bool {
        i
    }

    #[endpoint]
    fn echo_opt_bool(&self, i: Option<bool>) -> Option<bool> {
        i
    }

    #[endpoint]
    fn echo_nothing(&self, #[var_args] nothing: ()) -> () {
        nothing
    }

    #[endpoint]
    fn echo_array_u8(&self, s: [u8; 5]) -> [u8; 5] {
        s
    }

    #[endpoint]
    fn echo_vec_u8(&self, arg: Vec<u8>) -> MultiResult2<Vec<u8>, i64> {
        let l = arg.len() as i64;
        (arg, l).into()
    }

    #[endpoint]
    fn echo_multi_1(&self, _n: usize, #[multi(_n)] m: VarArgs<i32>, another_arg: u64) -> MultiResult2<MultiResultVec<i32>, u64> {
        (m.into_vec().into(), another_arg).into()
    }

    #[endpoint]
    fn echo_multi_vec_u8(&self, _n: usize, #[multi(_n)] m: VarArgs<Vec<u8>>) -> MultiResultVec<Vec<u8>> {
        m.into_vec().into()
    }

    #[endpoint]
    fn echo_multi_h256(&self, _n: usize, #[multi(_n)] m: VarArgs<H256>) -> MultiResultVec<H256> {
        m.into_vec().into()
    }

    #[endpoint]
    fn echo_varags_u32(&self, #[var_args] m: VarArgs<u32>) -> MultiResult2<usize, MultiResultVec<u32>> {
        let v = m.into_vec();
        (v.len(), v.into()).into()
    }

    #[endpoint]
    fn echo_varags_tuples(&self, #[var_args] m: VarArgs<MultiArg2<isize, Vec<u8>>>) -> MultiResultVec<MultiResult2<isize, Vec<u8>>> {
        let mut result: Vec<MultiResult2<isize, Vec<u8>>> = Vec::new();
        for m_arg in m.into_vec().into_iter() {
            result.push( m_arg.into_tuple().into())
        }
        result.into()
    }

    #[endpoint]
    fn echo_async_result_empty(&self, #[var_args] a: AsyncCallResult<()>) -> SCResult<()> {
        match a {
            AsyncCallResult::Ok(()) => Ok(()),
            AsyncCallResult::Err(msg) => Err(SCError::Dynamic(msg.err_msg)),
        }
    }

    #[endpoint]
    fn echo_ser_example_1(&self, se: SerExample1) -> SerExample1 {
        se
    }

    // OPERATIONS THAT HAVE CAUSED ISSUES IN THE PAST

    #[endpoint]
    fn count_ones(&self, arg: u64) -> u32 {
        arg.count_ones()
    }

    // STORAGE STORE

    #[endpoint]
    #[storage_set("big_uint")]
    fn store_big_uint(&self, bi: BigUint);

    #[endpoint]
    #[storage_set("big_int")]
    fn store_big_int(&self, bi: BigInt);

    #[endpoint]
    #[storage_set("usize")]
    fn store_usize(&self, i: usize);

    #[endpoint]
    #[storage_set("i64")]
    fn store_i64(&self, i: i64);

    #[endpoint]
    #[storage_set("bool")]
    fn store_bool(&self, i: bool);

    #[endpoint]
    #[storage_set("vec_u8")]
    fn store_vec_u8(&self, arg: Vec<u8>);

    #[endpoint]
    #[storage_set("addr")]
    fn store_addr(&self, arg: Address);

    #[storage_set("opt_addr")]
    fn _set_opt_addr(&self, opt_addr: Option<Address>);

    #[endpoint]
    fn store_opt_addr(&self, #[var_args] opt_addr: OptionalArg<Address>) {
        self._set_opt_addr(opt_addr.into_option());
    }

    #[endpoint]
    #[storage_set("ser_1")]
    fn store_ser_1(&self, arg: SerExample1);

    #[endpoint]
    #[storage_set("ser_2")]
    fn store_ser_2(&self, arg: SerExample2);

    #[endpoint]
    #[storage_set("map1")]
    fn store_map1(&self, addr: Address, bi: BigUint);

    #[endpoint]
    #[storage_set("map2")]
    fn store_map2(&self, addr1: &Address, addr2: &Address, bi: &BigUint);

    #[endpoint]
    #[storage_set("map3")]
    fn store_map3(&self, x: usize, b: bool);

    #[storage_set("slice1")]
    fn store_slice1(&self, slice: &[BigUint]);

    #[endpoint]
    #[storage_set("ELRONDi64")]
    fn store_reserved_i64(&self, i: i64);

    #[endpoint]
    #[storage_set("ELRONDBigUint")]
    fn store_reserved_big_uint(&self, i: BigUint);

    #[endpoint]
    #[storage_set("ELRONDreserved")]
    fn store_reserved_vec_u8(&self, i: Vec<u8>);

    // STORAGE LOAD

    #[endpoint]
    #[storage_get("big_uint")]
    fn load_big_uint(&self) -> BigUint;

    #[endpoint]
    #[storage_get("big_int")]
    fn load_big_int(&self) -> BigInt;

    #[endpoint]
    #[storage_get("usize")]
    fn load_usize(&self) -> usize;

    #[endpoint]
    #[storage_get("i64")]
    fn load_i64(&self) -> i64;

    #[endpoint]
    #[storage_get("bool")]
    fn load_bool(&self) -> bool;

    #[endpoint]
    #[storage_get("vec_u8")]
    fn load_vec_u8(&self) -> Vec<u8>;

    #[endpoint]
    #[storage_get("addr")]
    fn load_addr(&self) -> Address;

    #[storage_get("opt_addr")]
    fn _get_opt_addr(&self) -> Option<Address>;

    #[endpoint]
    fn load_opt_addr(&self) -> OptionalResult<Address> {
        self._get_opt_addr().into()
    }

    #[endpoint]
    #[storage_get("ser_1")]
    fn load_ser_1(&self) -> SerExample1;

    #[endpoint]
    #[storage_get("ser_2")]
    fn load_ser_2(&self) -> SerExample2;

    #[endpoint]
    #[storage_get("map1")]
    fn load_map1(&self, addr: Address) -> BigUint;

    #[endpoint]
    #[storage_get("map2")]
    fn load_map2(&self, addr1: &Address, addr2: &Address) -> BigUint;

    #[endpoint]
    #[storage_get("map3")]
    fn load_map3(&self, x: usize) -> bool;

    // EVENTS

    #[endpoint(logEventA)]
    fn log_event_a(&self, data: &BigUint) {
        self.event_a(data);
    }

    #[endpoint(logEventB)]
    fn log_event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint) {
        self.event_b(arg1, arg2, data);
    }

    // VEC OPERATIONS

    #[view]
    fn vec_concat_const(&self) -> Vec<u8> {
        let mut result = b"part1".to_vec();
        result.extend_from_slice(&[0u8;100][..]);
        result
    }

    // SEND TX

    #[endpoint]
    fn send_tx_endpoint(&self, to: &Address, amount: &BigUint) {
        self.send_tx(to, amount, "");
    }


    #[event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000a")]
    fn event_a(&self, data: &BigUint);

    #[event("0x0123456789abcdef0123456789abcdef0123456789abcdef000000000000000b")]
    fn event_b(&self, arg1: &BigUint, arg2: &Address, data: &BigUint);

    // BIG INT OPERATIONS

    // arithmetic ooperators: + - * / %
    #[endpoint]
    fn add_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a + b }
    #[endpoint]
    fn add_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a + b }
    #[endpoint]
    fn add_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a + b }
    #[endpoint]
    fn add_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a + b }
    #[endpoint]
    fn sub_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a - b }
    #[endpoint]
    fn sub_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a - b }
    #[endpoint]
    fn sub_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a - b }
    #[endpoint]
    fn sub_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a - b }
    #[endpoint]
    fn mul_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a * b }
    #[endpoint]
    fn mul_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a * b }
    #[endpoint]
    fn mul_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a * b }
    #[endpoint]
    fn mul_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a * b }
    #[endpoint]
    fn div_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a / b }
    #[endpoint]
    fn div_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a / b }
    #[endpoint]
    fn div_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a / b }
    #[endpoint]
    fn div_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a / b }
    #[endpoint]
    fn rem_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { a % b }
    #[endpoint]
    fn rem_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { a % b }
    #[endpoint]
    fn rem_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a % b }
    #[endpoint]
    fn rem_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a % b }

    // assign version of all operators above
    #[endpoint]
    fn add_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r += b; r }
    #[endpoint]
    fn add_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r += b; r }
    #[endpoint]
    fn add_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r += b; r }
    #[endpoint]
    fn add_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r += b; r }
    #[endpoint]
    fn sub_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r -= b; r }
    #[endpoint]
    fn sub_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r -= b; r }
    #[endpoint]
    fn sub_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r -= b; r }
    #[endpoint]
    fn sub_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r -= b; r }
    #[endpoint]
    fn mul_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r *= b; r }
    #[endpoint]
    fn mul_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r *= b; r }
    #[endpoint]
    fn mul_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r *= b; r }
    #[endpoint]
    fn mul_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r *= b; r }
    #[endpoint]
    fn div_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r /= b; r }
    #[endpoint]
    fn div_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r /= b; r }
    #[endpoint]
    fn div_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r /= b; r }
    #[endpoint]
    fn div_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r /= b; r }
    #[endpoint]
    fn rem_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt           { let mut r = a.clone(); r %= b; r }
    #[endpoint]
    fn rem_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt     { let mut r = a.clone(); r %= b; r }
    #[endpoint]
    fn rem_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r %= b; r }
    #[endpoint]
    fn rem_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r %= b; r }

    #[endpoint]
    fn bit_and_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a & b }
    #[endpoint]
    fn bit_and_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a & b }
    #[endpoint]
    fn bit_or_big_uint(&self, a: BigUint, b: BigUint) -> BigUint        { a | b }
    #[endpoint]
    fn bit_or_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint  { a | b }
    #[endpoint]
    fn bit_xor_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { a ^ b }
    #[endpoint]
    fn bit_xor_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { a ^ b }

    #[endpoint]
    fn bit_and_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r &= b; r }
    #[endpoint]
    fn bit_and_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r &= b; r }
    #[endpoint]
    fn bit_or_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint        { let mut r = a.clone(); r |= b; r }
    #[endpoint]
    fn bit_or_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint  { let mut r = a.clone(); r |= b; r }
    #[endpoint]
    fn bit_xor_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint       { let mut r = a.clone(); r ^= b; r }
    #[endpoint]
    fn bit_xor_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint { let mut r = a.clone(); r ^= b; r }

    #[endpoint]
    fn shr_big_uint(&self, a: BigUint, b: usize) -> BigUint      { a >> b }
    #[endpoint]
    fn shr_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { a >> b }
    #[endpoint]
    fn shl_big_uint(&self, a: BigUint, b: usize) -> BigUint      { a << b }
    #[endpoint]
    fn shl_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { a << b }

    #[endpoint]
    fn shr_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint      { let mut r = a.clone(); r >>= b; r }
    #[endpoint]
    fn shr_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { let mut r = a.clone(); r >>= b; r }
    #[endpoint]
    fn shl_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint      { let mut r = a.clone(); r <<= b; r }
    #[endpoint]
    fn shl_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint { let mut r = a.clone(); r <<= b; r }

    // CRYPTO FUNCTIONS

    #[endpoint(computeSha256)]
    fn compute_sha256(&self, input: Vec<u8>) -> Vec<u8> {
        self.sha256(&input).as_ref().into()
    }

    #[endpoint(computeKeccak256)]
    fn compute_keccak256(&self, input: Vec<u8>) -> Vec<u8> {
        self.keccak256(&input).as_ref().into()
    }

    // MACROS

    #[view]
    fn require_owner_calls(&self) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "Caller must be owner");
        Ok(())
    }

    #[view]
    fn return_error(&self) -> SCResult<()> {
        sc_error!("return_error")
    }

}
