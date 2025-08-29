struct InventoryItem {
    name: String,
    quantity: u32,
    description: String,
    item_type: String,
}

impl InventoryItem {
    fn format_item_details(&self) -> String {
    format!("--- Item Details ---\nName: {}\nQuantity: {}\nDescription: {}\nType: {}", 
    self.name, self.quantity, self.description, self.item_type)
    }
}
 

fn main(){
    let mut inventory = Vec::new();
        let potion = InventoryItem { 
        name: String::from("Heath Potion"), 
        quantity: 5, 
        description: String::from("Restore 50 HP"),
        item_type: String::from("Potion"),
    };

        let sword = InventoryItem { 
            name: String::from("Silver Sword"), 
            quantity: 1, 
            description: String::from("Basic Weapon"),
            item_type: String::from("Weapon"),
        };
        let shield = InventoryItem { 
            name: String::from("War Shield"), 
            quantity: 1, 
            description: String::from("Legendary Shield of Great War"),
            item_type: String::from("Item"),
        };

        inventory.push(potion);
        inventory.push(sword);
        inventory.push(shield);

         println!("--- Player Inventory ---");
          for item in &inventory {
            if item.item_type == "Potion" {
            println!("- {:<15} (x{}) {}", item.name, item.quantity, item.item_type);
            
         } else if item.item_type == "Weapon" {
            println!("- {:<15} (x{}) {}", item.name, item.quantity, item.item_type);

         } else {
            println!("- {:<15} (x{}) {}", item.name, item.quantity, item.item_type);
         }
        }
            println!("----------------------");
    }
    
