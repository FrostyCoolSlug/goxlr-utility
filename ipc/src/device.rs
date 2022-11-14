use enumset::EnumSet;
use goxlr_types::{
    ButtonColourOffStyle, ButtonColourTargets, ChannelName, CompressorAttackTime, CompressorRatio,
    CompressorReleaseTime, EchoStyle, EffectBankPresets, EncoderColourTargets, EqFrequencies,
    FaderDisplayStyle, FaderName, FirmwareVersions, GateTimes, GenderStyle, HardTuneSource,
    HardTuneStyle, InputDevice, MegaphoneStyle, MicrophoneType, MiniEqFrequencies, MuteFunction,
    OutputDevice, PitchStyle, ReverbStyle, RobotStyle, SampleBank, SampleButtons, SamplePlayOrder,
    SamplePlaybackMode, SamplerColourTargets, SimpleColourTargets,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use strum::EnumCount;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub daemon_version: String,
    pub mixers: HashMap<String, MixerStatus>,
    pub paths: Paths,
    pub files: Files,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixerStatus {
    pub hardware: HardwareStatus,
    pub fader_status: [FaderStatus; 4], // TODO: Does this need to be an array? :p
    pub mic_status: MicSettings,
    pub levels: Levels,
    pub router: [EnumSet<OutputDevice>; InputDevice::COUNT],
    pub router_table: [[bool; OutputDevice::COUNT]; InputDevice::COUNT],
    pub cough_button: CoughButton,
    pub lighting: Lighting,
    pub effects: Option<Effects>,
    pub sampler: Option<Sampler>,
    pub profile_name: String,
    pub mic_profile_name: String,
}

impl MixerStatus {
    pub fn get_fader_status(&self, fader: FaderName) -> &FaderStatus {
        &self.fader_status[fader as usize]
    }

    pub fn get_channel_volume(&self, channel: ChannelName) -> u8 {
        self.levels.volumes[channel as usize]
    }

    pub fn set_channel_volume(&mut self, channel: ChannelName, volume: u8) {
        self.levels.volumes[channel as usize] = volume;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareStatus {
    pub versions: FirmwareVersions,
    pub serial_number: String,
    pub manufactured_date: String,
    pub device_type: DeviceType,
    pub usb_device: UsbProductInformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderStatus {
    pub channel: ChannelName,
    pub mute_type: MuteFunction,
    pub scribble: Option<Scribble>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct CoughButton {
    pub is_toggle: bool,
    pub mute_type: MuteFunction,
}

impl Default for FaderStatus {
    fn default() -> Self {
        FaderStatus {
            channel: ChannelName::Mic,
            mute_type: MuteFunction::All,
            scribble: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicSettings {
    pub mic_type: MicrophoneType,
    pub mic_gains: [u16; MicrophoneType::COUNT],

    pub equaliser: Equaliser,
    pub equaliser_mini: EqualiserMini,
    pub noise_gate: NoiseGate,
    pub compressor: Compressor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Levels {
    pub volumes: [u8; ChannelName::COUNT],
    pub bleep: i8,
    pub deess: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equaliser {
    pub gain: HashMap<EqFrequencies, i8>,
    pub frequency: HashMap<EqFrequencies, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualiserMini {
    pub gain: HashMap<MiniEqFrequencies, i8>,
    pub frequency: HashMap<MiniEqFrequencies, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseGate {
    pub threshold: i8,
    pub attack: GateTimes,
    pub release: GateTimes,
    pub enabled: bool,
    pub attenuation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compressor {
    pub threshold: i8,
    pub ratio: CompressorRatio,
    pub attack: CompressorAttackTime,
    pub release: CompressorReleaseTime,
    pub makeup_gain: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lighting {
    pub faders: HashMap<FaderName, FaderLighting>,
    pub buttons: HashMap<ButtonColourTargets, ButtonLighting>,
    pub simple: HashMap<SimpleColourTargets, OneColour>,
    pub sampler: HashMap<SamplerColourTargets, SamplerLighting>,
    pub encoders: HashMap<EncoderColourTargets, ThreeColours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonLighting {
    pub off_style: ButtonColourOffStyle,
    pub colours: TwoColours,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerLighting {
    pub off_style: ButtonColourOffStyle,
    pub colours: ThreeColours,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderLighting {
    pub style: FaderDisplayStyle,
    pub colours: TwoColours,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneColour {
    pub colour_one: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoColours {
    pub colour_one: String,
    pub colour_two: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeColours {
    pub colour_one: String,
    pub colour_two: String,
    pub colour_three: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effects {
    pub active_preset: EffectBankPresets,
    pub preset_names: HashMap<EffectBankPresets, String>,
    pub current: ActiveEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEffects {
    pub reverb: Reverb,
    pub echo: Echo,
    pub pitch: Pitch,
    pub gender: Gender,
    pub megaphone: Megaphone,
    pub robot: Robot,
    pub hard_tune: HardTune,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reverb {
    pub style: ReverbStyle,
    pub amount: u8,
    pub decay: u16,
    pub early_level: i8,
    pub tail_level: i8,
    pub pre_delay: u8,
    pub lo_colour: i8,
    pub hi_colour: i8,
    pub hi_factor: i8,
    pub diffuse: i8,
    pub mod_speed: i8,
    pub mod_depth: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Echo {
    pub style: EchoStyle,
    pub amount: u8,
    pub feedback: u8,
    pub tempo: u16,
    pub delay_left: u16,
    pub delay_right: u16,
    pub feedback_left: u8,
    pub feedback_right: u8,
    pub feedback_xfb_l_to_r: u8,
    pub feedback_xfb_r_to_l: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pitch {
    pub style: PitchStyle,
    pub amount: i8,
    pub character: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gender {
    pub style: GenderStyle,
    pub amount: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Megaphone {
    pub is_enabled: bool,
    pub style: MegaphoneStyle,
    pub amount: u8,
    pub post_gain: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub is_enabled: bool,
    pub style: RobotStyle,
    pub low_gain: i8,
    pub low_freq: u8,
    pub low_width: u8,
    pub mid_gain: i8,
    pub mid_freq: u8,
    pub mid_width: u8,
    pub high_gain: i8,
    pub high_freq: u8,
    pub high_width: u8,
    pub waveform: u8,
    pub pulse_width: u8,
    pub threshold: i8,
    pub dry_mix: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardTune {
    pub is_enabled: bool,
    pub style: HardTuneStyle,
    pub amount: u8,
    pub rate: u8,
    pub window: u16,
    pub source: HardTuneSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sampler {
    pub banks: HashMap<SampleBank, HashMap<SampleButtons, SamplerButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerButton {
    pub function: SamplePlaybackMode,
    pub order: SamplePlayOrder,
    pub samples: Vec<Sample>,
    pub is_playing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub name: String,
    pub start_pct: f32,
    pub stop_pct: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paths {
    pub profile_directory: PathBuf,
    pub mic_profile_directory: PathBuf,
    pub samples_directory: PathBuf,
    pub presets_directory: PathBuf,
    pub icons_directory: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Files {
    pub profiles: HashSet<String>,
    pub mic_profiles: HashSet<String>,
    pub presets: HashSet<String>,
    pub samples: HashMap<String, String>,
    pub icons: HashSet<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Scribble {
    pub file_name: Option<String>,
    pub bottom_text: Option<String>,
    pub left_text: Option<String>,
    pub inverted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbProductInformation {
    pub manufacturer_name: String,
    pub product_name: String,
    pub version: (u8, u8, u8),
    pub bus_number: u8,
    pub address: u8,
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceType {
    Unknown,
    Full,
    Mini,
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Unknown
    }
}
