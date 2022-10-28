use crate::guesslang::{guess, Code};

pub async fn get_code_type(snippet: String) -> Option<String> {
    let code: Code = guess(snippet).await.unwrap();
    let mut code_type = String::from("");
    if code.probability >= 0.50 {
        code_type = code.language;
        return Some(code_type);
    }
    None
}
