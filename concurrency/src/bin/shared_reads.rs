use std::sync::Arc;
use clap::Parser;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
fn main(){
    let args = Parameters::parse();
    let n=args.n;
    let value = Arc::new(12345);
    let mut tableau = vec![];
    for i in 0..n {
        let value = Arc::clone(&value);
        let thread = std::thread::spawn(move || {
            println!("Bonjour n°{}", i);
            println!("Au revoir n°{}", i);
            println!("{:?}",value);
        });
        tableau.push(thread);
    };
    for thread in tableau{
        thread.join().unwrap();
    }
}