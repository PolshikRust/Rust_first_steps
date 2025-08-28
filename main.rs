fn main() {
    let player_name = String::from("Aragorn");
    let player_level = 15;
    generate_report(&player_name, &player_level);
    println!("Welcome to the main hall {}!", player_name);
}
fn generate_report(name: &String, level: &u32){
println!("Report: Player {} is level {}.", name, level);}
   
