fn print_menu() {
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}
fn can_afford(gold: i32, cost: i32) -> bool {
    gold >= cost
}
fn hunt_food(roll: i32) -> i32 {
    if roll == 1 {
        0
    } else if roll <= 3 {
        10
    } else if roll <= 5 {
        20
    } else {
        40
    }
}
fn roll_die() -> i32 {
    rand::random_range(1..=6)
}
fn roll_many(count: i32) -> i32 {
    let mut total = 0;

// Roll count times
    
for _ in 0..count {
        total += roll_die();
    }
// Return total of all rolls

    total
}
fn print_status(day: i32, gold: i32, food: i32, roll_count: i32) {
    println!("Day: {}", day);
    println!("Gold: {}", gold);
    println!("Food: {}", food);
    println!("Roll Count: {}", roll_count);
}
fn read_number(min: i32, _max: i32) -> i32 {
    min
// TODO: read user input
}
fn main() {
let gold = 100;
    let food = 0;
    let day = 1;
    let roll_count = 0;   
 print_menu();
let _choice = read_number(1, 5);
let roll = roll_die();
println!("Roll: {}", roll);
let food_found = hunt_food(roll);
println!("Food found: {}", food_found);
println!("Choice: {}", _choice);
print_status(day, gold, food, roll_count);
}

