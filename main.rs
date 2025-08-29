struct InventoryItem {
    name: String,
    quantity: u32,
    description: String,
}

impl InventoryItem {
    fn format_item_details(&self) -> String {
    format!("--- Item Details ---\nName: {}\nQuantity: {}\nDescription: {}", self.name, self.quantity, self.description)
    }


fn use_item(&mut self) {
    if self.quantity > 0 {
        self.quantity -= 1; // Decrease quantity 
        println!("Used one {}. Remaining: {}.", self.name, self.quantity);
    } else {
        println!("Cannot use {}. None left!", self.name);
    }
    }}
 

fn main(){
    let mut item = InventoryItem {
        name: String::from("Health Potion"),
        quantity: 5,
        description: String::from("A simple potion that restores 50 HP")};
        let initial_details = item.format_item_details();
         println!("{}", initial_details);
         println!("\n--- Using Items ---");
         item.use_item();
         item.use_item();
         item.use_item();
         println!("\n--- Final State ---");
         let final_details = item.format_item_details();
         println!("{}", final_details);

    }
