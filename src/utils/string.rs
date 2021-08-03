/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:17:18 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:17:18 
 */

pub fn to_string(s: &str) -> String {
  String::from_utf8(s.as_bytes().to_vec())
    .unwrap_or(String::from(""))
  // ? 소유권은 호출한쪽으로 돌아간다?
}

#[allow(dead_code)]
pub fn remove_whitespace_mut(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn remove_whitespace_ret(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}