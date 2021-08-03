/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:16:17 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:16:17 
 */

#![allow(dead_code)]
// ? 사용하지 않을때도 있으니 dead code 활성화
// TODO : PERMISSION 과 관련된 INTERFACE 들의 정의체

pub enum CurrentProgramMode {
  MASTER,
  SLAVE,
  TRADE,
  CRAWL,
  NONE,
}

impl CurrentProgramMode {
  pub fn value(&self) -> i32 {
    match self {
      CurrentProgramMode::MASTER => 0,
      CurrentProgramMode::SLAVE => 1,
      CurrentProgramMode::TRADE => 2,
      CurrentProgramMode::CRAWL => 3,
      CurrentProgramMode::NONE => -1,
    }
  }
  // TODO : MASTER / SLAVE / TRADE / CRAWL / NONE 에 정수값을 주어 비교하기 위해 사용

  pub fn str_value(&self) -> &str {
    match self {
      CurrentProgramMode::MASTER => "MASTER",
      CurrentProgramMode::SLAVE => "SLAVE",
      CurrentProgramMode::TRADE => "TRADE",
      CurrentProgramMode::CRAWL => "CRAWL",
      CurrentProgramMode::NONE => "NONE",
    }
  }
  // TODO 출력을 위해

  pub fn recover(&self) -> CurrentProgramMode {
    match self {
      CurrentProgramMode::MASTER => CurrentProgramMode::MASTER,
      CurrentProgramMode::SLAVE => CurrentProgramMode::SLAVE,
      CurrentProgramMode::TRADE => CurrentProgramMode::TRADE,
      CurrentProgramMode::CRAWL => CurrentProgramMode::CRAWL,
      CurrentProgramMode::NONE => CurrentProgramMode::NONE,
    }
  }
  // TODO 꼼수 > &CurrentProgramMode 를 이용해서 CurrentProgramMode 로 가져오기 위해 ( threadSafe )
}