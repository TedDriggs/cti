use stix::{Identity, Relationship};

#[derive(stix::Collection)]
enum Declaration {
    Identity(Identity),
    Relationship(Relationship),
}

fn main() {
    let builder = CollectionBuilder::default();
    println!("{}", builder.len());
}
