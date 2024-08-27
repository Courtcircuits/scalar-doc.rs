use scalar_doc::Documentation;

fn main() {
    //just templatize documentation
    let doc = Documentation::new("Api Documentation title", "Api Documentation content").build();

    println!("{}", doc.unwrap());
}
