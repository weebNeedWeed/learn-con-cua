mod back_of_house;
mod front_of_house;

pub use back_of_house::Breakfast;
pub fn eat_at_restaurant() -> Breakfast {
    let toast = String::from("toast");
    let bf = back_of_house::Breakfast::summer(&toast);
    bf
}
