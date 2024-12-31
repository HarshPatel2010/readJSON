use serde::{ Serialize,Deserialize};
#[derive(Seralize,Deserialize)]
struct Paragraph{
    name:String
}
struct Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>,
}
fn main() {

    println!("Hello, world!");
}
