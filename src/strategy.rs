use std::fs::File;
use std::io::Read;
use std::net::TcpStream;

fn main() {
    let file = File::open("image.png").unwrap();
    let mut image_loader = ImageLoader::new(file);
    let image_from_file = image_loader.load();

    let stream = TcpStream::connect("addr").unwrap();
    image_loader.set_source(stream);
    let image_from_network = image_loader.load();
}

struct ImageLoader {
    src: Box<dyn Read>,
}

impl ImageLoader {
    fn new(src: impl Read + 'static) -> Self {
        let src = Box::new(src);
        Self { src }
    }

    fn load(&mut self) -> Image {
        let mut buf = vec![];
        self.src.read_to_end(&mut buf).unwrap();
        Image(buf)
    }

    fn set_source(&mut self, src: impl Read + 'static) {
        self.src = Box::new(src);
    }
}

struct Image(Vec<u8>);
