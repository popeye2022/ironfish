/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use std::fmt::Display;


use std::ops::Deref;

// use ironfish_rust::keys::Language;
// use ironfish_rust::PublicAddress;
use ironfish_rust::SaplingKey;
// use napi::bindgen_prelude::*;
// use napi_derive::napi;

// use ironfish_rust::mining;
// use ironfish_rust::sapling_bls12;

// pub mod mpc;
// pub mod nacl;
// pub mod rolling_filter;
// pub mod signal_catcher;
// pub mod structs;

// fn to_napi_err(err: impl Display) -> napi::Error {
//     Error::from_reason(err.to_string())
// }

// use jni::objects::*;
// use jni::signature::JavaType;
use jni::sys::*;
use jni::JNIEnv;
use jni::objects::{JClass,JValue,JObject};

// static CLASS_MATH: &str = "java/lang/Math";
// static CLASS_STRING: &str = "java/lang/String";
// static CLASS_OBJECT: &str = "java/lang/Object";
// static CLASS_LOCAL_DATE_TIME: &str = "java/time/LocalDateTime";
// static METHOD_MATH_ABS: &str = "abs";
// static METHOD_OBJECT_HASH_CODE: &str = "hashCode";
// static METHOD_CTOR: &str = "<init>";
// static METHOD_LOCAL_DATE_TIME_OF: &str = "of";
// static SIG_OBJECT_CTOR: &str = "()V";
// static SIG_MATH_ABS: &str = "(I)I";
// static SIG_OBJECT_HASH_CODE: &str = "()I";
// static SIG_LOCAL_DATE_TIME_OF: &str = "(IIIIIII)Ljava/time/LocalDateTime;";
// static TESTING_OBJECT_STR: &str = "TESTING OBJECT";

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustTest_generateKey(
    env: JNIEnv,
    _class: JClass
) -> jstring {
    let key = SaplingKey::generate_key();

    // let spending_key = env.new_string(key.hex_spending_key()).unwrap();
    // let spending_key = env.new_string(key.view_key().hex_key()).unwrap();
    let spending_key = env.new_string(key.incoming_view_key().hex_key()).unwrap();
    // let spending_key = env.new_string(key.outgoing_view_key().hex_key()).unwrap();
    // let spending_key = env.new_string(key.public_address().hex_public_address()).unwrap();
    // spending_key
    // spending_key.drop_in_place()
     spending_key.into_inner()

}


#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_pers_metaworm_RustTest_generateKeyV2(
    env: JNIEnv,
    _class: JClass
) -> jobject {
    let key = SaplingKey::generate_key();

    let income_view_key = env.new_string(key.incoming_view_key().hex_key()).unwrap();
    let out_view_key = env.new_string(key.outgoing_view_key().hex_key()).unwrap();


    let ironkey_class = env.find_class("pers/metaworm/RustTest$IronKey").expect("cant find class");
    let iron_Object  = env.alloc_object(ironkey_class).expect("alloc IronKey object");

    let spentKey_Object =  JObject::from(income_view_key);
    let viewKey_Object =  JObject::from(out_view_key);

    env.set_field(iron_Object, "spentKey", "Ljava/lang/String;", JValue::Object(spentKey_Object)).expect("find spentKye type");
    env.set_field(iron_Object, "viewKey", "Ljava/lang/String;", JValue::Object(viewKey_Object)).expect("find viewKey type");


   *iron_Object
}


// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_isContainWallet(env: JNIEnv, _: JClass) -> jobject {
//     //Call to get all wallets and check the return value
//     let state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("can't found NativeLib$WalletState class");
//     let state_obj = env.alloc_object(state_class).expect("create state_obj instance ");
//     // let wallet = module::wallet::WalletManager {};
//     // match wallet.is_contain_wallet() {
//     //     Ok(data) => {
//     //         // env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("find status type ");
//     //         env.set_field(state_obj, "isContainWallet", "Z", JValue::Bool(data as u8)).expect("set isContainWallet value");
//     //     }
//     //     Err(e) => {
//     //         env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("find status type ");
//     //         env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(e.to_string()).unwrap()))).expect("isContainWallet set message ");
//     //     }
//     // }
//     *state_obj
// }



