use quartet::NibSlice;

fn main() {
    let n = NibSlice::from_bytes(&[1,2,3,4]);
    println!("{:?}", n);
    println!("{}", n);
}
