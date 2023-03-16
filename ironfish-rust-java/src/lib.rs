/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use std::fmt::Display;

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
use jni::sys::{jstring};
use jni::JNIEnv;
use jni::objects::{JClass, JString};

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

    let string = env.new_string(key.hex_spending_key()).unwrap();


    // let spedding = key.hex_spending_key();
    // spedding.as_bytes();
    // let result: JString = env.new_string(spedding.as_bytes());
    string.into_raw()
// Then we have to create a new java string to return. Again, more info
// in the `strings` module.
    // let output = env.new_string(format!("Hello, {}!", input))
    // .expect("Couldn't create java string!");
    // // output.into_inner() 
    // return output.read();
    // return env->NewStringUTF(key.hex_spending_key());
}


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
