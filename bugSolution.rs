fn main() {
    let mut x = 5;
    { // creating a new scope
        let y = &mut x;
        *y = 6; 
    }
    { // creating a new scope
        let z = &mut x; 
        *z = 7;
    }
    println!("x = {}", x);
}