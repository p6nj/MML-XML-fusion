use std::time::Duration;

use derive_new::new;

/// Single item contained in Wrappers.<br>
/// The Note item has frequency information deduced from the note list but both have no length information
/// (taken from either a higher-level length Wrappers or the default value).
pub enum Items {
    /// A single note with the index of the note found in the note list.
    Note(u8),
    /// A single rest.
    Rest,
}

/// Slice of the duration of a note, used only for Repartition.
/// Can be either static (in ms) or dynamic (in note length percentage up to 255).
pub enum SliceTypes {
    /// Static duration measured in milliseconds.
    Static(Duration),
    /// Dynamic duration measured in object length percentage up to 255.<br>
    /// Can also be used up to 100, allowing overload (110%).
    Dynamic(u8),
}

/// Repartition of the effect on the length of a nested note.
/// Dictates which side(s) of the note will be affected by an effect.<br>
/// Whole shouldn't be the same as a Start(Dynamic(255)) or something else as some effects have a different action on the start or end of the note and may act different if applied to a whole note. This way abstractions may be used to compare these two cases instead of using a normal attribute to theoretically speed up the process.
pub enum Repartition {
    /// Whole note.
    Whole,
    /// Only the start of the note.
    Start(SliceTypes),
    /// Only the end of the note.
    End(SliceTypes),
    /// Both sides of the note.
    BothSides { start: SliceTypes, end: SliceTypes },
}

/// Generic ADSR member.
#[derive(new)]
pub struct AdsrComponent {
    /// Time it takes to get to the final value (`until`).
    time: Duration,
    /// Final value, percentage (255 max).
    until: u8,
}

impl Default for AdsrComponent {
    fn default() -> Self {
        AdsrComponent::new(Duration::new(0, 0), 255)
    }
}

/// ADSR effect filter to control the volume of the note during a trigger.
#[derive(new)]
pub struct ADSR {
    attack: AdsrComponent,
    decay: AdsrComponent,
    sustain: AdsrComponent,
    release: Duration,
}

/// Dynamics are used instead of a volume for simplicity.
/// FortePiano isn't included but can be achieved with ADSR.
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
#[derive(new)]
pub enum Tuning {
    /// Static tuning without dependencies.
    Static { tuning: u16 },
    /// Relative tuning depending on the previous instrument tuning.
    Relative { relation: Operations },
}

/// Instruments collection.
pub enum Instruments {
    /// Basic sine wave.
    Sine(Tuning),
    /// Basic square wave.
    Square(Tuning),
    /// Basic triangle wave.
    Triangle(Tuning),
    /// Waveform product between two boxed instruments.
    Product(Box<(Instruments, Instruments)>),
    /// Waveform sum between two boxed instruments (combines two waveforms).
    Sum(Box<(Instruments, Instruments)>),
}

/// Vibrato configuration.
#[derive(new)]
pub struct VibratoConfig {
    /// Amplitude of the vibrato, in tenths of a frequency whole interval (10 means it will touch the whole note above and below).
    amplitude: u8,
    /// Frequency of the vibrato in hertz.
    frequency: u8,
}

/// Volume fader configuration.
#[derive(new)]
pub struct VolumeFadeConfig {
    /// Initial volume.
    from: Dynamics,
    /// Final volume.
    to: Dynamics,
    /// States if volume changes will be applied inside a note duration or not.
    inside: bool,
}

/// Acceleration configuration
pub struct AccelConfig {
    /// Initial tempo.
    from: u8,
    /// Final tempo.
    to: u8,
}

/// Wrappers that can either contain an Item or another Wrappers with more information based on the variant.
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
    Legato(Repartition, Vec<Wrappers>),
    Vibrato(Repartition, VibratoConfig, Vec<Wrappers>),
    Volume(Dynamics, Vec<Wrappers>),
    VolumeFader(VolumeFadeConfig, Vec<Wrappers>),
    ADSR(ADSR, Vec<Wrappers>),
    Singleton(Items), //TODO: finish this enum with elements found in Musescore
}
