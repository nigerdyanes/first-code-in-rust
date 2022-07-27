use::rand::random;
use fake::faker::name::raw::*;
use fake::{locales::*, Fake};
#[derive(Debug)]
struct Competitor {
    name: String,
    last_name: String,
    age: i32,
    pass: bool,
}

impl Competitor {
    fn passed(&mut self, status: bool) -> () {
        self.pass = status;
    }

    fn create(name: String, last_name: String, age: i32) -> Competitor {
        Competitor {
            name,
            last_name,
            age,
            pass: false,
        }
    }
}

fn main() {
    let mut competitor = make_fake_competitor();

    if competitor.age > 17 {
        competitor.passed(true);
    }


    if competitor.pass {
        println!("Congratulations!! {} {}", competitor.name, competitor.last_name);
    }else {
        println!("Sorry, you did not pass");
    }
}

fn make_fake_competitor() -> Competitor {
    let name: String = FirstName(EN).fake();
    let last_name: String = LastName(EN).fake();
    let age = random_number();
    Competitor::create(name, last_name, age)
}

fn random_number() ->i32 {
    random()
}
