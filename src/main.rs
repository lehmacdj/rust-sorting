extern crate sorting;

fn main() {
    let mut b = [1,2,1];
    sorting::quicksort(&mut b[..]);
    println!("{:?}", b);
}
