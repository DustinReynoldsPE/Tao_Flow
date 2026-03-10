use serde::{Deserialize, Serialize};

use crate::water::{River, Stream};

/// A pearl is the layered record of a single flow through the system.
///
/// Like a natural pearl, it forms around a core (the question) with
/// each layer of processing adding nacre. For Storm flows, sub-pearls
/// nest inside — one per sub-question, each a complete flow record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pearl {
    /// The original question (rain input).
    pub core: String,
    /// Spring responses before merging.
    pub streams: Vec<Stream>,
    /// Merged output from confluence (carries eddies, clarity, tributaries).
    pub river: Option<River>,
    /// What the user received.
    pub ocean: String,
    /// Nested pearls from Storm decomposition.
    pub sub_pearls: Vec<Pearl>,
}

impl Pearl {
    pub fn new(
        core: impl Into<String>,
        streams: Vec<Stream>,
        river: Option<River>,
        ocean: impl Into<String>,
    ) -> Self {
        Self {
            core: core.into(),
            streams,
            river,
            ocean: ocean.into(),
            sub_pearls: Vec::new(),
        }
    }

    pub fn with_sub_pearls(mut self, sub_pearls: Vec<Pearl>) -> Self {
        self.sub_pearls = sub_pearls;
        self
    }

    /// Write the pearl as a folder structure under `.storms/`.
    /// Failures are silent — the pearl is a record, not a requirement.
    pub fn write(&self) {
        let name = format!("{}-{}", self.slug(), timestamp());
        let dir = format!(".storms/{name}");
        if std::fs::create_dir_all(&dir).is_err() {
            return;
        }
        self.write_to_dir(&dir);
    }

    /// Write this pearl's layers into the given directory.
    pub(crate) fn write_to_dir(&self, dir: &str) {
        // core.md — the question, nothing else
        std::fs::write(format!("{dir}/core.md"), &self.core).ok();

        // streams/ — each spring's voice, unmodified
        if !self.streams.is_empty() {
            let streams_dir = format!("{dir}/streams");
            if std::fs::create_dir_all(&streams_dir).is_ok() {
                for stream in &self.streams {
                    std::fs::write(
                        format!("{streams_dir}/{}.md", stream.source),
                        &stream.content,
                    )
                    .ok();
                }
            }
        }

        // river.md — YAML frontmatter (metadata) + woven content
        if let Some(ref river) = self.river {
            let mut content = format!(
                "---\nclarity: {:.2}\ntributaries: [{}]\n",
                river.clarity,
                river.tributaries.join(", ")
            );

            if river.has_eddies() {
                content.push_str("eddies:\n");
                for eddy in &river.eddies {
                    let status = if let Some(ref r) = eddy.resolution {
                        format!("resolved: \"{}\"", r.synthesis)
                    } else {
                        "resolved: false".to_string()
                    };
                    content.push_str(&format!(
                        "  - topic: \"{}\"\n    nature: {:?}\n    {}\n",
                        eddy.topic, eddy.nature, status
                    ));
                }
            }

            content.push_str("---\n\n");
            content.push_str(&river.content);
            std::fs::write(format!("{dir}/river.md"), content).ok();
        }

        // ocean.md — what the user received
        std::fs::write(format!("{dir}/ocean.md"), &self.ocean).ok();

        // sub-pearls/ — nested folders for Storm decomposition
        if !self.sub_pearls.is_empty() {
            let sub_dir = format!("{dir}/sub-pearls");
            if std::fs::create_dir_all(&sub_dir).is_ok() {
                for (i, sub) in self.sub_pearls.iter().enumerate() {
                    let sub_slug = sub.slug();
                    let sub_path = format!("{sub_dir}/{:02}-{sub_slug}", i + 1);
                    if std::fs::create_dir_all(&sub_path).is_ok() {
                        sub.write_to_dir(&sub_path);
                    }
                }
            }
        }

        // pearl.json — full structured data for programmatic access
        if let Ok(json) = serde_json::to_string_pretty(self) {
            std::fs::write(format!("{dir}/pearl.json"), json).ok();
        }
    }

    pub(crate) fn slug(&self) -> String {
        self.core
            .split_whitespace()
            .take(6)
            .collect::<Vec<_>>()
            .join("-")
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }
}

