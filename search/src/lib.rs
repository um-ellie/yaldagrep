/// Searches for a query string in the contents case-insensitively.
///
/// # Arguments
///
/// * `query` - The query string to search for.
/// * `contents` - The string contents to search within.
///
/// # Returns
///
/// A vector containing references to lines containing the query string.
pub fn search_insensitive_case<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    // Implementation...

    contents
    .lines()
    .filter(|line| line.contains(&query))
    .collect()
}


/// Searches for a query string in the contents case-sensitively.
///
/// # Arguments
///
/// * `query` - The query string to search for.
/// * `contents` - The string contents to search within.
///
/// # Returns
///
/// A vector containing references to lines containing the query string.
pub fn search_sensitive_case<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    // Implementation...
    
    contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}

#[cfg(test)]
mod search {
    use super::*;

    #[test]
    fn result_search_insensitive_case(){
        let query = "search";
        let contents = "This is a\n\
        Text filefor test\n\
        search functionallity.\n\
        Copyright by Mitsuha";

        assert_eq!(vec!["search functionallity."],search_insensitive_case(query, contents));
    }

    #[test]
    fn result_search_sensitive_case(){
        let query = "FunCtionalliTy";
        let contents = "This is a\n\
        Text filefor test\n\
        search functionallity.\n\
        Copyright by Mitsuha";

        assert_eq!(vec!["search functionallity."],search_sensitive_case(query, contents));
    }
}