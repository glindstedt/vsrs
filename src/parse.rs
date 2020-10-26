use std::collections::HashMap;

use korg_syro::{pattern, pattern::num_enum::TryFromPrimitive};
use log::{debug, trace};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SampleDef {
    pub file: String,
    pub compression: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub enum SampleAction {
    Sample(SampleDef),
    Erase,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToggleDef {
    On,
    Off
}

impl Into<korg_syro::pattern::Toggle> for ToggleDef {
    fn into(self) -> korg_syro::pattern::Toggle {
        use korg_syro::pattern::Toggle;
        match self {
            ToggleDef::On => Toggle::On,
            ToggleDef::Off => Toggle::Off,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PartDef {
    pub sample: u32,
    pub steps: Vec<u32>,
    pub motion: Option<ToggleDef>,
    #[serde(rename = "loop")]
    pub looped: Option<ToggleDef>,
    pub reverb: Option<ToggleDef>,
    pub reverse: Option<ToggleDef>,
    pub mute: Option<ToggleDef>,
}

#[derive(Debug, Deserialize)]
pub struct PatternDef {
    pub parts: HashMap<u32, PartDef>,
}

// TODO validation
#[derive(Debug, Deserialize)]
pub struct VolcaSample {
    // Default compression to apply for all
    pub default_compression: Option<u32>,
    pub samples: Option<HashMap<u32, SampleAction>>,
    pub patterns: Option<HashMap<u32, PatternDef>>,
}

pub fn parse_steps_definition(steps_definition: &[u32]) -> anyhow::Result<pattern::Steps> {
    let mut steps = pattern::Steps::builder();
    for (index, value) in steps_definition.iter().enumerate() {
        let step = pattern::Step::try_from_primitive(index as u8)?;
        if *value == 1 {
            steps.on(step);
        }
    }
    Ok(steps)
}

pub fn parse_part_definition(part_definition: &PartDef) -> anyhow::Result<pattern::Part> {
    let mut part = pattern::Part::for_sample(part_definition.sample as u16)?;
    let steps = parse_steps_definition(&part_definition.steps)?;
    part.with_steps(steps);
    if let Some(motion) = part_definition.motion {
        part.motion(motion.into());
    }
    if let Some(looped) = part_definition.looped {
        part.looped(looped.into());
    }
    if let Some(reverb) = part_definition.reverb {
        part.reverb(reverb.into());
    }
    if let Some(reverse) = part_definition.reverse {
        part.reverse(reverse.into());
    }
    if let Some(mute) = part_definition.mute {
        part.mute(mute.into());
    }
    debug!("{:?}", part);
    Ok(part)
}

pub fn parse_pattern_definition(
    pattern_index: u32,
    pattern_definition: &PatternDef,
) -> anyhow::Result<pattern::Pattern> {
    let mut pattern = pattern::Pattern::default();
    for (part_index, part_definition) in pattern_definition.parts.iter() {
        debug!("Part Definition {}: {:?}", part_index, part_definition);
        let part = parse_part_definition(part_definition)?;
        pattern.with_part(*part_index as u8, part)?;
    }
    trace!("Pattern {}: {:?}", pattern_index, pattern);
    Ok(pattern)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow;
    use ron::de::from_str;

    #[test]
    fn test_parse_steps_definition() -> anyhow::Result<()> {
        let steps_def = [1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0];
        let steps = parse_steps_definition(&steps_def)?;
        println!("{:016b}", steps.to_bytes());
        assert_eq!(steps.to_bytes(), 0b0001000101010111);
        Ok(())
    }

    #[test]
    fn test_pattern() -> anyhow::Result<()> {
        let ron_data = r#"
        #![enable(implicit_some)]
        VolcaSample(
            samples: {},
            patterns: {
                0: (
                    parts: {
                        0: (
                            sample: 0,
                            steps: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
                            loop: on,
                            reverb: off,
                            reverse: on,
                            motion: off,
                            mute: off,
                        ),
                        1: (
                            sample: 1,
                            steps: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                            loop: off,
                            reverb: on,
                            reverse: off,
                            motion: on,
                            mute: on,
                        ),
                        2: (
                            sample: 2,
                            steps: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                        )
                    }
                )
            },
        )
        "#;

        let parsed = from_str::<VolcaSample>(ron_data)?;

        let patterns: Vec<pattern::Pattern> = parsed
            .patterns
            .unwrap()
            .iter()
            .map(|(i, pd)| parse_pattern_definition(*i, pd).unwrap())
            .collect();

        println!("{:?}", patterns);

        // println!("{:?}", parsed);
        Ok(())
    }
}
