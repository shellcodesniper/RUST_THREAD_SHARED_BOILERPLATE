/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:16:01 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:16:01 
 */

#[macro_use]
extern crate log;
// use std::sync::{Arc, RwLock};
// use std::{time};
// use std::thread;

mod interfaces;
// ? 인터페이스 분리

mod permissions;
// ? 권한 관련하여 처리하는 부분

mod utils;
// ? 유틸 메소드

mod variables;
// ? 전역 변수 등

mod program_mode;
// ? Master / Slave / Crawler / Trader / None 으로 구성, 프로그램의 현재 모드를 

mod communicator;
// ? 키움 / 대신 등과 통신을 담당하는 부분, Listener, Sender 로 구성

fn initializing () {
  if cfg!(debug_assertions) {
    std::env::set_var("RUST_LOG", "debug");
    
  } else {
    std::env::set_var("RUST_LOG", "info");
    
  }
  env_logger::builder()
    .format_module_path(false)
    .format_indent(Some(4))
    .format_suffix("\n___ CONTACT : admin@kuuwang.com ___\n\n")
    .format_target(false)
    .init();

  if cfg!(debug_assertions) {
    debug!("DEBUG MODE ENABLED");
  } else {
    info!("PRODUCTION MODE ENABLED");
  }
}
// ? 초기화

fn main() {
  utils::printing::clear_screen();
  println!("");
  initializing();

  permissions::check_permission();
  // TODO 권한 체크 및 종료

  program_mode::init_program();

  // let lock = Arc::new(RwLock::new(1));
  // let c_lock = Arc::clone(&lock);
  // thread::spawn(move || loop {
  //   let r = c_lock.read();
  //   println!("{:?}", r.unwrap());
  //   thread::sleep(time::Duration::from_millis(500));
  // });

  // loop {
  //   let mut w = lock.write().unwrap();
  //   *w += 1;
  //   thread::sleep(time::Duration::from_secs(1));
  //   println!("현재 프로그램 모드 {}", current_program_mode.str_value());
  // }
}