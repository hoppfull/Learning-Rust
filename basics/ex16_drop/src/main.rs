// Will print out "my_struct2 msg!" first then "my_struct1 msg!"
fn main() {
    let my_struct1 = MyStruct { msg: "my_struct1 msg!".to_string() };
    let my_struct2 = MyStruct { msg: "my_struct2 msg!".to_string() };

    assert_eq!("my_struct1 msg!", my_struct1.msg);
    assert_eq!("my_struct2 msg!", my_struct2.msg);
}

struct MyStruct {
    msg: String,
}

// Drop is like a deconstructor, drop is called when object goes out of scope:
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("{}", self.msg);
    }
}
