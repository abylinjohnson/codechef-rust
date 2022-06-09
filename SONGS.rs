use std::io;
fn main(){
    let mut sn = String::new();
    io::stdin().read_line(&mut sn);
    sn = sn.trim().to_string();
    let n: u32 = sn.parse::<u32>().unwrap();
    for x in 0..n{
        let mut sval = String::new();
        io::stdin().read_line(&mut sval);
        let strings: Vec<_> = sval.split_whitespace().collect();
        let v: Vec<u16> = strings.iter().flat_map(|x| x.parse()).collect();
        let a: u16  = v[0] / (v[1]*3);
        println!("{}",a);
    }
}