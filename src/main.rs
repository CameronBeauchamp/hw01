const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 1000;
fn print_menu() {
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}
fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}
fn hunt_food(roll: u64) -> u64 {

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
fn roll_die(seed: u64) -> u64 {
    rand::random_range(1..=6)
}
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;

// Roll count times
    
for _ in 0..count {
        total += roll_die(seed);
    }
// Return total of all rolls

    total
}
fn print_status(day: u64, gold: u64, food: u64, roll_count: u64) {
    println!("Day: {}", day);
    println!("Gold: {}", gold);
    println!("Food: {}", food);
    println!("Roll Count: {}", roll_count);
}
fn read_number(min: u64, max: u64) -> u64 {
   

let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let number: u64 = match input.trim().parse() {
    Ok(n) => n,
    Err(_) => return min,
};
if number < min || number > max {
    return min;
}
return number;

}
fn main() {
let mut gold: u64 = START_GOLD;
let mut seed: u64 = read_number(1, MAX_SEED);
let mut food: u64 = 0;
let mut day: u64 = 1;
let mut roll_count: u64 = 0;   
 print_menu();
let _choice: u64 = read_number(1, 5);
let _can_buy = can_afford(gold, FOOD_PRICE);
let roll: u64 = roll_die(seed);
let _total_rolls: u64 = roll_many(seed, 1);
println!("Roll: {}", roll);
let mut food_found: u64 = hunt_food(roll);
println!("Food found: {}", food_found);
let mut total_food: u64 = food + food_found;
println!("Total food: {}", total_food);
println!("Choice: {}", _choice);
if _choice == 1 {
    food += 1;
    gold -= FOOD_PRICE;
    day += 1;
}

if _choice == 2 {
    food += hunt_food(roll);
    roll_count += 1;
    day += 1;
}

if _choice == 3 {
    gold += 2 * roll_many(seed, 3);
    roll_count += 3;
    day += 1;
}

if _choice == 4 {
    print_status(day, gold, food, roll_count);
}

if _choice == 5 {
    println!("Goodbye!");


    

}
println!("Game started.");
println!("Good luck!");
println!("Let's begin!");
println!("Day {}", day);

print_status(day, gold, food, roll_count);
}

