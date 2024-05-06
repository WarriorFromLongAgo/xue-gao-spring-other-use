use rand::Rng;

fn main() {
    println!("======= {} ======= ", "test_rand");
    println!("======= {} ======= ", "test_rand");
    println!("======= {} ======= ", "test_rand");

    let rand_str = rand::thread_rng().gen_range(1..101);
    println!("随机的数据是 {}", rand_str);
}