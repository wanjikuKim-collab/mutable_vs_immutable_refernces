fn main() {
    let mut s = String::from("hello"); 
    let r1 = &s;
    let r2 = &s;

    let r3: &mut String = &mut s;

println!("r1:{} and r2:{} are immutable references", r1, r2);

}
