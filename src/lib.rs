
trait MyTraitForString {
    fn to_vec_char(&self) -> Vec<char>;
    fn to_vec_u8(&self) -> Vec<u8>;
}

impl MyTraitForString for String {
    fn to_vec_char(&self) -> Vec<char> {
        self.chars().collect::<Vec<_>>()
    }

    fn to_vec_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
