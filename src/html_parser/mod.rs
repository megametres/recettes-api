fn load_page(page_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::{Error, ErrorKind};

    let result = reqwest::blocking::get(page_url)?;

    if !result.status().is_success() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            format!("Incorrect result status : {}", result.status()),
        )));
    }

    Ok(result.text()?)
}

fn extract_jsonld(page_body: &str) -> &str {
    use regex::Regex;

    let jsonld_regex =
        Regex::new(r#"<script type=['"]application/ld\+json['"]>([^<]*)</script>"#).unwrap();

    match jsonld_regex.captures(&page_body[..]) {
        Some(result) => result.get(1).map_or("", |m| m.as_str()),
        None => "",
    }
}

pub fn parse_recipe(page_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let page_body = load_page(page_url)?;
    let page_jsonld = extract_jsonld(page_body.as_str());
    Ok(())
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
    fn test_load_page_404() {
        let page_body = load_page("https://httpstat.us/404");
        assert!(page_body.is_err());
    }

    #[test]
    fn test_load_page_500() {
        let page_body = load_page("https://httpstat.us/500");
        assert!(page_body.is_err());
    }

    #[test]
    fn test_extract_jsonld() {
        let jsonld = "{\"name\": \"Carr\u{00e9} aux dattes (le meilleur)\"}";
        let html_body = format!("<!DOCTYPE html><head><script type='application/ld+json'>{}</script></head><body>Body</body></html>", jsonld);

        assert_eq!(extract_jsonld(html_body.as_str()), jsonld);
    }

    #[test]
    fn test_extract_without_jsonld() {
        let html_body = "<!DOCTYPE html><head></head><body>Body</body></html>";

        assert_eq!(extract_jsonld(html_body), "");
    }
}
