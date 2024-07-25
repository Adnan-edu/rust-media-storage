#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}
impl Media {
    fn description(&self) -> String {
        //Pattern matching to handle enum to find the type
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
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
    println!("{:#?}", good_book.description());
    println!("{:#?}", good_movie.description());
    println!("{:#?}", audiobook.description());
}
