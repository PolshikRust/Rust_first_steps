// enum и struct у тебя были идеальны
enum ItemKind {
    Weapon,
    Potion,
}

struct Item {
    name: String,
    kind: ItemKind,
}

// Функция-анализатор. Обрати внимание, где теперь `println!`
fn analyze_inventory(inventory: &Vec<Item>) {
    let mut weapon_count = 0;
    let mut potion_count = 0;

    // Цикл for ТОЛЬКО считает предметы. Он ничего не печатает.
    for item in inventory {
        match &item.kind {
            ItemKind::Weapon => {
                weapon_count += 1;
            }
            ItemKind::Potion => {
                potion_count += 1;
            }
        }
    } // Цикл for заканчивается здесь

    // КЛЮЧЕВОЙ МОМЕНТ №1:
    // Блок `println!` с результатами находится ПОСЛЕ цикла `for`, а не внутри него.
    // Он выполняется один раз, когда все уже посчитано.
    println!("--- Inventory Analysis ---\nWeapons: {}\nPotions: {}\n--------------", weapon_count, potion_count);
}

// Главная функция
fn main() {
    let mut inventory = Vec::new();

    // КЛЮЧЕВОЙ МОМЕНТ №2:
    // Я убрал ненужные переменные `sword` и `buff`, чтобы не было предупреждений.
    // Мы создаем предметы прямо при добавлении в вектор. Это чище.
    inventory.push(Item { name: String::from("Silver Sword"), kind: ItemKind::Weapon });
    inventory.push(Item { name: String::from("Iron Sword"), kind: ItemKind::Weapon });
    inventory.push(Item { name: String::from("Buff Potion"), kind: ItemKind::Potion });
    inventory.push(Item { name: String::from("Health Potion"), kind: ItemKind::Potion });
    inventory.push(Item { name: String::from("Mana Potion"), kind: ItemKind::Potion });

    // Вызываем нашу функцию-анализатор
    analyze_inventory(&inventory);
}