fn main() {
    let rocket = Rocket::default();
    let launched = launch_rocket(&rocket);
    println!("rocket {} in orbit", launched.name);
}

pub trait Launchable {}
struct Grounded;
struct Launched;

impl Launchable for Grounded {}

// marker trait Launchable used here
fn launch_rocket<S>(rocket: &Rocket<S>) -> Rocket<Launched>
where
    S: Launchable,
{
    println!("Launching rocket {}", rocket.name);
    Rocket {
        stage: std::marker::PhantomData::<Launched>,
        name: rocket.name.clone(),
    }
}

struct Rocket<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>,
    name: String,
}

impl Default for Rocket {
    fn default() -> Self {
        Rocket {
            stage: std::marker::PhantomData::<Grounded>,
            name: "R2D2".to_string(),
        }
    }
}

impl<Stage> Rocket<Stage> {
    pub fn check_systems(&self) {
        println!("checking systems");
    }
}
