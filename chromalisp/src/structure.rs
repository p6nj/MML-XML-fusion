/// Single item contained in wrappers.<br>
/// The Note item has frequency information deduced from the note list but both have no length information
/// (taken from either a higher-level length wrapper or the default value).
pub enum Item{
    /// A single note with the index of the note found in the note list.
    Note(u8),
    /// A single rest.
    Rest
}

/// Duration for a slice of an object, can be either static (in ms) or dynamic (in note length percentage up to 255).
pub enum Duration{
    /// Static duration measured in milliseconds.
    Static(u16),
    /// Dynamic duration measured in object length percentage up to 255.<br>
    /// Can also be used up to 100, allowing overload (110%).
    Dynamic(u8)
}

/// Wrapper that can either contain an Item or another wrapper with more information based on the variant.
pub enum Wrapper{
    /// The title of the underlying song.
    Song(String, Vec<Wrapper>),
    /// The title of the underlying album.
    Album(String, Vec<Wrapper>),
    /// The name of the artist responsible for the underlying notes.
    Artist(String, Vec<Wrapper>),
    /// The year of the underlying song or album.
    Year(u16, Vec<Wrapper>),
    /// The notes names used for the interface.
    NoteDef(Vec<String>, Vec<Wrapper>),
    /// The channel name for the underlying notes.
    Channel(Vec<Wrapper>),
    Length(u8, Vec<Wrapper>),
    Octave(u8, Vec<Wrapper>),
    Loop(u8, Vec<Wrapper>),
    Legato{start:Duration, end:Duration},
    Vibrato{start:Duration, end:Duration},
    Volume(u8),
    VolumeFader{from:u8,to:u8},
    Singleton(Item)
    //TODO: finish this enum
}