/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:17:09 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:17:09 
 */

use super::interfaces::permissions::CurrentProgramMode;
use super::variables;

pub mod master;
pub mod slave;
pub mod trade;
pub mod crawl;

// 각 파트별로 빠지는 분기점

pub fn init_program() {
  let current_program_mode: CurrentProgramMode = variables::get_current_program_mode();
  match current_program_mode {
    CurrentProgramMode::MASTER => master::init(),
    // TODO : 조건검색식 가져오는 봇
    CurrentProgramMode::SLAVE => slave::init(),
    // TODO : 조건검색식 무한 크롤링 해주는 봇
    CurrentProgramMode::TRADE => trade::init(),
    // TODO : 중앙 서버에 의해 트레이딩 하는 클라이언트
    CurrentProgramMode::CRAWL => crawl::init(),
    // TODO : 유휴 시간에 따라 크롤링을 진행하는 봇
    _ => {
      // TODO : 오류 / NONE 등으로 인해 NONE 으로 빠졌을경우?
      crate::utils::printing::clear_screen();
      println!("더이상 진행할 수 없습니다.");
      std::process::exit(-1);
    },
  }
}
