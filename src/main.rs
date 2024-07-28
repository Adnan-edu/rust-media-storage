mod content;
use content::catalog::Catalog;
use content::media::Media;

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

    let item = catalog.get_by_index(10);

    let placeholder = Media::Placeholder;
    //unwrap_or => Some: returns value, None: returns the provided default value/placeholder
    println!("{:#?}", item.unwrap_or(&placeholder));

    //Unwrap => Some: returns value, None: panics - Use: Debugging
    //println!("{:#?}", item.unwrap());

    //Expect => Some: returns value, None: panics with error message
    //Use Expect => When we want to crash if there is no value - Use case checking of env variable presence in program
    //println!("{:#?}", item.expect("Expected there to be an item here!"));
}
