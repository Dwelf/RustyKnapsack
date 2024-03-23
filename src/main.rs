#[derive(Copy, Clone)]
struct Item {
    name: &'static str,
    weight: f32,
    value: f32,
    value_ratio: f32,
}

impl Item {
    fn new(name: &'static str, weight: f32, value: f32) -> Self {
        let value_ratio = value / weight;

        return Self {
            name,
            weight,
            value,
            value_ratio,
        };
    }
}

fn main() {
    println!("knapsacking problem");

    const KNAPSACK_MAX_WEIGHT: f32 = 400.0;

    let items = vec![
        Item::new("map", 9.0, 150.0),
        Item::new("compass", 13.0, 35.0),
        Item::new("water", 153.0, 200.0),
        Item::new("sandwich", 50.0, 160.0),
        Item::new("glucose", 15.0, 60.0),
        Item::new("tin", 68.0, 45.0),
        Item::new("banana", 27.0, 60.0),
        Item::new("apple", 39.0, 40.0),
        Item::new("cheese", 23.0, 30.0),
        Item::new("beer", 52.0, 10.0),
        Item::new("suntancream", 11.0, 70.0),
        Item::new("camera", 32.0, 30.0),
        Item::new("T-shirt", 24.0, 15.0),
        Item::new("trousers", 48.0, 10.0),
        Item::new("umbrella", 73.0, 40.0),
        Item::new("waterproof trousers", 42.0, 70.0),
        Item::new("waterproof overclothes", 43.0, 75.0),
        Item::new("note-case", 22.0, 80.0),
        Item::new("sunglasses", 7.0, 20.0),
        Item::new("towel", 18.0, 12.0),
        Item::new("socks", 4.0, 50.0),
        Item::new("book", 30.0, 10.0),
    ];

    println!("----knapsack items-----");
    print_items(&items);
    greedy_solution(items.clone(), KNAPSACK_MAX_WEIGHT)
}

fn greedy_solution(items: Vec<Item>, knapsack_max_weight: f32) {
    println!("----Greedy Solution knapsack-----");
    let mut greedy_items = items.to_vec();
    greedy_items.sort_by(|a, b| b.value_ratio.partial_cmp(&a.value_ratio).unwrap_or(std::cmp::Ordering::Equal));

    println!("----sorted items-----");

    let mut current_weight = 0.0;
    let mut total_value = 0.0;

    println!("----Greedy Backpack-----");
    for item in greedy_items {
        if (current_weight + item.weight) < knapsack_max_weight as f32 {
            current_weight += item.weight;
            total_value += item.value;
            println!("Item: {}", item.name);
        }
    }

    println!("------------------------");
    println!("Total Value: {}", total_value);
    println!("Total Weight: {}", current_weight);
}

fn print_items(items: &[Item]) {
    for item in items {
        println!("Item: {}", item.name);
    }
}