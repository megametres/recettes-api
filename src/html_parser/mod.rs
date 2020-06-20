use crate::database::models::model_recipe::*;
use crate::database::recipe_helper::*;
use crate::error::*;

fn load_page(page_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = reqwest::blocking::get(page_url)?;

    if !result.status().is_success() {
        return Err(Box::new(RecipeError {
            message: format!("Incorrect result status : {}", result.status()),
        }));
    }

    Ok(result.text()?)
}

fn extract_jsonld(page_body: &str) -> Result<&str, Box<dyn std::error::Error>> {
    use regex::Regex;

    let jsonld_regex =
        Regex::new(r#"<script type=['"]application/ld\+json['"]>([^<]*)</script>"#).unwrap();

    match jsonld_regex.captures(&page_body[..]) {
        Some(result) => Ok(result.get(1).map_or("", |m| m.as_str())),
        _ => Err(Box::new(RecipeError {
            message: "No json-ld found in the provided url".to_string(),
        })),
    }
}

pub fn recipe_parser(page_url: &str) -> Result<RecipeFull, Box<dyn std::error::Error>> {
    let page_body = load_page(page_url)?;
    let page_jsonld = extract_jsonld(page_body.as_str())?;

    Ok(parse_jsonld(page_jsonld, page_url))
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

        assert_eq!(extract_jsonld(html_body.as_str()).unwrap(), jsonld);
    }

    #[test]
    fn test_extract_without_jsonld() {
        let html_body = "<!DOCTYPE html><head></head><body>Body</body></html>";

        assert!(extract_jsonld(html_body).is_err());
    }

    // #[test]
    // fn test_parse_url() {
    //     let haha = parse_recipe("https://www.ricardocuisine.com/recettes/8584-jarrets-d-agneau-aux-epices-et-au-jus-de-carottes").unwrap();
    //     println!("hahaha");
    // }
}
