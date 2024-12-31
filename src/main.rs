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

//For writing JSON
#[derive(Serialize, Deserialize, Debug)]
struct Para{
    name:String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Art{
art:String,
auth:String,
Para:Vec<Para>,
}
fn main() {
    let json = r#"
    {
        "article": "An Example Article",
        "author": "John Doe",
        "paragraph": [
            {"name":"starting sentencex"},
            {"name":"body paragraph"},
            {"name":"end of the paragraph"}
        ]
    }"#;

    let parsed:Article = read_json_type(json);
    println!("the first sentece of para is : {}",parsed.article);

    // write JSON
    let art = Art{
        art:String::from("how to work with json"),
        auth:String::from("i am auth from the string okay!"),
        Para:vec![Para{name: String::from("starting sentencex")},Para{name: String::from("body paragraph")},Para{name: String::from("end of the paragraph")},Para{name: String::from("paragraph")},Para{name: String::from("the end of the paragraph")}]
    };

    let wjson = serde_json::to_string(&art).unwrap();
    println!("{}", wjson);

}
fn read_json_type(json:&str) -> Article {
    let parsed:Article = serde_json::from_str(json).unwrap();
    return parsed
}
