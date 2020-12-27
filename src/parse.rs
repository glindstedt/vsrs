use std::collections::HashMap;
use std::convert::TryInto;

use anyhow::Context;
use korg_syro::{pattern, pattern::num_enum::TryFromPrimitive};
use log::{debug, trace};
use pattern::Toggle;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleDef {
    pub file: String,
    pub compression: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SampleAction {
    Sample(SampleDef),
    Erase,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum ToggleDef {
    On,
    Off,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartDef {
    pub sample: u32,
    pub steps: Vec<u32>,
    pub motion: Option<ToggleDef>,
    #[serde(rename = "loop")]
    pub looped: Option<ToggleDef>,
    pub reverb: Option<ToggleDef>,
    pub reverse: Option<ToggleDef>,
    pub mute: Option<ToggleDef>,
    pub level: Option<u8>,
    pub pan: Option<u8>,
    pub amp_eg_attack: Option<u8>,
    pub amp_eg_decay: Option<u8>,
    pub pitch_eg_attack: Option<u8>,
    pub pitch_eg_int: Option<u8>,
    pub pitch_eg_decay: Option<u8>,
    pub starting_point: Option<u8>,
    pub length: Option<u8>,
    pub hi_cut: Option<u8>,
    pub motion_sequences: Option<MotionSequencesDef>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MotionSequencesDef {
    pub level_start: Option<Vec<u8>>,
    pub level_end: Option<Vec<u8>>,
    pub pan_start: Option<Vec<u8>>,
    pub pan_end: Option<Vec<u8>>,
    pub speed_start: Option<Vec<u8>>,
    pub speed_end: Option<Vec<u8>>,
    pub amp_eg_attack: Option<Vec<u8>>,
    pub amp_eg_decay: Option<Vec<u8>>,
    pub pitch_eg_int: Option<Vec<u8>>,
    pub pitch_eg_attack: Option<Vec<u8>>,
    pub pitch_eg_decay: Option<Vec<u8>>,
    pub start_point: Option<Vec<u8>>,
    pub length: Option<Vec<u8>>,
    pub hi_cut: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatternDef {
    pub parts: HashMap<u32, PartDef>,
}

// TODO validation
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VolcaSample {
    // Default compression to apply for all
    pub default_compression: Option<u32>,
    pub samples: Option<HashMap<u32, SampleAction>>,
    pub default_part_reverb: Option<ToggleDef>,
    pub patterns: Option<HashMap<u32, PatternDef>>,
}

trait VecU32Ext {
    fn into_steps(&self) -> anyhow::Result<pattern::Steps>;
}

impl VecU32Ext for Vec<u32> {
    fn into_steps(&self) -> anyhow::Result<pattern::Steps> {
        let mut steps = pattern::Steps::builder();
        for (index, value) in self.iter().enumerate() {
            let step = pattern::Step::try_from_primitive(index as u8)?;
            if *value == 1 {
                steps.on(step);
            }
        }
        Ok(steps)
    }
}

trait VecU8Ext {
    fn into_motion_seq(&self) -> anyhow::Result<[u8; 16]>;
}

impl VecU8Ext for Vec<u8> {
    fn into_motion_seq(&self) -> anyhow::Result<[u8; 16]> {
        self.clone()
            .as_slice()
            .try_into()
            .with_context(|| "unable to parse into motion sequence")
    }
}

pub fn parse_part_definition(
    part_definition: &PartDef,
    default_reverb: ToggleDef,
) -> anyhow::Result<pattern::Part> {
    let mut part = pattern::Part::for_sample(part_definition.sample as u16)?;
    let steps = part_definition.steps.into_steps()?;
    part.with_steps(steps);
    if let Some(motion) = part_definition.motion {
        part.motion(motion.into());
    }
    if let Some(looped) = part_definition.looped {
        part.looped(looped.into());
    }
    if let Some(reverb) = part_definition.reverb {
        part.reverb(reverb.into());
    } else {
        part.reverb(default_reverb.into());
    }
    if let Some(reverse) = part_definition.reverse {
        part.reverse(reverse.into());
    }
    if let Some(mute) = part_definition.mute {
        part.mute(mute.into());
    }
    if let Some(level) = part_definition.level {
        part.level(level)?;
    }
    if let Some(pan) = part_definition.pan {
        part.pan(pan)?;
    }
    if let Some(amp_eg_attack) = part_definition.amp_eg_attack {
        part.amp_eg_attack(amp_eg_attack)?;
    }
    if let Some(amp_eg_decay) = part_definition.amp_eg_decay {
        part.amp_eg_decay(amp_eg_decay)?;
    }
    if let Some(pitch_eg_attack) = part_definition.pitch_eg_attack {
        part.pitch_eg_attack(pitch_eg_attack)?;
    }
    if let Some(pitch_eg_int) = part_definition.pitch_eg_int {
        part.pitch_eg_int(pitch_eg_int)?;
    }
    if let Some(pitch_eg_decay) = part_definition.pitch_eg_decay {
        part.pitch_eg_decay(pitch_eg_decay)?;
    }
    if let Some(starting_point) = part_definition.starting_point {
        part.starting_point(starting_point)?;
    }
    if let Some(length) = part_definition.length {
        part.length(length)?;
    }
    if let Some(hi_cut) = part_definition.hi_cut {
        part.hi_cut(hi_cut)?;
    }
    if let Some(motion_seqs) = &part_definition.motion_sequences {
        if let Some(level_start) = motion_seqs.level_start.as_ref() {
            part.level_start_motion_seq(level_start.into_motion_seq()?)?;
        }
        if let Some(level_end) = motion_seqs.level_end.as_ref() {
            part.level_end_motion_seq(level_end.into_motion_seq()?)?;
        }
        if let Some(pan_start) = motion_seqs.pan_start.as_ref() {
            part.pan_start_motion_seq(pan_start.into_motion_seq()?)?;
        }
        if let Some(pan_end) = motion_seqs.pan_end.as_ref() {
            part.pan_end_motion_seq(pan_end.into_motion_seq()?)?;
        }
        if let Some(speed_start) = motion_seqs.speed_start.as_ref() {
            part.speed_start_motion_seq(speed_start.into_motion_seq()?)?;
        }
        if let Some(speed_end) = motion_seqs.speed_end.as_ref() {
            part.speed_end_motion_seq(speed_end.into_motion_seq()?)?;
        }
        if let Some(amp_eg_attack) = motion_seqs.amp_eg_attack.as_ref() {
            part.amp_eg_attack_motion_seq(amp_eg_attack.into_motion_seq()?)?;
        }
        if let Some(amp_eg_decay) = motion_seqs.amp_eg_decay.as_ref() {
            part.amp_eg_decay_motion_seq(amp_eg_decay.into_motion_seq()?)?;
        }
        if let Some(pitch_eg_int) = motion_seqs.pitch_eg_int.as_ref() {
            part.pitch_eg_int_motion_seq(pitch_eg_int.into_motion_seq()?)?;
        }
        if let Some(pitch_eg_attack) = motion_seqs.pitch_eg_attack.as_ref() {
            part.pitch_eg_attack_motion_seq(pitch_eg_attack.into_motion_seq()?)?;
        }
        if let Some(pitch_eg_decay) = motion_seqs.pitch_eg_decay.as_ref() {
            part.pitch_eg_decay_motion_seq(pitch_eg_decay.into_motion_seq()?)?;
        }
        if let Some(start_point) = motion_seqs.start_point.as_ref() {
            part.start_point_motion_seq(start_point.into_motion_seq()?)?;
        }
        if let Some(length) = motion_seqs.length.as_ref() {
            part.length_motion_seq(length.into_motion_seq()?)?;
        }
        if let Some(hi_cut) = motion_seqs.hi_cut.as_ref() {
            part.hi_cut_motion_seq(hi_cut.into_motion_seq()?)?;
        }
    }
    debug!("{:?}", part);
    Ok(part)
}

pub fn parse_pattern_definition(
    pattern_index: u32,
    pattern_definition: &PatternDef,
    default_reverb: ToggleDef,
) -> anyhow::Result<pattern::Pattern> {
    let mut pattern = pattern::Pattern::default();
    for (part_index, part_definition) in pattern_definition.parts.iter() {
        debug!("Part Definition {}: {:?}", part_index, part_definition);
        let part = parse_part_definition(part_definition, default_reverb)?;
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
        let steps_def: Vec<u32> = vec![1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0];
        let steps = steps_def.into_steps()?;
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
                            motion_sequences: (
                                level_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                level_end: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                pan_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                pan_end: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                speed_start: [40, 43, 46, 49, 52, 55, 58, 61, 64, 67, 70, 73, 76, 79, 82, 85],
                                speed_end: [129, 137, 145, 153, 161, 169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249],
                                amp_eg_attack: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                amp_eg_decay: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                pitch_eg_int: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                pitch_eg_attack: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                pitch_eg_decay: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                start_point: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                length: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                                hi_cut: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                            )
                        ),
                        1: (
                            sample: 1,
                            steps: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                            loop: off,
                            reverb: on,
                            reverse: off,
                            motion: on,
                            mute: on,
                            motion_sequences: (
                                level_start: [1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120],
                            )
                        ),
                        2: (
                            sample: 2,
                            level: 64,
                            pan: 127,
                            speed: 192,
                            amp_eg_attack: 64,
                            amp_eg_decay: 64,
                            pitch_eg_attack: 64,
                            pitch_eg_int: 64,
                            pitch_eg_decay: 64,
                            starting_point: 64,
                            length: 64,
                            hi_cut: 64,
                            steps: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                        )
                    }
                )
            },
        )
        "#;

        let json_data = r#"
        {
          "samples": {},
          "patterns": {
            "0": {
              "parts": {
                "0": {
                  "sample": 0,
                  "steps": [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1 ],
                  "motion": "off",
                  "loop": "on",
                  "reverb": "off",
                  "reverse": "on",
                  "mute": "off",
                  "motion_sequences": {
                    "level_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "level_end": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "pan_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "pan_end": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "speed_start": [ 40, 43, 46, 49, 52, 55, 58, 61, 64, 67, 70, 73, 76, 79, 82, 85 ],
                    "speed_end": [ 129, 137, 145, 153, 161, 169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249 ],
                    "amp_eg_attack": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "amp_eg_decay": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "pitch_eg_int": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "pitch_eg_attack": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "pitch_eg_decay": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "start_point": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "length": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ],
                    "hi_cut": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ]
                  }
                },
                "1": {
                  "sample": 1,
                  "steps": [ 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0 ],
                  "motion": "on",
                  "loop": "off",
                  "reverb": "on",
                  "reverse": "off",
                  "mute": "on",
                  "motion_sequences": {
                    "level_start": [ 1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120 ]
                  }
                },
                "2": {
                  "sample": 2,
                  "level": 64,
                  "pan": 127,
                  "speed": 192,
                  "amp_eg_attack": 64,
                  "amp_eg_decay": 64,
                  "pitch_eg_attack": 64,
                  "pitch_eg_int": 64,
                  "pitch_eg_decay": 64,
                  "starting_point": 64,
                  "length": 64,
                  "hi_cut": 64,
                  "steps": [ 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0 ]
                }
              }
            }
          }
        }
        "#;

        let parsed_ron = from_str::<VolcaSample>(ron_data)?;
        let parsed_json = serde_json::from_str::<VolcaSample>(json_data)?;

        assert_eq!(parsed_ron, parsed_json);

        let patterns: Vec<pattern::Pattern> = parsed_ron
            .patterns
            .unwrap()
            .iter()
            .map(|(i, pd)| parse_pattern_definition(*i, pd, ToggleDef::Off).unwrap())
            .collect();

        println!("{:?}", patterns);

        // println!("{:?}", parsed);
        Ok(())
    }
}
