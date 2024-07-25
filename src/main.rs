#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}
fn print_media(media: Media) {
    println!("{:#?}", media);
}
fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let good_book = Media::Book {
        title: String::from("Good Book"),
        author: String::from("Good Author"),
    };
    print_media(good_movie);
    print_media(good_book);
    print_media(audiobook);
}
