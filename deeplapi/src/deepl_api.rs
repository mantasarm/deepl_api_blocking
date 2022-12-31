use std::collections::HashMap;

pub struct DeeplTranslator {
	client: reqwest::blocking::Client,
	key: String
}

impl DeeplTranslator {
	pub fn new(key: &str) -> Self {
		let mut auth = "DeepL-Auth-Key ".to_string();
		auth.push_str(key);
		
		Self {
			client: reqwest::blocking::Client::new(),
			key: auth
		}
	}
	
	pub fn translate(&self, text: &str, source_lang: Option<Lang>, target_lang: Lang) -> String {
        let url = "https://api-free.deepl.com/v2/";
        let endpoint = reqwest::Url::parse(url).unwrap();
        let translate_url = endpoint.join("translate").unwrap();
		
		let mut params = HashMap::new();
		params.insert("text", text);
		
		if let Some(src_lang) = source_lang {
			params.insert("source_lang", src_lang.as_str());
		}
		
		params.insert("target_lang", target_lang.as_str());
		
		let post = self.client.post(translate_url.clone()).header("Authorization", &self.key);
        let response: serde_json::Value = post.form(&params).send().unwrap().json().unwrap();
		
		response["translations"][0]["text"].as_str().unwrap().to_string()
	}
}

pub enum Lang {
    BG,
    CS,
    DA,
    DE,
    EL,
    EN,
    EN_GB,
    EN_US,
    ES,
    ET,
    FI,
    FR,
    HU,
    ID,
    IT,
    JA,
    LT,
    LV,
    NL,
    PL,
    PT,
    PT_BR,
    PT_PT,
    RO,
    RU,
    SK,
    SL,
    SV,
    TR,
    UK,
    ZH,
}

impl Lang {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BG => "BG",
            Self::CS => "CS",
            Self::DA => "DA",
            Self::DE => "DE",
            Self::EL => "EL",
            Self::EN => "EN",
            Self::EN_US => "EN-US",
            Self::EN_GB => "EN-GB",
            Self::ES => "ES",
            Self::ET => "ET",
            Self::FI => "FI",
            Self::FR => "FR",
            Self::HU => "HU",
            Self::ID => "ID",
            Self::IT => "IT",
            Self::JA => "JA",
            Self::LT => "LT",
            Self::LV => "LV",
            Self::NL => "NL",
            Self::PL => "PL",
            Self::PT => "PT",
            Self::PT_BR => "PT-BR",
            Self::PT_PT => "PT-PT",
            Self::RO => "RO",
            Self::RU => "RU",
            Self::SK => "SK",
            Self::SL => "SL",
            Self::SV => "SV",
            Self::TR => "TR",
            Self::UK => "UK",
            Self::ZH => "ZH",
        }
    }
}