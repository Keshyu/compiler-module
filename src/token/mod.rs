pub trait Token {
    type Type;

    fn r#type(&self) -> Self::Type;
}
