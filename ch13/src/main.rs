#[derive(Debug)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for c in &self.shirts {
            match c {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let invent = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user1_pref = Some(ShirtColor::Blue);
    let user2_pref: Option<ShirtColor> = None;

    let ga1 = invent.giveaway(user1_pref);
    let ga2 = invent.giveaway(user2_pref);

    println!("{ga1:#?}, {ga2:#?}");
}