/// UTC timestamp as YYYYMMDD-HHMMSS (no external dependency).
fn timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;

    // Civil date from days since epoch (Howard Hinnant's algorithm)
    let days = (secs / 86400) as i64;
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };

    format!("{y:04}{mo:02}{d:02}-{h:02}{m:02}{s:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::River;

    #[test]
    fn pearl_forms_from_single_stream() {
        let stream = Stream::new("desert", "Quick answer.");
        let river = River::from_single("desert".into(), "Quick answer.".into());
        let pearl = Pearl::new("hello", vec![stream], Some(river), "Quick answer.");

        assert_eq!(pearl.core, "hello");
        assert_eq!(pearl.streams.len(), 1);
        assert!(pearl.sub_pearls.is_empty());
    }

    #[test]
    fn pearl_nests_sub_pearls() {
        let sub1 = Pearl::new("sub question 1", vec![], None, "answer 1");
        let sub2 = Pearl::new("sub question 2", vec![], None, "answer 2");
        let pearl = Pearl::new("big question", vec![], None, "combined answer")
            .with_sub_pearls(vec![sub1, sub2]);

        assert_eq!(pearl.sub_pearls.len(), 2);
        assert_eq!(pearl.sub_pearls[0].core, "sub question 1");
    }

    #[test]
    fn pearl_serializes() {
        let stream = Stream::new("mountain", "Deep analysis.");
        let pearl = Pearl::new("What is the Tao?", vec![stream], None, "The way.");

        let json = serde_json::to_string(&pearl).unwrap();
        let restored: Pearl = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.core, "What is the Tao?");
        assert_eq!(restored.streams.len(), 1);
    }

    #[test]
    fn slug_from_input() {
        let pearl = Pearl::new("What is the nature of water?", vec![], None, "");
        assert_eq!(pearl.slug(), "what-is-the-nature-of-water");
    }

    #[test]
    fn slug_truncates_long_input() {
        let pearl = Pearl::new(
            "one two three four five six seven eight nine ten",
            vec![],
            None,
            "",
        );
        assert_eq!(pearl.slug(), "one-two-three-four-five-six");
    }

    #[test]
    fn timestamp_is_valid_format() {
        let ts = timestamp();
        assert_eq!(ts.len(), 15); // YYYYMMDD-HHMMSS
        assert_eq!(ts.as_bytes()[8], b'-');
    }

    #[test]
    fn pearl_writes_folder_structure() {
        let stream = Stream::new("desert", "Quick answer.");
        let river = River::from_single("desert".into(), "Quick answer.".into());
        let pearl = Pearl::new("folder test", vec![stream], Some(river), "Settled.");

        let dir = ".storms/_test_folder";
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap();
        pearl.write_to_dir(dir);

        assert!(std::path::Path::new(&format!("{dir}/core.md")).exists());
        assert!(std::path::Path::new(&format!("{dir}/ocean.md")).exists());
        assert!(std::path::Path::new(&format!("{dir}/river.md")).exists());
        assert!(std::path::Path::new(&format!("{dir}/streams/desert.md")).exists());
        assert!(std::path::Path::new(&format!("{dir}/pearl.json")).exists());

        assert_eq!(
            std::fs::read_to_string(format!("{dir}/core.md")).unwrap(),
            "folder test"
        );
        assert_eq!(
            std::fs::read_to_string(format!("{dir}/ocean.md")).unwrap(),
            "Settled."
        );
        assert_eq!(
            std::fs::read_to_string(format!("{dir}/streams/desert.md")).unwrap(),
            "Quick answer."
        );

        let river_content = std::fs::read_to_string(format!("{dir}/river.md")).unwrap();
        assert!(river_content.contains("clarity: 1.00"));
        assert!(river_content.contains("tributaries: [desert]"));

        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn pearl_writes_nested_sub_pearls() {
        let sub1 = Pearl::new(
            "sub question one",
            vec![Stream::new("mountain", "Deep.")],
            None,
            "answer one",
        );
        let sub2 = Pearl::new("sub question two", vec![], None, "answer two");
        let pearl = Pearl::new("storm test", vec![], None, "final answer")
            .with_sub_pearls(vec![sub1, sub2]);

        let dir = ".storms/_test_nested";
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap();
        pearl.write_to_dir(dir);

        // Sub-pearl folders exist with numbered prefixes
        let sp1 = format!("{dir}/sub-pearls/01-sub-question-one");
        let sp2 = format!("{dir}/sub-pearls/02-sub-question-two");
        assert!(std::path::Path::new(&sp1).is_dir());
        assert!(std::path::Path::new(&sp2).is_dir());

        // Each sub-pearl has its own layers
        assert_eq!(
            std::fs::read_to_string(format!("{sp1}/core.md")).unwrap(),
            "sub question one"
        );
        assert_eq!(
            std::fs::read_to_string(format!("{sp1}/ocean.md")).unwrap(),
            "answer one"
        );
        assert!(std::path::Path::new(&format!("{sp1}/streams/mountain.md")).exists());

        assert_eq!(
            std::fs::read_to_string(format!("{sp2}/core.md")).unwrap(),
            "sub question two"
        );

        // Top level
        assert_eq!(
            std::fs::read_to_string(format!("{dir}/ocean.md")).unwrap(),
            "final answer"
        );

        std::fs::remove_dir_all(dir).ok();
    }
}
