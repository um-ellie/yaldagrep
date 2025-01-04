/// Configuration for the Yalda Grep Tool.
///
/// Represents the query to search for, the file path to search in, and whether the search
/// should be case-insensitive.
pub struct GrabConfig {
    /// The query string to search for.
    pub query: String,
    /// The path to the file to search in.
    pub file_path: String,
    /// Whether the search should be case-insensitive.
    pub ignore_case: bool,
}

impl GrabConfig {
    /// Parses command-line arguments to generate a GrabConfig instance.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator of strings representing command-line arguments.
    ///
    /// # Returns
    ///
    /// A Result containing the GrabConfig instance if successful, or an error message if parsing fails.

    pub fn grabargs (
        mut args: impl Iterator<Item = String>,
    ) -> Result<GrabConfig, &'static str> {
        // Implementation...

        args.next();


        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't grab for Query - Not enough arguments."),
        };


        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't grab for File path - Not enough arguments."),
        };


        let mut ignore_case = true;
        if query.chars().any(|char| char.is_uppercase()) {
            ignore_case = false;
        }


        Ok(GrabConfig{query,file_path,ignore_case})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grabargs_with_all_arguments (){
        let mut args = vec![
            "name".to_string(),
            "test".to_string(),
            "new.txt".to_string(),
            "1".to_string()]
            .into_iter();
        let config = GrabConfig::grabargs(&mut args).unwrap();
        assert_eq!(config.query,"test");
        assert_eq!(config.file_path,"new.txt");
        assert_eq!(config.ignore_case,true);
    }

    #[test]
    fn grabargs_missing_query () {
        let mut args = vec![
            "name".to_string()
            ].into_iter();

        let result = GrabConfig::grabargs(&mut args);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(),"Didn't grab for Query - Not enough arguments.");
    }

    #[test]
    fn grabargs_missing_file_path(){
        let mut args = vec![
            "name".to_string(),
            "test".to_string()
            ].into_iter();
        let result = GrabConfig::grabargs(&mut args);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(),"Didn't grab for File path - Not enough arguments.");
    }

    #[test]
    fn grabargs_sensitive_case(){
        let mut args = vec![
            "name".to_string(),
            "test".to_string(),
            "new.txt".to_string(),
        ].into_iter();

        let config = GrabConfig::grabargs(&mut args).unwrap();
        assert_eq!(config.ignore_case,true);
    }

    #[test]
    fn grabargs_insensitive_case(){
        let mut args = vec![
            "name".to_string(),
            "TeSt".to_string(),
            "new.txt".to_string(),
        ].into_iter();

        let config = GrabConfig::grabargs(&mut args).unwrap();
        assert_eq!(config.ignore_case,false);
    }
}