use clap::Parser;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
fn main(){
    let args = Parameters::parse();
    let n=args.n;
    for i in 0..n {
        let thread1 = std::thread::spawn(move || {
            println!("Bonjour n°{}", i);
        });
        let thread2 = std::thread::spawn(move || {
            println!("Au revoir n°{}", i);
        });
        thread1.join();
        thread2.join();
    };
}