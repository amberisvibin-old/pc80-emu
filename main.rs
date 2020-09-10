extern crate rz80;

use rz80::{CPU,Bus,RegT};
use std::str;
use std::convert::TryInto;
use std::cell::RefCell;

//system
struct PC80 {
  pub cpu: RefCell<CPU>,
}

//bus on system
impl Bus for PC80 {
  fn cpu_inp(&self, port: RegT) -> RegT {
    0x0000
  }
  fn cpu_outp(&self, port: RegT, data: RegT) {
    match port {
      0x0000 => out_text(data),
      _ => ()
    }
  }
}

//system functions
impl PC80 {
  pub fn new() -> PC80 {
    PC80 {
      cpu: RefCell::new(CPU::new()),
    }
  }
  pub fn power_on(&self) {
    let mut cpu = self.cpu.borrow_mut();

    //map 32k of memory
    cpu.mem.map(0, 0, 0x0000, true, 32*1024);

    //start exec at 0x0000
    cpu.reg.set_pc(0x0000); 
  }
  pub fn run(&self) {
    let mut cpu = self.cpu.borrow_mut();

    for _i in 0..100000 {
      cpu.step(self);
    }
    
  }
}

//TODO: these funcs are kinda messy
#[allow(dead_code)]
fn out_byte(byte: i32) {
  println!("{:#x}", byte);
}
#[allow(dead_code)]
fn out_text(byte: i32) {
  print!("{}", str::from_utf8(&[byte.try_into().unwrap()]).unwrap());
}

fn main() {
  println!("rz80 test");

  let pc80 = PC80::new();
  pc80.power_on();
  pc80.run();

}