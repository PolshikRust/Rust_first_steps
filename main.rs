fn format_character_sheet(name: &String, level: &u32, class: &String) -> String {
let character_sheet = format!("--- Character Sheet --- \n Name: {} \n Level: {} \n Class {}", name, level, class);
return character_sheet;

}
fn main(){
    let name = String::from("Legolas");
    let level = 18;
    let class = String::from("Archer");
    let character_sheet = format_character_sheet(&name, &level, &class);
    println!("{}",character_sheet);
    println!("The character {} is ready for adventure!", name);
}