// unfortunately napi doesn't support reexport of enums (bip39::Language) so we
// have to recreate if we want type safety. hopefully in the future this will work with napi:
// #[napi]
// pub use bip39::Language as Language;
// https://github.com/napi-rs/napi-rs/issues/1463
// #[napi]
// pub enum LanguageCode {
//     English,
//     ChineseSimplified,
//     ChineseTraditional,
//     French,
//     Italian,
//     Japanese,
//     Korean,
//     Spanish,
// }
// impl From<LanguageCode> for Language {
//     fn from(item: LanguageCode) -> Self {
//         match item {
//             LanguageCode::English => Language::English,
//             LanguageCode::ChineseSimplified => Language::ChineseSimplified,
//             LanguageCode::ChineseTraditional => Language::ChineseTraditional,
//             LanguageCode::French => Language::French,
//             LanguageCode::Italian => Language::Italian,
//             LanguageCode::Japanese => Language::Japanese,
//             LanguageCode::Korean => Language::Korean,
//             LanguageCode::Spanish => Language::Spanish,
//         }
//     }
// }

// #[napi(object)]
// pub struct Key {
//     pub spending_key: String,
//     pub view_key: String,
//     pub incoming_view_key: String,
//     pub outgoing_view_key: String,
//     pub public_address: String,
// }

// #[napi]
// pub fn generate_key() -> Key {
//     let sapling_key = SaplingKey::generate_key();

//     Key {
//         spending_key: sapling_key.hex_spending_key(),
//         view_key: sapling_key.view_key().hex_key(),
//         incoming_view_key: sapling_key.incoming_view_key().hex_key(),
//         outgoing_view_key: sapling_key.outgoing_view_key().hex_key(),
//         public_address: sapling_key.public_address().hex_public_address(),
//     }
// }

// #[napi]
// pub fn spending_key_to_words(private_key: String, language_code: LanguageCode) -> Result<String> {
//     let key = SaplingKey::from_hex(&private_key).map_err(to_napi_err)?;
//     let mnemonic = key.to_words(language_code.into()).map_err(to_napi_err)?;
//     Ok(mnemonic.into_phrase())
// }

// #[napi]
// pub fn words_to_spending_key(words: String, language_code: LanguageCode) -> Result<String> {
//     let key = SaplingKey::from_words(words, language_code.into()).map_err(to_napi_err)?;
//     Ok(key.hex_spending_key())
// }

// #[napi]
// pub fn generate_key_from_private_key(private_key: String) -> Result<Key> {
//     let sapling_key = SaplingKey::from_hex(&private_key).map_err(to_napi_err)?;

//     Ok(Key {
//         spending_key: sapling_key.hex_spending_key(),
//         view_key: sapling_key.view_key().hex_key(),
//         incoming_view_key: sapling_key.incoming_view_key().hex_key(),
//         outgoing_view_key: sapling_key.outgoing_view_key().hex_key(),
//         public_address: sapling_key.public_address().hex_public_address(),
//     })
// }

// #[napi]
// pub fn initialize_sapling() {
//     let _ = sapling_bls12::SAPLING.clone();
// }

// #[napi(constructor)]
// pub struct FoundBlockResult {
//     pub randomness: String,
//     pub mining_request_id: f64,
// }

// #[napi]
// pub struct ThreadPoolHandler {
//     threadpool: mining::threadpool::ThreadPool,
// }
// #[napi]
// impl ThreadPoolHandler {
//     #[napi(constructor)]
//     pub fn new(thread_count: u32, batch_size: u32, pause_on_success: bool) -> Self {
//         ThreadPoolHandler {
//             threadpool: mining::threadpool::ThreadPool::new(
//                 thread_count as usize,
//                 batch_size,
//                 pause_on_success,
//             ),
//         }
//     }

//     #[napi]
//     pub fn new_work(&mut self, header_bytes: Buffer, target: Buffer, mining_request_id: u32) {
//         self.threadpool
//             .new_work(&header_bytes, &target, mining_request_id)
//     }

//     #[napi]
//     pub fn stop(&self) {
//         self.threadpool.stop()
//     }

//     #[napi]
//     pub fn pause(&self) {
//         self.threadpool.pause()
//     }

//     #[napi]
//     pub fn get_found_block(&self) -> Option<FoundBlockResult> {
//         if let Some(result) = self.threadpool.get_found_block() {
//             return Some(FoundBlockResult {
//                 randomness: format!("{:016x}", result.0),
//                 mining_request_id: result.1 as f64,
//             });
//         }
//         None
//     }

//     #[napi]
//     pub fn get_hash_rate_submission(&self) -> u32 {
//         self.threadpool.get_hash_rate_submission()
//     }
// }

// #[napi]
// pub fn is_valid_public_address(hex_address: String) -> bool {
//     PublicAddress::from_hex(&hex_address).is_ok()
// }
