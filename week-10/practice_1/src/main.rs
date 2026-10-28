fn main() {
    // a vector v owns the object in heap
    let v = vec![101, 250, 330, 400];
    
    // only a single variable owns the heap memory at any given time
    let v2 = v;
    
    // here two variables owns heap value,
    // two pointers to the same content is not allowed in rust
    // Rust is very smart in terms of memory access so it detects a race condition
    // as two variables point to same heap
    println!("{:?}", v2);
}