---
title: Rust Type Code
cover: /images/rust.png
date: 2023-11-14T23:14:24+07:00
tags:
  - RUST
draft: true
summary: Rust类型转换代码示例
---

```rust
// 使用std::any::type_name可以打印数据类型

fn print_type_of<T>(_: &T) {
    println!("type is = {}", std::any::type_name::<T>())
}
```

- impl copy

![未命名 14.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/31c2dfc8-f5dd-4394-bdd8-1a17a2afa255/%E6%9C%AA%E5%91%BD%E5%90%8D_14.png)

![未命名 15.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/5e0e6294-45c6-44d5-a9d5-8548ec92503c/%E6%9C%AA%E5%91%BD%E5%90%8D_15.png)

```rust
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/guessing_game`
type is = alloc::string::String
str2 is: "tosei"
type is = alloc::string::String
str is: "tosei"
type is = &[u8]
str1_bytes is: [116, 111, 115, 101, 105]
type is = [u8; 5]
str1_num is: [116, 111, 115, 101, 105]
type is = alloc::vec::Vec<u8>
vec_from_slice is [116, 111, 115, 101, 105]
type is = alloc::vec::Vec<u8>
str1_vec is [116, 111, 115, 101, 105]
type is = alloc::string::String
str_from_vec is "tosei"
type is = alloc::vec::Vec<u8>
vec_from_string is [116, 111, 115, 101, 105]
type is = u32
bytes_u32 is 203435063
```

- struct

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Transfer {
    trans: Rc<Vec<u8>>,
    message: String,
}

impl Transfer {
    fn new(trans: &[u8], message: &str) -> Self {
        Transfer {
            trans: Rc::new(trans.to_owned()),
            message: message.to_owned(),
        }
    }

    fn trans(&self) -> Vec<u8> {
        self.trans.clone().to_vec()
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

fn main() {
    let transfer = Transfer::new(b"tosei", "hello");
    let trans = transfer.trans();
    let message = transfer.message();
    // Vec -> [u8]
    let trans_array: [u8; 5] = trans.try_into().unwrap();
    println!("trans_array is: {:?}", trans_array);

    // String  -> [u8]
    let message_array: [u8; 5] = message.as_bytes().try_into().unwrap();
    println!("message_array is: {:?}", message_array);

    // [u8] -> &str
    let trans_str = std::str::from_utf8(&trans_array).unwrap();
    println!("trans_str is: {:?}", trans_str);
}
```

```rust
❯ cargo run
   Compiling guessing_game v0.1.0 (/Users/tosei/workspace/practice/rust/rust-p/rust_small_demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.82s
     Running `target/debug/guessing_game`
trans_array is: [116, 111, 115, 101, 105]
message_array is: [104, 101, 108, 108, 111]
trans_str is: "tosei"
```
