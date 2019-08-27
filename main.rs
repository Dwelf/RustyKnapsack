struct Item {
    name:&'static str,
    weight:f32,
    value:f32,
    value_ratio:f32,
    
}

fn main() {
    println!("knapsacking problem");

    let _knapsack_max_weight = 400.0;

    let mut items = vec!
   [
        Item { name: "map",                    weight: 9f32,   value: 150f32, value_ratio:0.0 },
        Item { name: "compass",                weight: 13f32,  value: 35f32, value_ratio:0.0 },
        Item { name: "water",                  weight: 153f32, value: 200f32, value_ratio:0.0 },
        Item { name: "sandwich",               weight: 50f32,  value: 160f32, value_ratio:0.0 },
        Item { name: "glucose",                weight: 15f32,  value: 60f32, value_ratio:0.0 },
        Item { name: "tin",                    weight: 68f32,  value: 45f32, value_ratio:0.0 },
        Item { name: "banana",                 weight: 27f32,  value: 60f32, value_ratio:0.0 },
        Item { name: "apple",                  weight: 39f32,  value: 40f32, value_ratio:0.0 },
        Item { name: "cheese",                 weight: 23f32,  value: 30f32, value_ratio:0.0 },
        Item { name: "beer",                   weight: 52f32,  value: 10f32, value_ratio:0.0 },
        Item { name: "suntancream",            weight: 11f32,  value: 70f32, value_ratio:0.0 },
        Item { name: "camera",                 weight: 32f32,  value: 30f32, value_ratio:0.0 },
        Item { name: "T-shirt",                weight: 24f32,  value: 15f32, value_ratio:0.0 },
        Item { name: "trousers",               weight: 48f32,  value: 10f32, value_ratio:0.0 },
        Item { name: "umbrella",               weight: 73f32,  value: 40f32, value_ratio:0.0 },
        Item { name: "waterproof trousers",    weight: 42f32,  value: 70f32, value_ratio:0.0 },
        Item { name: "waterproof overclothes", weight: 43f32,  value: 75f32, value_ratio:0.0 },
        Item { name: "note-case",              weight: 22f32,  value: 80f32, value_ratio:0.0 },
        Item { name: "sunglasses",             weight: 7f32,   value: 20f32, value_ratio:0.0 },
        Item { name: "towel",                  weight: 18f32,  value: 12f32, value_ratio:0.0 },
        Item { name: "socks",                  weight: 4f32,   value: 50f32, value_ratio:0.0 },
        Item { name: "book",                   weight: 30f32,  value: 10f32, value_ratio:0.0 }
    ];
    
    for item in &items {
          println!("Item: {}", item.name);
    }
    
    sort_items(&mut items);
    
     println!("----knapsack items-----");
    
    for item in &items {
          println!("Item: {}", item.name);
    }
    
    
    for i in 0..items.len() {
        for k in 0..items.len() {
            if items[i].value_ratio  > items[k].value_ratio {
                items.swap(i,k);
            }
        }
    }
    
      println!("----sorted items-----");
    
    for item in &items {
          println!("Item: {}", item.name);
    }
    
    let mut knapsack_contents = Vec::new();
    let mut current_weight = 0.0;
    
    for item in &items {
        if (current_weight + item.weight) < _knapsack_max_weight {
            knapsack_contents.push(item);
            current_weight += item.weight;
        }
    }
    
    println!("----Greedy Solution knapsack-----");
    let mut total_value = 0.0;
    for item in &knapsack_contents {
          println!("Item: {}", item.name);
          total_value += item.value;
    }
    
    println!("Total Value: {}", total_value);
    println!("Total Weight: {}", current_weight);
    
    
}
fn sort_items(items: &mut[Item]) {
    for i in items{
        i.value_ratio = i.value/i.weight;
    }
}
