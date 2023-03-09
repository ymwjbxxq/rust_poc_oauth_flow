pub trait SerdeExt {
    fn value_to_string(self) -> String;
}

impl SerdeExt for serde_json::Value {
    #[inline]
    fn value_to_string(self) -> String {
        serde_json::from_value(self).unwrap()
    }
}
