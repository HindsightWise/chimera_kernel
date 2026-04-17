use scraper::{Html, Selector};
use crate::error::LeviathanError;

pub struct LeviathanDocument {
    pub tree: Html,
}

impl LeviathanDocument {
    /// Parses an incoming raw HTML byte stream or string into a queryable DOM
    pub fn parse(html: &str) -> Self {
        let tree = Html::parse_document(html);
        Self { tree }
    }

    /// Convenience wrapper to perform rapid multi-element CSS extraction (HTML inclusive)
    pub fn css_select(&self, query: &str) -> Result<Vec<String>, LeviathanError> {
        let selector = Selector::parse(query).map_err(|e| {
            LeviathanError::ConnectionFailed(format!("Invalid CSS Selector '{}': {:?}", query, e))
        })?;

        let mut results = Vec::new();
        for element in self.tree.select(&selector) {
            results.push(element.inner_html());
        }

        Ok(results)
    }

    /// Extract the text directly (stripping internal HTML nodes)
    pub fn css_select_text(&self, query: &str) -> Result<Vec<String>, LeviathanError> {
        let selector = Selector::parse(query).map_err(|e| {
            LeviathanError::ConnectionFailed(format!("Invalid CSS Selector '{}': {:?}", query, e))
        })?;

        let mut results = Vec::new();
        for element in self.tree.select(&selector) {
            let text = element.text().collect::<Vec<_>>().join(" ");
            results.push(text.trim().to_string());
        }

        Ok(results)
    }

    /// Extracts raw JS string blocks from all <script> tags for consumption by the JS evaluator
    pub fn extract_scripts_raw(&self) -> Result<Vec<String>, LeviathanError> {
        self.css_select("script")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraper_dom_creation() {
        let raw_html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Leviathan Test</title></head>
            <body>
                <div class="target">Entity Alpha</div>
                <div class="target">Entity Beta</div>
                <span id="secret">Omega</span>
            </body>
            </html>
        "#;

        let doc = LeviathanDocument::parse(raw_html);

        // Test Title Extraction
        let titles = doc.css_select_text("title").unwrap();
        assert_eq!(titles.len(), 1);
        assert_eq!(titles[0], "Leviathan Test");

        // Test CSS Target class extraction
        let targets = doc.css_select_text(".target").unwrap();
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0], "Entity Alpha");
        assert_eq!(targets[1], "Entity Beta");

        // Test id extraction
        let secret = doc.css_select_text("#secret").unwrap();
        assert_eq!(secret[0], "Omega");
    }
}
