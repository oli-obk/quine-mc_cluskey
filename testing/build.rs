extern crate skeptic;

fn main() {
    println!("cargo:rerun-if-changed=../README.md");
    skeptic::generate_doc_tests(&["../README.md"]);
}
