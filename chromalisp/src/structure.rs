pub enum Item{
    Note(u8),
    Silence
}
pub enum Wrapper{
    Song(String, Vec<Wrapper>),
    Album(String, Vec<Wrapper>),
    Artist(String, Vec<Wrapper>),
    Year(u16, Vec<Wrapper>),
    NoteDef(Vec<String>, Vec<Wrapper>),
    Channel(Vec<Wrapper>),
    Singleton(Item)
    //TODO: finish this enum
}