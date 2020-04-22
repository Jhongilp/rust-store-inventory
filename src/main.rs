use rand::Rng;

fn main() {

    let item = Item::create_item();

    
    println!("item: {:?}", item);



}


fn generate_color() -> &'static str {
    let colors = vec!["red", "blue", "gray", "purple", "yellow", "orange"];
    let picker = rand::thread_rng().gen_range(0, colors.len());
    println!("picker: {} from {}", picker, colors.len());
    colors[picker]
}

fn generate_code() -> u32 {
    rand::thread_rng().gen_range(0, 10000000)
}

fn select_item_type() -> &'static str {
    let options = vec!["Shoes", "Skirts", "Pants", "Socks", "Other"];
    let picker = rand::thread_rng().gen_range(0, options.len());
    options[picker]
}

fn select_size_item() -> u8 {
    let options: [u8; 14]= [4,6,8,10,12,14,16,18,20,22,24,26,28,30];
    let picker= rand::thread_rng().gen_range(0, options.len());
    options[picker]
}

fn select_season() -> &'static str {
    let options: [&'static str; 4] = ["Spring", "Summer", "Fall", "Winter"];
    let picker = rand::thread_rng().gen_range(0, options.len());
    options[picker]
}


#[derive(Debug)]
struct Item {
    code_item: u32,
    type_item: String,
    name: String,
    size: u8,
    color: String,
    available: bool,
    season: String,
    brand: String,
}

impl Item {
    fn create_item() -> Item {
        let code_item = generate_code();
        let type_item = String::from(select_item_type());
        let color = String::from(generate_color());
        let size = select_size_item();
        let season = select_season();
        Item {
            code_item,
            type_item,
            name: String::from("Slim pants"),
            size,
            color,
            available: true,
            season,
            brand: String::from("Levy's"),
        }
    }
}
