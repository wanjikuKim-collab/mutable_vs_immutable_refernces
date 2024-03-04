//mutable and immutable variables
fn main() {
    let mut s = String::from("hello"); 
    let r1 = &s;
    let r2 = &s;

    println!("r1:{} and r2:{} are immutable references", r1, r2);
    
    //borrowing s when it's out of scope for the immutable variables
    let r3: &mut String = &mut s;
    println!("r3:{} is a mutable reference", r3);
}
