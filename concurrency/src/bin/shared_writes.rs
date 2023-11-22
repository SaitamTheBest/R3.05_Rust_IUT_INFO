use clap::Parser;
use std::sync::RwLock;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
fn main(){
    let args = Parameters::parse();
    let n=args.n;
    let str = RwLock::new("Bonjour");
    let mut tableau = vec![];
    for i in 0..n {
        let clone_str = str.read().unwrap().clone();
        let thread = std::thread::spawn(move || {
            println!("{} n°{}", clone_str, i);
            println!("Au revoir n°{}", i);
        });
        tableau.push(thread);
    };
    for thread in tableau{
        thread.join().unwrap();
    }
}