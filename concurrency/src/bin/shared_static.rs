use clap::Parser;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
fn main(){
    let args = Parameters::parse();
    let n=args.n;
    let stri : &str;
    stri = "Bonjour";
    let mut tableau = vec![];
    for i in 0..n {
        let thread = std::thread::spawn(move || {
            println!("{} n°{}", stri,i);
            println!("Au revoir n°{}", i);
        });
        tableau.push(thread);
    };
    for thread in tableau{
        thread.join().unwrap();
    }
}