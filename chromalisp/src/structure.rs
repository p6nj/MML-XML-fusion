use std::time::Duration;

use derive_new::new;

/// Single item contained in Wrappers.<br>
/// The Note item has frequency information deduced from the note list but both have no length information
/// (taken from either a higher-level length Wrappers or the default value).
#[derive(PartialEq, Debug)]
pub enum Items {
    /// A single note with the index of the note found in the note list.
    Note(u8),
    /// A single rest.
    Rest,
}

#[derive(PartialEq, Debug)]
pub enum MaskItems {
    Note(i8),
    Rest,
}

/// Time relative to the duration of a note.
/// Can be either static (in ms) or dynamic (in note length percentage up to 255).
#[derive(PartialEq, Debug)]
pub enum Time {
    /// Static duration measured in milliseconds.
    Static(Duration),
    /// Dynamic duration measured in object length percentage up to 255.<br>
    /// Can also be used up to 100, allowing overload (110%).
    Dynamic(u8),
}

/// Repartition of the effect on the length of a nested note.
/// Dictates which side(s) of the note will be affected by an effect.<br><br>
/// <h4>Dev note:</h4>
/// <i>
/// Whole shouldn't be the same as a Start(Dynamic(255)) or something else as some effects have a different action on the start or end of the note and may act different if applied to a whole note. This way abstractions may be used to compare these two cases instead of using a normal attribute to theoretically speed up the process.
/// </i>
#[derive(PartialEq, Debug)]
pub enum Repartition {
    /// Whole note.
    Whole,
    /// Only the start of the note.
    Start(Time),
    /// Only the end of the note.
    End(Time),
    /// Both sides of the note.
    BothSides { start: Time, end: Time },
}

/// Generic ADSR member.
#[derive(new, PartialEq, Debug)]
pub struct AdsrComponent {
    /// Time it takes to get to the final value (`until`).
    /// Can be calculated as a percentage of the note length left with the Time enum.
    time: Time,
    /// Final value, percentage (255 max).
    until: u8,
}

impl Default for AdsrComponent {
    fn default() -> Self {
        AdsrComponent::new(Time::Static(Duration::from_secs(0)), 255)
    }
}

/// ADSR effect filter to control the volume of the note during a trigger.
/// A pizz. effect can be achieved with ADSR.
#[derive(new, PartialEq, Debug)]
pub struct ADSR {
    attack: AdsrComponent,
    decay: AdsrComponent,
    sustain: AdsrComponent,
    release: Duration,
}

/// Dynamics are used instead of a volume for simplicity.
/// FortePiano isn't included but can be achieved with ADSR.
#[derive(PartialEq, Debug)]
pub enum Dynamics {
    Pianississimo,
    Pianissimo,
    Piano,
    MezzoPiano,
    MezzoForte,
    Forte,
    Fortissimo,
    Fortississimo,
}

/// Instrument operations, allowing scalars to alter the frequency.
#[derive(PartialEq, Debug)]
pub enum Operations {
    /// Multiply the frequency.
    Multiply(u8),
    /// Divide the frequency.
    Divide(u8),
    /// Add a number to the frequency (offset).
    /// You can add negative numbers for subtraction.
    Add(u16),
}

impl Default for Tuning {
    fn default() -> Self {
        Tuning::new_static(255)
    }
}

/// Instrument tuning.
#[derive(new, PartialEq, Debug)]
pub enum Tuning {
    /// Static tuning without dependencies.
    Static { tuning: u16 },
    /// Relative tuning depending on the previous instrument tuning.
    Relative { relation: Operations },
}

/// Instruments collection.
#[derive(PartialEq, Debug)]
pub enum Instruments {
    /// Basic sine wave.
    Sine(Tuning),
    /// Basic square wave.
    Square(Tuning),
    /// Basic triangle wave.
    Triangle(Tuning),
    /// Bounded white noise. Low and high are the percentage (/255) of low and high spectrum space allowed.<br>
    /// Unbounded white noise has theoretically no recognizable tune so no note.
    Noise { tuning: Tuning, low: u8, high: u8 },
    /// Waveform product between two boxed instruments.
    Product(Box<(Instruments, Instruments)>),
    /// Waveform sum between two boxed instruments (combines two waveforms).
    Sum(Box<(Instruments, Instruments)>),
}

/// Vibrato configuration.
#[derive(new, PartialEq, Debug)]
pub struct VibratoConfig {
    /// Amplitude of the vibrato, in tenths of a frequency whole interval (10 means it will touch the whole note above and below).
    amplitude: u8,
    /// Frequency of the vibrato in hertz.
    frequency: u8,
}

