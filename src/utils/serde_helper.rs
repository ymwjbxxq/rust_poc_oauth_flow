pub trait SerdeExt {
    fn value_to_string(self) -> String;
}

impl SerdeExt for serde_json::Value {
    #[inline]
    fn value_to_string(self) -> String {
      serde_json::from_value(self).unwrap()
    }
}

// impl<T> SerdeExt for Option<T> {
//     #[inline]
//     fn value_to_string(self) -> String {
//         self.is_some().to_string()
//     }
// }

// impl<T, E> SerdeExt for Result<T, E> {
//     #[inline]
//     fn value_to_string(self) -> std::string::String {
//         self.is_ok().to_string()
//     }
// }