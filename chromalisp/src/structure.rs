pub enum Item{
    Note(u8),
    Silence
}

pub enum Slide{
    Static(u16),
    Dynamic(u8)
}

pub enum Wrapper{
    Song(String, Vec<Wrapper>),
    Album(String, Vec<Wrapper>),
    Artist(String, Vec<Wrapper>),
    Year(u16, Vec<Wrapper>),
    NoteDef(Vec<String>, Vec<Wrapper>),
    Channel(Vec<Wrapper>),
    Length(u8, Vec<Wrapper>),
    Octave(u8, Vec<Wrapper>),
    Loop(u8, Vec<Wrapper>),
    Legato{start:Slide, end:Slide},
    Singleton(Item)
    //TODO: finish this enum
}