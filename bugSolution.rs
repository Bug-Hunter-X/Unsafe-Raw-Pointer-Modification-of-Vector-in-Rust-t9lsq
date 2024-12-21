fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, modify the vector elements directly.
    v[0] = 10;
    println!( "{:?}", v);
} 