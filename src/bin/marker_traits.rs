fn main() {
    let rocket = Rocket::default();
    rocket.check_systems();
    let launched = rocket.launch();
    launched.navigate_to("Mars");
}

struct Grounded;
struct Launched;

struct Rocket<Stage = Grounded> {
    pub stage: std::marker::PhantomData<Stage>,
    pub accelleration: f32,
}

impl<Stage> Rocket<Stage> {
    pub fn check_systems(&self) {
        println!("checking systems");
    }
}

impl Default for Rocket {
    fn default() -> Self {
        Rocket {
            stage: std::marker::PhantomData::<Grounded>,
            accelleration: 0.1,
        }
    }
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        println!("launching rocket");
        Rocket {
            stage: std::marker::PhantomData::<Launched>,
            accelleration: self.accelleration + 0.5,
        }
    }
}

impl Rocket<Launched> {
    pub fn navigate_to(&self, destination: &str) {
        println!("rocket navigating to: {}", destination);
    }
}
