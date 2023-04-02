use std::time::Duration;

/// Single item contained in Wrapperss.<br>
/// The Note item has frequency information deduced from the note list but both have no length information
/// (taken from either a higher-level length Wrappers or the default value).
pub enum Items {
    /// A single note with the index of the note found in the note list.
    Note(u8),
    /// A single rest.
    Rest,
}

/// Slice of the duration of a note, used only for Repartitions.
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
/// Whole shouldn't be the same as a Start(Dynamic(255)) or something else as some effects have a different action on the start or end of the note and may act different if applied to a whole note. This way abstractions may be used to compare these two cases instead of using a normal attribute to theoratically speed up the process.
pub enum Repartitions {
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
pub struct AdsrComponent {
    /// Time it takes to get to the final value (`until`).
    time: Duration,
    /// Final value, percentage (255 max).
    until: u8,
}

/// ADSR effect filter to control the volume of the note during a trigger.
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

/// Instrument operations, allowing sound wave operations.
pub enum Operations {
    Multiply,
    Divide,
    AddOrSubstract,
}

pub enum IntrumentTypes {
    Constant { freq: u16 },
    Relative { offset: i16, relation: Operations },
}

pub enum Instruments {
    Sine(IntrumentTypes),
    Square(IntrumentTypes),
    Triangle(IntrumentTypes),
    Product(Vec<Instruments>),
    Sum(Vec<Instruments>),
}

pub struct VibratoConfig {
    amplitude: u8,
    frequency: u8
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
    /// The notes names used for the interface.
    NoteDef(Vec<String>, Vec<Wrappers>),
    /// The channel name for the underlying notes.
    Channel(Vec<Wrappers>),
    Instrument(Instruments, Vec<Wrappers>),
    Length(u8, Vec<Wrappers>),
    Octave(u8, Vec<Wrappers>),
    Loop(u8, Vec<Wrappers>),
    Legato(Repartitions, Vec<Wrappers>),
    Vibrato(Repartitions, VibratoConfig, Vec<Wrappers>),
    Volume(u8, Vec<Wrappers>),
    VolumeFader {
        from: u8,
        to: u8,
    },
    ADSR(ADSR, Vec<Wrappers>),
    Singleton(Items), //TODO: finish this enum with elements found in Musescore
}
