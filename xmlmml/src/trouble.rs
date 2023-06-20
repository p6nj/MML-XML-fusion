pub enum Result<T>{
    Ok(T),
    Err(Error)
}

pub enum Error{
    Dummy
}