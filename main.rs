struct InventoryItem {
    name: String,
    quantity: u32,
    description: String,
}
fn main(){
    let item = InventoryItem {
        name: String::from("Health Potion"),
        quantity: 5,
        description: String::from("A simple potion that restores 50 HP")};
        let item_details = format_item_details(&item);
        println!("{}", item_details);
    }
fn format_item_details(inventoryitem: &InventoryItem) -> String {
    let details = format!("--- Item Details ---\nName: {}\nQuantity: {}\nDescription: {}", inventoryitem.name, inventoryitem.quantity, inventoryitem.description);
    return details;
}