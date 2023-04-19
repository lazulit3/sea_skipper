/// Trait providing the `Location` of an API resource.
pub trait Location {
    fn location(&self) -> String;
}
