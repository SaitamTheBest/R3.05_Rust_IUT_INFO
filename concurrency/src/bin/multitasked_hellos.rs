use clap::Parser;
#[derive(Parser)]
struct Parameters{
    n: usize,
}
#[tokio::main]
async fn main(){
    let args = Parameters::parse();
    let n=args.n;
    let mut tableau = vec![];
    for i in 0..n {
        let thread = tokio::spawn(async move {
            println!("Bonjour n°{}", i);
            println!("Au revoir n°{}", i);
        });
        tableau.push(thread);
    };
    for thread in tableau{
        thread.await;
    }
}