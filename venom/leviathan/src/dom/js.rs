use boa_engine::{Context, Source};
use crate::error::LeviathanError;

pub struct LeviathanJsContext {
    pub context: Context,
}

impl LeviathanJsContext {
    pub fn new() -> Self {
        let context = Context::default();
        
        // Stub native Web APIs (window, navigator) to passively bypass basic anti-bot scripts
        let script = r#"
            const window = {
                navigator: {
                    userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                    webdriver: false,
                    vendor: "Google Inc.",
                    platform: "MacIntel"
                },
                document: {
                    cookie: ""
                }
            };
            const navigator = window.navigator;
            const document = window.document;
        "#;
        
        let mut self_instance = Self { context };
        let _ = self_instance.evaluate(script); // Bootstrap global APIs silently
        
        self_instance
    }

    /// Safely evaluates ECMAScript native logic inside the headless boundary
    pub fn evaluate(&mut self, script: &str) -> Result<String, LeviathanError> {
        let source = Source::from_bytes(script.as_bytes());
        match self.context.eval(source) {
            Ok(value) => Ok(value.display().to_string()),
            Err(err) => Err(LeviathanError::ConnectionFailed(format!("JS Execution Error: {}", err))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_evaluation() {
        let mut js = LeviathanJsContext::new();
        // Basic math eval verification
        let res = js.evaluate("let x = 5; x * 2;").unwrap();
        assert_eq!(res, "10");

        // Anti-bot stealth validation test
        let nav = js.evaluate("navigator.webdriver").unwrap();
        assert_eq!(nav, "false");
    }
}
