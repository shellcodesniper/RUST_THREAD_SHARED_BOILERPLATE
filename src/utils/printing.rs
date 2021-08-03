/*
 * @Author: KuuwangE [admin@kuuwang.com] OR ShellcodeSniper [shellcodesniper@icloud.com] 
 * @Date: 2021-08-03 16:17:14 
 * @Last Modified by:   KuuwangE 
 * @Last Modified time: 2021-08-03 16:17:14 
 */

pub fn clear_screen() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}