use std::collections::HashMap;

#[derive(Debug)]
struct FrostEffect {
    intensity: f64,
    duration: u32,
}

impl FrostEffect {
    pub fn new(intensity: f64, duration: u32) -> Self {
        FrostEffect { intensity, duration }
    }

    fn is_active(&self) -> bool {
        self.duration > 0 && self.intensity > 0.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut effects: HashMap<String, FrostEffect> = HashMap::new();
    
    effects.insert(
        String::from("arctic"),
        FrostEffect::new(0.8, 1000)
    );

    if let Some(effect) = effects.get("arctic") {
        println!("Arctic effect: {:?}", effect);
    }

    Ok(())
}