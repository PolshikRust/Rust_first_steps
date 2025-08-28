fn format_log_entry(event: &String) -> String {
        let new_log_entry = format!("[LOG: 28.08.2025] {}", event);
return new_log_entry;
} 

fn main(){
    let latest_event = String::from("The Dragon was defeated");
let journal_entry = format_log_entry(&latest_event);
println!("Original evenet: {}", latest_event);
println!("New journal entry: {}", journal_entry);
}