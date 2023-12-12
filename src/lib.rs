pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


/// Stores the prefix and local unique identifier
/// for a compact URI (CURIE)
pub struct Reference {
    prefix: String,
    identifier: String,
}


pub struct Record {
    curie_prefix: String,
    uri_prefix: String,
    curie_prefix_synonyms: Vec<String>,
    uri_prefix_synonyms: Vec<String>,
}
