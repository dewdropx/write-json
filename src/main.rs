use serde::{Deserialize, Serialize}; // serialization and deserialization

// create the required structs
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {

    // generate the article
    let article: Article = Article {
        article: String::from("how to work with json in rust"),
        author: String::from("arjun"),
        paragraph: vec! [
            Paragraph {
                name: String::from("starting sentence")
            },
            Paragraph {
                name: String::from("body of the article")
            },
            Paragraph {
                name: String::from("end of the article")
            }
        ]
    };

    // use serde to serialize it into json format and log into the terminal
    let json = serde_json::to_string(&article).unwrap();
    println!("the json is: {}", json);
}