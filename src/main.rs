use std::thread;
use std::time::Duration;

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let example_closure = | message: String | -> String{
        println!("{message}");
        message
    };
    let returned_from_closure = example_closure("Daniel".to_string());
    {
        returned_from_closure;
    }

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(10));
        num
    };

    let returned_num = expensive_closure(10);

    let v1 = vec![1, 2, 3];
    for v in &v1 {
        println!("{v}");
    }
    let mut v1_iter = v1.iter();
    println!("{}", v1_iter.next().unwrap());
    println!("{}", v1_iter.next().unwrap());
    println!("{}", v1_iter.next().unwrap());
    println!("{}", v1[0]);

    let mut v2_iter = v1.iter();
    let mapped_iter: Vec<_> = v2_iter.map(|x| x + 1).collect();
    let summed_value: u32 = mapped_iter.sum();
    println!("{summed_value}");
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(
        &self,
        user_preference: Option<ShirtColor>
    ) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}