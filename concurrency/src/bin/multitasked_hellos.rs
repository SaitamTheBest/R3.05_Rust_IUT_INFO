use clap::Parser;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
#[tokio::main]
async fn main(){
    let args = Parameters::parse();
    let n=args.n;
    for i in 0..n {
        let thread1 = tokio::spawn(async move {
            println!("Bonjour n°{}", i);
        });
        let thread2 = tokio::spawn(async move {
            println!("Au revoir n°{}", i);
        });
        thread1.await;
        thread2.await;
    };
}