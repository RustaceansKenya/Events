pub type MemdbResult<T> = Result<T, MemdbError>;

pub enum MemdbError {
    KeyAlreadyExists,
    ValueAlreadyExists,
    ValueNotFound,
}
