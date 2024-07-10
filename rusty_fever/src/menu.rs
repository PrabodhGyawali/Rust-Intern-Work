use crate::menu_item::MenuItem;

pub struct Menu {
    pub menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu_items: Vec::new(),
        }
    } // Constructor of the Menu struct

    pub fn add_item(&mut self, item: MenuItem) {
        self.menu_items.push(item);
    } // Adding MenuItem 
}

// Defualt Menu
pub fn level_1_menu() -> Menu {
    let mut menu = Menu::new();
    menu.add_item(MenuItem::new("Burger", 10.0));
    menu.add_item(MenuItem::new("Coke", 2.0));
    return menu;
}
