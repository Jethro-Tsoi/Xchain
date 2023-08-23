use bincode;
use serde::{
    Deserialize,
    Serialize
};
use crypto::{
    sha3::Sha3,
    digest::Digest,
};

pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize, 
{
    bincode::serialize(value).unwrap()
}

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>, 
{
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) -> String{
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::{
        Point,
        serialize,
        deserialize,
    };

    #[test]
    fn coder_works() {
        let p = Point{
            x: 1,
            y: 1,
        };
        let se = serialize(&p);
        let de: Point = deserialize(&se);
        assert_eq!(p, de);
    }
}