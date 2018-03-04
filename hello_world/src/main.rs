#[derive(Debug)]
struct Book<'title_valid> {
    title: &'title_valid str,
}

fn main() {
    let my_tile: &'static str = "Title of my book";
    let book = Book { title: &my_tile };
    println!("{:?}", book);
}
