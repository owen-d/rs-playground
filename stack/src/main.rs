struct Stack {
  _data: Vec<i32>
}

impl Stack {
  fn new() -> Stack {
    Stack {
      _data: Vec::new()
    }
  }
  fn push(&mut self, val: i32) -> &mut Stack {
    self._data.push(val);
    self
  }
  fn pop(&mut self) -> i32 {
    match self._data.pop() {
      Some(v) => v,
      None => 0
    }
  }
}

fn main() {
  let mut s = Stack::new();
  s.push(2).push(4);
  // for x in &s._data {
  //   println!("{}", x);
  // }
  // for x in s._data.len() {
  //   println!("{}", x.pop())
  // }
  let mut range = 0..s._data.len();
  loop {
    match range.next() {
      Some(_) => println!("{}", s.pop()),
      None => { break }
    }
  }
}