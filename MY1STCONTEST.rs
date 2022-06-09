use std::io;
use std::str::FromStr; 
fn main(){
    let mut sval = String::new();
    io::stdin().read_line(&mut sval);
    let strings: Vec<_> = sval.split_whitespace().collect();
    let v: Vec<u16> = strings.iter().flat_map(|x| x.parse()).collect();
    println!("{} {}", v[0]-v[1],v[0]-v[1]-v[2]);
}