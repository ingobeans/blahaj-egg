slint::slint! {
    export component App inherits Window {
        background: rgba(0, 0, 0, 0.5);
        no-frame: true;
        title: "Blåhaj";
        width: 410px;
        height: 563px;
        Text { text: "Hello World!"; x:0;y:0;}

        Image { source: @image-url("backdrop.png");}
        Image { source: @image-url("blahaj.png"); width: 300px; x:100px;}
        img:= Image { source: @image-url("egg.png");}
        callback mouse_move(length, length);

        TouchArea {
            moved => {
                if (self.pressed ) {
                    root.mouse_move((self.mouse_x) - self.pressed_x, (self.mouse_y) - self.pressed_y);
                }
            }
        }
    }
}

fn main() {
    println!("{}, v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let pin_win = App::new().unwrap();

    pin_win
        .window()
        .set_position(slint::LogicalPosition::new(0., 0.));

    let pin_win_clone = pin_win.as_weak();
    pin_win.on_mouse_move(move |delta_x, delta_y| {
        let pin_win_clone = pin_win_clone.unwrap();
        let logical_pos = pin_win_clone
            .window()
            .position()
            .to_logical(pin_win_clone.window().scale_factor());
        pin_win_clone
            .window()
            .set_position(slint::LogicalPosition::new(
                logical_pos.x + delta_x,
                logical_pos.y + delta_y,
            ));
    });

    pin_win.run().unwrap();
}
