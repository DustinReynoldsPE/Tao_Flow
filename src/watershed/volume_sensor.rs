use crate::water::rain::{Rain, Volume};

pub struct VolumeSensor {
    droplet_max: usize,
    shower_max: usize,
    downpour_max: usize,
}

impl VolumeSensor {
    pub fn new() -> Self {
        Self {
            droplet_max: 5,
            shower_max: 30,
            downpour_max: 100,
        }
    }

    pub fn sense(&self, rain: &Rain) -> Volume {
        let weight = rain.weight();
        if weight <= self.droplet_max {
            Volume::Droplet
        } else if weight <= self.shower_max {
            Volume::Shower
        } else if weight <= self.downpour_max {
            Volume::Downpour
        } else {
            Volume::Storm
        }
    }
}

impl Default for VolumeSensor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::Vapor;

    #[test]
    fn droplet_for_simple_input() {
        let sensor = VolumeSensor::new();
        let rain = Rain::new("hello", Vapor::default());
        assert_eq!(sensor.sense(&rain), Volume::Droplet);
    }

    #[test]
    fn shower_for_moderate_input() {
        let sensor = VolumeSensor::new();
        let rain = Rain::new(
            "Can you explain how async programming works in Rust?",
            Vapor::default(),
        );
        assert_eq!(sensor.sense(&rain), Volume::Shower);
    }

    #[test]
    fn downpour_for_complex_input() {
        let sensor = VolumeSensor::new();
        let rain = Rain::new("word ".repeat(50), Vapor::default());
        assert_eq!(sensor.sense(&rain), Volume::Downpour);
    }

    #[test]
    fn storm_for_transformative_input() {
        let sensor = VolumeSensor::new();
        let rain = Rain::new("word ".repeat(150), Vapor::default());
        assert_eq!(sensor.sense(&rain), Volume::Storm);
    }

    #[test]
    fn volume_progression_is_monotonic() {
        let sensor = VolumeSensor::new();
        let volumes: Vec<Volume> = [1, 5, 10, 30, 50, 100, 200]
            .iter()
            .map(|&n| {
                let rain = Rain::new("word ".repeat(n), Vapor::default());
                sensor.sense(&rain)
            })
            .collect();

        // Each volume should be >= the previous (conceptually)
        for window in volumes.windows(2) {
            let a = volume_rank(window[0]);
            let b = volume_rank(window[1]);
            assert!(b >= a, "{:?} should be >= {:?}", window[1], window[0]);
        }
    }

    fn volume_rank(v: Volume) -> u8 {
        match v {
            Volume::Droplet => 0,
            Volume::Shower => 1,
            Volume::Downpour => 2,
            Volume::Storm => 3,
        }
    }
}
