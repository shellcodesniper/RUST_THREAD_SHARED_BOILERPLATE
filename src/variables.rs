/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:15:55 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:15:55 
 */

#![allow(dead_code)]
use super::interfaces::permissions::CurrentProgramMode;

use lazy_static::lazy_static;


use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::Deref;

enum WrapIt<'a, T>{
    Read(RwLockReadGuard<'a, T>),
    Write(RwLockWriteGuard<'a, T>)
}

impl<'a, T> Deref for WrapIt<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            WrapIt::Read(r_g) => r_g.deref(),
            WrapIt::Write(w_g) => w_g.deref()
        }
    }
}
// ? 이쪽은 read 랑 write 를 사용하기 위해 구성한 Trait들임
// ? lazy_static! macro 를 이용하여 구성한 global 변수들을 사용할 수 있음



lazy_static! {
  pub static ref GLOBAL_PROGRAM_MODE_LOCK
    : Arc<RwLock<CurrentProgramMode>> = Arc::new(RwLock::new(CurrentProgramMode::NONE));
  // ? 전역변수 - 프로그램 실행모드
}

pub fn get_current_program_mode() -> CurrentProgramMode {
  let next = GLOBAL_PROGRAM_MODE_LOCK.clone();
  let next_lock = Arc::clone(&next);
  let wrapped_value = WrapIt::Read(next_lock.read().unwrap());
  let x = wrapped_value.deref();
  x.recover()
}

// TODO : 내부에 있는 값 꺼내는 방법 : *value.deref()

pub fn set_current_program_mode(value: CurrentProgramMode){
  let next = GLOBAL_PROGRAM_MODE_LOCK.clone();
  let next_lock = Arc::clone(&next);
  let mut t = next_lock.write().unwrap();
  *t = value;
}