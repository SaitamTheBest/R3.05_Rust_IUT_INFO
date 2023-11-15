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

fn iterate_over_words(chaine : &str){
    for word in chaine.split_whitespace() {
        println!("{}", word);
    }
}

fn convert_to_int1(chaine: &str){
    match chaine.parse::<i32>(){
        Ok(n) => println!("Le carré de {chaine} vaut {}", chaine.parse::<i32>().unwrap()*chaine.parse::<i32>().unwrap()),
        Err(e) => println!("{chaine} n'est pas convertible"),
    }
}
fn convert_to_int2(chaine: &str){
    let numero :i32 = chaine.trim().parse()
        .expect("La chaine n'est pas un nombre entier!");
    let chaine1 :String = numero.to_string();
    let resultat = numero*numero;
    let chaine2 :String = resultat.to_string();
    println!("le carré de {chaine1} vaut {chaine2}");
}
fn convert_to_int3(chaine: &str) -> anyhow::Result<()>{
    let numero : i32 = chaine.trim().parse()?;
    Ok(())
}
fn main() {
    //Exercice 2.1
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2);

    //Exercice 2.2
    println!("{}", print_first_word2(sentence1));
    //println!("{}", print_first_word2(sentence2));

    //Exercice 2.3
    iterate_over_words(sentence1);
    iterate_over_words(sentence2);

    //Exercice 3
    let string1 = "-17";
    let string2 = "Tux";

    //Exercice 3.1
    convert_to_int1(string1);
    convert_to_int1(string2);

    //Exercice 3.2
    convert_to_int2(string1);
    //convert_to_int2(string2);

    //Exercice 3.3
    println!("{:?}", convert_to_int3(string1));
    println!("{:?}", convert_to_int3(string2));

}