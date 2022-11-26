#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestyte: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestyte: LifeState::Alive,
        }
    }
}

impl Character {
    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }
}

fn main() {
    let npc_1 = Character::default();
    println!("{npc_1:?}")
}
