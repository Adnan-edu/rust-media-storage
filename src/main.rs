#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
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
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}
impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }
    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            //We have something to return
            Some(&self.items[index])
        } else {
            //Index out of bound
            None
        }
    }
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
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{:#?}", good_book.description());
    // println!("{:#?}", good_movie.description());
    // println!("{:#?}", audiobook.description());
    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(good_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.get_by_index(1) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }
    }

    // if let Some(value) = catalog.get_by_index(0) {
    //     println!("Item in pattern match: {:#?}", value);
    // } else {
    //     println!("No value!!!");
    // }
}
