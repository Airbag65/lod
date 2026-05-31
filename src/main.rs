use std::fmt::Display;

use anyhow::anyhow;
use serde_json::Value;

struct Restaurant {
    id: u8,
    name: String,
    menu: Vec<String>,
}

impl Restaurant {
    fn new(id: u8, name: String) -> Restaurant {
        Restaurant {
            id,
            name,
            menu: vec![],
        }
    }

    fn add_menu_item(&mut self, item: String) {
        self.menu.push(item.clone());
    }
}

impl Display for Restaurant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**{}**:\n", self.name)?;
        if self.menu.is_empty() {
            write!(f, "- *NO MENU AVAILABLE YET*\n")?;
            return Ok(());
        }
        for item in &self.menu {
            write!(f, "- {}\n", item)?;
        }
        Ok(())
    }
}

fn main() {
    // Get API key from: https://lunchaimjardevi.com/api/index.php
    dotenvy::dotenv().ok();
    let mut restaurants = match get_restaurants() {
        Ok(val) => val,
        Err(err) => return eprintln!("Error: {err}"),
    };
    for res in &mut restaurants {
        match get_menu(res) {
            Ok(_) => (),
            Err(err) => return eprintln!("Error: {err}"),
        };
    }
    for res in &restaurants {
        println!("{res}");
    }
}

fn get_restaurants() -> Result<Vec<Restaurant>, anyhow::Error> {
    let res = reqwest::blocking::get(format!(
        "https://lunchaimjardevi.com/api/v4/getRestaurants?key={}",
        std::env::var("API_KEY")?
    ))?;
    let val: Value = serde_json::from_str(res.text()?.as_str())?;
    if val.get("error").unwrap() != "none" {
        return Err(anyhow!("Error-field(get_restaurants) not none"));
    }
    let restaurants = val.get("restaurants").unwrap();
    let mut result: Vec<Restaurant> = vec![];
    if !restaurants.is_array() {
        return Err(anyhow!("\"restaurants\" is not an array"));
    }
    for rest in restaurants.as_array().unwrap() {
        let id: u8 = rest.get("id").unwrap().as_str().unwrap().parse().unwrap();
        let name: String = rest.get("name").unwrap().as_str().unwrap().to_string();
        result.push(Restaurant::new(id, name));
    }

    Ok(result)
}

fn get_menu(restaurant: &mut Restaurant) -> Result<(), anyhow::Error> {
    let res = reqwest::blocking::get(format!(
        "https://lunchaimjardevi.com/api/v4/getMenu?id={}&key={}",
        restaurant.id,
        std::env::var("API_KEY")?
    ))?;
    let val: Value = serde_json::from_str(res.text()?.as_str())?;
    if val.get("error").unwrap() != "none" {
        return Ok(());
    }
    let menu_items = val.get("menuItems").unwrap();
    if !menu_items.is_array() {
        return Err(anyhow!("\"menuItems\" is not an array "));
    }
    for item in menu_items.as_array().unwrap() {
        restaurant.add_menu_item(String::from(
            item.get("title").unwrap().as_str().unwrap().to_string(),
        ));
    }
    Ok(())
}
