/*
 * Rust自習（std::time::Instant）
 * CreatedAt: 2019-07-26
 */
fn main() {
    let ins = std::time::Instant::now();
    println!("{:?}", ins);
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{:?}", ins.elapsed().as_secs());

    let new = std::time::Instant::now();
    println!("{:?}", new.duration_since(ins));
    println!("{:?}", new - ins);
//    println!("{:?}", new + ins); // error[E0308]: mismatched types
}

