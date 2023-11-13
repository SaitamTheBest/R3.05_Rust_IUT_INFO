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

fn main() {
    //Exercice 2.1
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2);
}