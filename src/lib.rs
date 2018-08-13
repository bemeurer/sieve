extern crate num;

use num::{Integer, Unsigned, FromPrimitive, CheckedAdd};
use num::integer::{self, Roots};

include!(concat!(env!("OUT_DIR"), "/cache_size.rs"));

pub fn segsieve<T>(limit: T) -> Vec<T>
where T: FromPrimitive + Integer + Unsigned + CheckedAdd + Roots + Clone
{
    let sqrt = integer::sqrt(limit.clone());
    let cache_size = T::from_usize(CACHE_SIZE);
    let segment_size = if cache_size.is_some() {
        sqrt.max(cache_size.unwrap())
    } else {
        sqrt
    };

    let primes: Vec<T> = Vec::new();

    for low in num::range_step(T::from_usize(0).unwrap(), limit, segment_size){
        
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(!segsieve(100_usize).is_empty());
        assert_eq!(2 + 2, 4);
    }
}