/// Volume fader configuration.
#[derive(new, PartialEq, Debug)]
pub struct VolumeFadeConfig {
    /// Initial volume.
    from: Dynamics,
    /// Final volume.
    to: Dynamics,
    /// States if volume changes will be applied inside a note duration or not.
    inside: bool,
}

/// Acceleration configuration
#[derive(PartialEq, Debug)]
pub struct AccelConfig {
    /// Initial tempo.
    from: u16,
    /// Final tempo.
    to: u16,
}

#[derive(PartialEq, Debug)]
pub enum MaskType {
    Rhythm,
    Note,
}

/// Mask with a layer to apply on top of a note or multiple notes.
/// Can either apply the rhythm on notes identified by their index in the masked scope or
/// rhythm and notes applied either on all input notes (there must be as much input notes as
/// mask notes) or one input note (to apply i.e. a tremolo effect).<br>
/// A note applied on another note transposes it by addition (applying a 3 on a 1 gives a 4).
#[derive(PartialEq, Debug)]
pub struct Mask {
    id: u8,
    layer: Vec<MaskItems>,
}

/// Wrappers that can either contain an Item or another Wrappers with more information based on the variant.
#[derive(PartialEq, Debug)]
pub enum Wrappers {
    /// The title of the underlying song.
    Song(String, Vec<Wrappers>),
    /// The title of the underlying album.
    Album(String, Vec<Wrappers>),
    /// The name of the artist responsible for the underlying notes.
    Artist(String, Vec<Wrappers>),
    /// The year of the underlying song or album.
    Year(u16, Vec<Wrappers>),
    /// Tempo in BPM.
    Tempo(u16, Vec<Wrappers>),
    /// Tempo acceleration.
    Accel(AccelConfig, Vec<Wrappers>),
    /// The notes names used for the interface.
    NoteDef(Vec<String>, Vec<Wrappers>),
    /// The channel name for the underlying notes.
    Channel(Vec<Wrappers>),
    /// Instrument used for the underlying notes.
    Instrument(Instruments, Vec<Wrappers>),
    /// Length of the underlying notes described by the number of notes in a beat.
    Length(u8, Vec<Wrappers>),
    /// Octave of the underlying notes.
    Octave(u8, Vec<Wrappers>),
    /// Loops the underlying notes.
    Loop(u8, Vec<Wrappers>),
    /// Apply a glissando effect on some part of the underlying notes length.
    Glissando(Repartition, Vec<Wrappers>),
    /// Vibrato effect for some part of the underlying notes length.
    Vibrato(Repartition, VibratoConfig, Vec<Wrappers>),
    /// Volume applied to underlying notes.
    Volume(Dynamics, Vec<Wrappers>),
    /// Volume fade applied to underlying notes.
    VolumeFader(VolumeFadeConfig, Vec<Wrappers>),
    /// ADSR applies a volume envelope to underlying notes.
    ADSR(ADSR, Vec<Wrappers>),
    /// Single note or rest, the atom of the DOM.
    Singleton(Items),
    /// Mask definition.
    Mask(MaskType, Mask),
    /// Masks items using the mask ID.
    Masked(u8, Vec<Wrappers>),
    Test(String, Vec<Wrappers>),
    Test2(String, Vec<Wrappers>),
}

pub trait Tagging {
    fn tag(&self) -> char;
}

impl Tagging for Wrappers {
    fn tag(&self) -> char {
        match self {
            Wrappers::Song(_, _) => 'S',
            Wrappers::Album(_, _) => 'A',
            Wrappers::Artist(_, _) => 'a',
            Wrappers::Year(_, _) => 'Y',
            Wrappers::Tempo(_, _) => 'T',
            Wrappers::Accel(_, _) => 't',
            Wrappers::NoteDef(_, _) => 'N',
            Wrappers::Channel(_) => 'C',
            Wrappers::Instrument(_, _) => 'I',
            Wrappers::Length(_, _) => 'l',
            Wrappers::Octave(_, _) => 'o',
            Wrappers::Loop(_, _) => 'x',
            Wrappers::Glissando(_, _) => 'G',
            Wrappers::Vibrato(_, _, _) => 'V',
            Wrappers::Volume(_, _) => 'v',
            Wrappers::VolumeFader(_, _) => 'F',
            Wrappers::ADSR(_, _) => '~',
            Wrappers::Singleton(_) => unimplemented!(),
            Wrappers::Mask(_, _) => 'M',
            Wrappers::Masked(_, _) => 'm',
            Wrappers::Test(_, _) => 'Y',
            Wrappers::Test2(_, _) => 'A',
        }
    }
}
