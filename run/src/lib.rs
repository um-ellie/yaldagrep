use std::{error::Error, fs};

use grab::GrabConfig;
use search::{search_insensitive_case, search_sensitive_case};

/// Executes the search based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The configuration for the search.
///
/// # Returns
///
/// A Result indicating success or failure.
pub fn run(config: GrabConfig) -> Result<(), Box<dyn Error>> {


    let contents = fs::read_to_string(config.file_path)?;



    let result = if config.ignore_case {
        search_insensitive_case(&config.query, &contents)
    } else {
        search_sensitive_case(&config.query, &contents)
    };

    for line in result{
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod test_run {
    use std::{fs::File, io::Write};
    use super::*;

    #[test]
    fn result_run(){
        let file_name = "test.txt";
        let content = "This is text file for test run functionallity.";
        let mut file = File::create(file_name).expect("faild");
        file.write_all(content.as_bytes()).expect("Faild");

        let config = GrabConfig {
            query: String::from("test"),
            file_path: String::from("test.txt"),
            ignore_case: true,
        };

        let result = run(config);

        assert!(result.is_ok());
    }
}