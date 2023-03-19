/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// use std::fmt::Display;



// use ironfish_rust::keys::Language;
// use ironfish_rust::PublicAddress;
use ironfish_rust::SaplingKey;
// use ironfish_rust::serializing::{bytes_to_hex, hex_to_bytes};

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

use jni::objects::JString;
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

// #[no_mangle]
// pub unsafe extern "C" fn Java_pers_metaworm_RustTest_generateKey(
//     env: JNIEnv,
//     _class: JClass
// ) -> jstring {
//     let key = SaplingKey::generate_key();

//     // let spending_key = env.new_string(key.hex_spending_key()).unwrap();
//     // let spending_key = env.new_string(key.view_key().hex_key()).unwrap();
//     let spending_key = env.new_string(key.incoming_view_key().hex_key()).unwrap();
//     // let spending_key = env.new_string(key.outgoing_view_key().hex_key()).unwrap();
//     // let spending_key = env.new_string(key.public_address().hex_public_address()).unwrap();
//     // spending_key
//     // spending_key.drop_in_place()
//     //  spending_key.into_inner()
//     spending_key.into_raw()
// }


#[no_mangle]
pub unsafe extern "C" fn Java_com_IronFishNativeApi_generateKey(
    mut env: JNIEnv,
    _class: JClass
) -> jobject {
    let key = SaplingKey::generate_key();
    let income_view_key = env.new_string(key.incoming_view_key().hex_key()).unwrap();
    let out_view_key = env.new_string(key.outgoing_view_key().hex_key()).unwrap();

    let ironkey_class = env.find_class("com/IronFishNativeApi$IronKey").expect("cant find class");
    let iron_object  = &env.alloc_object(ironkey_class).expect("cant alloc IronKey object");

    let view_key_object =  JObject::from(out_view_key);
    let spent_key_object =  JObject::from(income_view_key);

    env.set_field(iron_object, "viewKey", "Ljava/lang/String;", JValue::Object(&view_key_object)).expect("cant find viewKey type");
    env.set_field(iron_object, "spentKey", "Ljava/lang/String;", JValue::Object(&spent_key_object)).expect("cnat find spentKey type");

    iron_object.as_raw()
}


#[no_mangle]
pub unsafe extern "C" fn Java_com_IronFishNativeApi_generateKeyBySeed(
    mut env: JNIEnv,
    _class: JClass,
    _hex_key: JString
) -> jobject {
    let seed = env.get_string(&_hex_key).expect("cant find hex key");
    let key = SaplingKey::from_hex(&String::from(seed)).expect("cant find seed");
    let income_view_key = env.new_string(key.incoming_view_key().hex_key()).unwrap();
    let out_view_key = env.new_string(key.outgoing_view_key().hex_key()).unwrap();

    let ironkey_class = env.find_class("com/IronFishNativeApi$IronKey").expect("cant find class");
    let iron_object  = &env.alloc_object(ironkey_class).expect("cant alloc IronKey object");

    let view_key_object =  JObject::from(out_view_key);
    let spent_key_object =  JObject::from(income_view_key);

    env.set_field(iron_object, "viewKey", "Ljava/lang/String;", JValue::Object(&view_key_object)).expect("cant find viewKey type");
    env.set_field(iron_object, "spentKey", "Ljava/lang/String;", JValue::Object(&spent_key_object)).expect("cnat find spentKey type");

    iron_object.as_raw()
}



// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn Java_pers_metaworm_RustTest_generateKeyV2(
//     env: JNIEnv,
//     _class: JClass
// ) -> jobject {
//     let key = SaplingKey::generate_key();

//     let income_view_key = env.new_string(key.incoming_view_key().hex_key()).unwrap();
//     let out_view_key = env.new_string(key.outgoing_view_key().hex_key()).unwrap();


//     let ironkey_class = env.find_class("pers/metaworm/RustTest$IronKey").expect("cant find class");
//     let iron_Object  = env.alloc_object(ironkey_class).expect("alloc IronKey object");

//     let spentKey_Object =  JObject::from(income_view_key);
//     let viewKey_Object =  JObject::from(out_view_key);

//    let test =  JValueGen::Object(spentKey_Object).as_jni();
//     env.set_field(iron_Object, "spentKey", "Ljava/lang/String;", JValueGen::Object(spentKey_Object).as_jni()).expect("find spentKye type");
//     env.set_field(iron_Object, "viewKey", "Ljava/lang/String;", JValue::Object(viewKey_Object)).expect("find viewKey type");


//    *iron_Object
// }


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
