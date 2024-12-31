use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
fn main() {
    let json = r#"
   {
   "article": {},
   "author": {},
   "paragraph": [
   {"name":"starting sentence"},
   {"name":"body paragraph"},
   {"name":"end of the paragraph"},
   ]}"#;

    let parsed:Article = read_json_type(json);
    println!("the first sentece of para is : {}",parsed.paragraph[0].name);

}
fn read_json_type(json:&str)->Article {
    let parsed:Article = serde_json::from_str(json).unwrap();
    return parsed
}
