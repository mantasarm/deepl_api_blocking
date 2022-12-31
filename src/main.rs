pub mod deepl_api;

use deepl_api::{DeeplTranslator, Lang};

fn main() {
    let translator = DeeplTranslator::new("YOUR-KEY-HERE");
    
    let translation = translator.translate("Hello beautiful world", None, Lang::DE);
    
    println!("{}", translation);
}