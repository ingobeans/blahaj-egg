slint::slint! {
    component Button inherits TouchArea {
        width:39px;
        height:41px;
        in property <image> glyph;

        pointer-event() => {
            if (self.pressed) {
                img.source = @image-url("button_pressed.png");
                img.x = 2px;
                glyph.x = 2px;
                img.y = 2px;
                glyph.y = 2px;
            } else {
                img.source = @image-url("button.png");
                img.x = 0px;
                glyph.x = 0px;
                img.y = 0px;
                glyph.y = 0px;
            }
        }

        img:=Image {
            source: @image-url("button.png");
            width:39px;
            height:41px;
        }
        glyph:=Image {
            source: root.glyph;
            width:39px;
            height:41px;
        }
    }
    export component App inherits Window {
        background: rgba(0, 0, 0, 0);
        no-frame: true;
        always-on-top: true;
        title: "Blåhaj";
        width: 410px;
        height: 563px;

        Image { source: @image-url("backdrop.png");}
        Image { source: @image-url("blahaj.png"); width: 300px; x:100px; y:170px;}
        Image { source: @image-url("egg.png");}
        callback mouse_move(length, length);

        TouchArea {
            moved => {
                if (self.pressed ) {
                    root.mouse_move((self.mouse_x) - self.pressed_x, (self.mouse_y) - self.pressed_y);
                }
            }
        }
        TouchArea {
            width:22px;
            height:25px;
            x:375px;
            y:5px;
            clicked => {
                root.close()
            }
        }
        // buttons:
        b1:=Button {
            x:100px;
            y:472px;
            glyph: @image-url("glyphs/timer.png");
        }
        b2:=Button {
            x:182px;
            y:491px;
            glyph: @image-url("glyphs/pat.png");
        }
        b3:=Button {
            x:260px;
            y:472px;
            glyph: @image-url("glyphs/menu.png");
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
