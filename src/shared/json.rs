// interface à implémenter si on veut imposer la méthode from_json aux dtos
pub trait JsonFmt {
    fn from_json(json: &String) -> Self
    where
        Self: Sized;
}
