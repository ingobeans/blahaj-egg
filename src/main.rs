slint::slint! {
    export component App inherits Window {
        background: rgba(0, 0, 0, 0);
        no-frame: true;
        title: "Blåhaj";
        Text { text: "Hello World!"; }

        Image { source: @image-url("blahaj.png"); }
    }
}

fn main() {
    App::new().unwrap().run().unwrap()
}
