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

/// Duration for a SliceTypes of an object, can be either static (in ms) or dynamic (in note length percentage up to 255).
pub enum SliceTypes {
    /// Static duration measured in milliseconds.
    Static(Duration),
    /// Dynamic duration measured in object length percentage up to 255.<br>
    /// Can also be used up to 100, allowing overload (110%).
    Dynamic(u8),
}

pub enum Repartitions {
    Whole,
    Start(SliceTypes),
    End(SliceTypes),
    BothSides { start: SliceTypes, end: SliceTypes },
}

pub struct AdsrComponent {
    time: Duration,
    until: u8,
}

pub struct ADSR {
    attack: AdsrComponent,
    decay: AdsrComponent,
    sustain: AdsrComponent,
    release: Duration,
}

/// FortePiano can be achieved with ADSR.
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
    Vibrato(Repartitions, Vec<Wrappers>),
    Volume(u8, Vec<Wrappers>),
    VolumeFader {
        from: u8,
        to: u8,
    },
    ADSR(ADSR, Vec<Wrappers>),
    Singleton(Items), //TODO: finish this enum with elements found in Musescore
}
