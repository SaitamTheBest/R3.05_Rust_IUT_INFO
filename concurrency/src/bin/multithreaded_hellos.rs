use clap::Parser;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
fn main(){
    let args = Parameters::parse();
    let n=args.n;
    for i in 0..n {
        println!("Bonjour n°{}", i);
        println!("Au revoir n°{}", i);
    };
}