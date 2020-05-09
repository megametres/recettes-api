use std::io::{Error, ErrorKind};

pub fn load_page(page_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = reqwest::blocking::get(page_url)?;

    if !result.status().is_success() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            format!("Incorrect result status : {}", result.status()),
        )));
    }

    Ok(result.text()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_page_200() {
        let page_body = load_page("https://httpstat.us/200");
        assert!(page_body.is_ok());
    }
    #[test]
    fn test_load_page_500() {
        let page_body = load_page("https://httpstat.us/500");
        assert!(page_body.is_err());
    }
}
