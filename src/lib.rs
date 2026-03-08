//! # Tao Flow
//!
//! A multi-LLM system that flows like water.
//!
//! *"The supreme good is like water, which nourishes all things without trying to."*
//! -- Tao Te Ching, Chapter 8

pub mod config;
pub mod confluence;
pub mod creation;
pub mod error;
pub mod flow;
pub mod still_lake;
pub mod vessel;
pub mod water;
pub mod watershed;

pub use error::FlowError;
pub use flow::TaoFlow;
pub use water::{Ocean, Rain, River, Stream, Vapor};

#[cfg(test)]
mod tests {
    use super::*;
    use water::rain::Volume;
    use water::{Message, Role};

    /// The complete journey: Rain -> Stream -> River -> Ocean.
    /// Each type is distinct. The compiler enforces the path.
    #[test]
    fn water_types_are_distinct() {
        let vapor = Vapor::default();
        let rain = Rain::new("What is the Tao?", vapor);
        let stream = Stream::new("mountain", "The Tao that can be told...");
        let river = River::from_single(stream.source.clone(), stream.content.clone());
        let ocean = Ocean::new(river.content.clone());

        // Each type carries its identity
        assert_eq!(rain.raw_input, "What is the Tao?");
        assert_eq!(stream.source, "mountain");
        assert_eq!(river.tributary_count(), 1);
        assert!(ocean.has_substance());
    }

    /// The water cycle: ocean becomes vapor for the next rain.
    #[test]
    fn water_cycle_completes() {
        let mut vapor = Vapor::default();

        // First cycle
        let rain = Rain::new("Hello", vapor.clone());
        let ocean = Ocean::new("Greetings, traveler.");

        // Ocean evaporates into vapor
        vapor.conversation_history.push(Message {
            role: Role::User,
            content: rain.raw_input.clone(),
        });
        vapor.conversation_history.push(Message {
            role: Role::Assistant,
            content: ocean.content.clone(),
        });

        // Second cycle carries the vapor from the first
        let rain2 = Rain::new("What did I just say?", vapor);
        assert_eq!(rain2.vapor.conversation_history.len(), 2);
        assert_eq!(rain2.vapor.conversation_history[0].content, "Hello");
    }

    /// Volume determines the depth of the watershed's response.
    #[test]
    fn volume_determines_depth() {
        let sensor = watershed::VolumeSensor::new();

        let droplet = Rain::new("hi", Vapor::default());
        let storm = Rain::new("word ".repeat(200), Vapor::default());

        assert_eq!(sensor.sense(&droplet), Volume::Droplet);
        assert_eq!(sensor.sense(&storm), Volume::Storm);
    }
}
