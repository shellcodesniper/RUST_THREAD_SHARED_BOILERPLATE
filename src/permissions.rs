/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:16:40 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:16:40 
 */

const ACCESS_KEY: &'static str = "PASSWORD";
// ? ACCESS_KEY : 접근 가능 키

use super::{ utils, utils::printing };
use super::interfaces::permissions::CurrentProgramMode;
// ? 사용하는 모듈 / crate

fn show_usage_and_exit(program_path: &str) {
  printing::clear_screen();
  colour::dark_red!("\n\n\nUsage: {} [MASTER/SLAVE/TRADE/CRAWL] [password]\n\n", program_path);
  colour::cyan!("\n\n비밀번호는 환경변수 STOCK_KEY로 전달을 권장합니다.\n\n\n\n");
  std::process::exit(-1);
}

pub fn check_permission() {
  // ? 권한 체크 및 비밀번호 확인, 프로그램 모드 설정
  let system_args: Vec<String> = std::env::args().collect();

  // * 커맨드라인 인자 0 : 프로그램 경로, 1 : M/S/T/C, 2: 비밀번호
  // * 비밀번호는 환경변수를 통해 전달하기를 권장

  if system_args.len() < 2 {
    show_usage_and_exit(system_args[0].as_str());
  }
  let env_password: String =
    if let Some(s) = std::env::var_os("STOCK_KEY") {
      let result: String = 
        if let Some(var) = s.to_str() {
          utils::string::to_string(var)
        } else {
          String::from("NONE")
        };
      result
    } else {
      String::from("NONE")
    };
  // TODO : 환경변수에서 password 추출 시도

  if (system_args.len() < 3
    || (system_args.len() >= 3 && system_args[2] != ACCESS_KEY))
    && (env_password != ACCESS_KEY)
  {
    show_usage_and_exit(system_args[0].as_str());
  } else {
    let selected_mode = match utils::string::remove_whitespace_ret(system_args[1].as_str()).as_str() {
      "MASTER" => CurrentProgramMode::MASTER,
      "SLAVE" => CurrentProgramMode::SLAVE,
      "TRADE" => CurrentProgramMode::TRADE,
      "CRAWL" => CurrentProgramMode::CRAWL,
      _ => {
        show_usage_and_exit(system_args[0].as_str());
        std::process::exit(-1);
      },
    };
    super::variables::set_current_program_mode(selected_mode);

    let current_program_mode: CurrentProgramMode = super::variables::get_current_program_mode();
    info!("현재 프로그램 모드 {}", current_program_mode.str_value());
  }
  // TODO : 패스워드 비교 및 프로그램 종료 ( 도움말 출력 이후 )

  let _program_mode: bool = std::env::var("STOCK_MASTER").is_err();
  let program_key: String = match std::env::var("STOCK_KEY") {
    Ok(m) => String::from(m).to_owned(),
    Err(_) => String::from("").to_owned(),
  };

  if program_key.trim() != ACCESS_KEY {
    println!("ErrorWhile Initialize Program");
    std::process::exit(-1);
  }
}

// TODO : CurrentProgramMode 에 따라서
// TODO : MASTER > 주식 조건검색식에 따라 읽어오기만 하는 용도로 사용
// TODO : SLAVE > 주식 조건검색식에 따라 읽어온 내용에 따라 KAFKA 를 통해 검색된 결과를 중앙 서버로 보내는 역할
// TODO : TRADE > 중앙 서버의 결과값에 따라 실제 트레이딩에 관여하는 역할
// TODO : CRAWL > 유휴시간동안 크롤링만 담당하는 역할