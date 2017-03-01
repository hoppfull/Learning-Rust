fn main() {
    let mybool: bool = true;
    println!("mybool={}", mybool); // true
    let mychar: char = 'a';
    println!("mychar={}", mychar); // a
    let myint: i16 = 7; // signed integer of 16 bits
    println!("myint={}", myint); // 7
    let myfloat: f32 = 0.2; // floating-point of 32 bits
    println!("myfloat={}", myfloat); // 0.2
    let myuint: u64 = 10; // unsigned integer of 64 bits
    println!("myuint={}", myuint); // 10
    let mycpuint: isize = 2; // signed integer of bits determined by cpu
    let mycpuuint: usize = 1; // unsigned integer of bits determined by cpu
    println!("mycpuuint={1}, mycpuint={0}", mycpuint, mycpuuint); // 1, 2
}
