slint::slint! {
    export component App inherits Window {
        background: rgba(0, 0, 0, 0.0);
        no-frame: true;
        title: "Blåhaj";
        width: 415px;
        height: 559px;
        Text { text: "Hello World!"; x:0;y:0;}

        Image { source: @image-url("backdrop.png");}
        Image { source: @image-url("blahaj.png"); width: 300px; x:100px;}
        Image { source: @image-url("egg.png");}
    }
}

fn main() {
    println!("{}, v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    App::new().unwrap().run().unwrap()
}
