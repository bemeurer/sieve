include!(concat!(env!("OUT_DIR"), "/cache_size.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
