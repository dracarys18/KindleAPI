use crate::constant;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Kindle {
    sno: i8,
    name: String,
    version: String,
    dw_link: String,
    release_notes: String,
}

impl Kindle {
    pub fn scrape_ota() -> Value {
        let mut kindvec = Vec::<Kindle>::new();
        let resp = reqwest::blocking::get(constant::URL).unwrap();
        let doc = Document::from(resp.text().unwrap().as_str());
        for (i, node) in doc.find(Class("cs-help-landing-section")).enumerate() {
            let kindle_name = &node
                .find(Class("sectiontitle").and(Name("h4")))
                .map(|s| s.text())
                .collect::<Vec<_>>()[0];
            let ver = node
                .find(Class("a-list-item"))
                .map(|s| s.text())
                .collect::<Vec<_>>()[0]
                .clone()
                .splitn(2, ':')
                .last()
                .unwrap()
                .trim()
                .to_string();
            let links = node
                .find(Attr("href", ()))
                .map(|s| s.attr("href").unwrap_or(""))
                .collect::<Vec<_>>();
            let dw = links[0].to_string();
            let rnotes = links.into_iter().nth(1).unwrap_or("").to_string();
            kindvec.push(Kindle {
                sno: i as i8,
                name: kindle_name.to_string(),
                version: ver,
                dw_link: dw,
                release_notes: rnotes,
            });
        }
        serde_json::json!(kindvec)
    }
}