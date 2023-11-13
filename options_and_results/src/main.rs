fn print_first_word1(chaine : &str){
    let longueur = chaine.len();
    match longueur{
        0 => {
            println!("Chaine vide")
        }
        _ => {
            println!("Premier mot : {:?} " , chaine.split_whitespace().next());
        }
    }
}

fn print_first_word2(chaine : &str) -> & str {
    let mot = chaine.split_whitespace().next().expect("La chaine doit être non vide");
    return mot;
}

fn main() {
    //Exercice 2.1
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2);

    //Exercice 2.2
    println!("{}", print_first_word2(sentence1));
    println!("{}", print_first_word2(sentence2));
}