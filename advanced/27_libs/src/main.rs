use kube::{CustomResourceExt, CustomResource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Book 作为一个新的 Custom resource
#[derive(Debug, CustomResource, Clone, Deserialize, JsonSchema, Serialize)]
#[kube(group = "k8s.tyr.app", version = "v1", kind = "Book", namespaced)]
pub struct BookSpec {
    pub title: String,
    pub authors: Option<Vec<String>>,
}

fn main() {
    let book = Book::new(
        "rust-programming",
        BookSpec {
            title: "rust-programming".into(),
            authors: Some(vec!["test".into()]),
        },
    );
    println!("{}", serde_yaml::to_string(&Book:crd()).unwrap());
    println!("{}", serde_yaml::to_string(&book).unwrap());
}
