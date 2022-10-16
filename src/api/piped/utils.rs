use serde_json::Value;

use super::misc::RelatedStream;

pub(super) fn parse_related(value: Value)-> Vec<RelatedStream>{
    match value.as_array(){
        Some(array) =>{
            let mut vec = Vec::with_capacity(array.len()-1);
         for obj in array.into_iter() {
            vec.push(serde_json::from_value(obj.to_owned()).unwrap())
        }
        return vec;
        },
        None => return  Vec::with_capacity(0),
    }
}