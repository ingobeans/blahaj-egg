use std::{collections::HashMap, sync::Mutex, time::Instant};

use rgb::{Rgba};
use slint::{Image};


slint::slint! {
    component Button inherits TouchArea {
        width:39px;
        height:41px;
        in property <bool> glyph_set: false;
        in property <image> default_glyph;
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
            source: {
                if (glyph_set) {
                    root.glyph
                } else {
                    root.default_glyph
                } 
            };
            width:39px;
            height:41px;
        }
    }
    component Clock inherits Image {
        in property <int> target_minutes: 30;
        in property <int> seconds;
        source: @image-url("clock/base.png");

        pointer:=Image {
            source: @image-url("clock/pointer.png");
            transform-rotation: seconds / (target_minutes*60.0) * 360.0deg;
            x:116px;
            y:265px;
        }

        text:=Text {
            text: {
                let remaining = 60*(target_minutes) - seconds;
                let p1 = (Math.floor(remaining / 60)) + "";
                let p2 = (Math.mod(remaining,60)) + "";
                if (p1.character-count == 1) {
                    if (p2.character-count == 1) {
                        "0" + p1 + ":" + "0" + p2
                    } else {
                        "0" +p1 + ":" + p2
                    }
                } else if (p2.character-count == 1) {
                    p1 + ":" + "0" + p2
                } else {
                    p1 + ":" + p2
                }
            };
            font-family: "Calibri";
            x:165px;
            y:225px;
            font-size: 32px;
        }
    }

    export component App inherits Window {
        background: rgba(0, 0, 0, 0);
        no-frame: true;
        always-on-top: true;
        title: "Blåhaj";
        width: 410px;
        height: 563px;

        property <int> state;
        property <bool> timer_paused;

        in-out property<duration> ticker: animation-tick();
        in-out property<duration> delta: 0ms;
        in-out property<duration> prev-ticker: 0ms;
        callback callback_ticker() -> int;
        callback start_timer();
        callback set_pause_timer(bool);
        changed ticker => {
            delta = ((ticker) - prev-ticker);
            prev-ticker = ticker;
            let value = callback_ticker();
            clock.seconds = value;
            if (value >= 60*clock.target_minutes) {
                if !timer_paused {
                    timer_paused=true;
                    start_timer();
                    set_pause_timer(true);
                    b2.glyph = @image-url("glyphs/play.png");
                    confetti.visible = true;
                }
            }
            if (confetti.visible) {
                confetti.source = load_gif_frame("confetti.gif", animation-tick() / 1ms);
            }
        };

        callback load_gif_frame(string, int) -> image;

        Image { source: @image-url("backdrop.png");}
        blahaj:= Image { source: @image-url("blahaj.png"); width: 300px; x:100px; y:170px;}
        confetti := Image { y:220px; visible: false;}
        Image { source: @image-url("egg.png");}
        clock := Clock {
            visible: false;
        }
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
            default_glyph: @image-url("glyphs/timer.png");

            clicked => {
                confetti.visible = false;
                clock.visible = !clock.visible;
                blahaj.visible = !blahaj.visible;
                if (clock.visible) {
                    state = 1;
                    start_timer();
                    b2.glyph_set = true;
                    b3.glyph_set = true;
                    b2.glyph = @image-url("glyphs/pause.png");
                    b3.glyph = @image-url("glyphs/30-5.png");
                } else {
                    state = 0;
                    if timer_paused {
                        set_pause_timer(false);
                        timer_paused = false;
                    }
                    b2.glyph_set = false;
                    b3.glyph_set = false;
                }
            }
        }
        b2:=Button {
            x:182px;
            y:491px;
            default_glyph: @image-url("glyphs/pat.png");
            clicked => {
                if (state == 1) {
                    confetti.visible = false;
                    timer_paused = !timer_paused;
                    if (timer_paused) {
                        self.glyph = @image-url("glyphs/play.png");
                    } else {
                        self.glyph = @image-url("glyphs/pause.png");
                    }
                    set_pause_timer(timer_paused);
                }
            }
        }
        b3:=Button {
            x:260px;
            y:472px;
            default_glyph: @image-url("glyphs/menu.png");
            clicked => {
                if (state == 1) {
                    confetti.visible = false;
                    if (clock.target_minutes == 30) {
                        clock.target_minutes = 5;
                        self.glyph = @image-url("glyphs/5-30.png");
                    } else {
                        clock.target_minutes = 30;
                        self.glyph = @image-url("glyphs/30-5.png");
                    }
                }
            }
        }
    }
}

static START: Mutex<Option<Instant>>  = Mutex::new(None);
static PAUSED_AT: Mutex<Option<Instant>>  = Mutex::new(None);
static PAUSED: Mutex<bool>  = Mutex::new(false);


fn main() {
    println!("{}, v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let app = App::new().unwrap();

    app
        .window()
        .set_position(slint::LogicalPosition::new(0., 0.));

    let app_clone = app.as_weak();
    app.on_mouse_move(move |delta_x, delta_y| {
        let pin_win_clone = app_clone.unwrap();
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

    let mut gifs: HashMap<&str, Vec<Image>> = HashMap::new();
    
    fn preload_gif(gifs: &mut HashMap<&str, Vec<Image>>, name:&'static str, bytes:&[u8]) {
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        
        let mut decoder = options.read_info(&bytes[..]).unwrap();
        let mut frames = Vec::new();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            let mut slint_pixel_buffer =
                slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(frame.width as _, frame.height as _);
            let buffer = frame.buffer.chunks(4).map(|f| {
                Rgba::new(f[0], f[1], f[2], f[3])
            }).collect::<Vec<Rgba<u8>>>();
            slint_pixel_buffer.make_mut_slice().copy_from_slice(&buffer[..]);
            let image: Image = Image::from_rgba8(slint_pixel_buffer);
            frames.push(image);
        }
        gifs.insert(name, frames);
    }
    preload_gif(&mut gifs, "confetti.gif", include_bytes!("../confetti.gif"));
    app.on_load_gif_frame(move |a,b| {
        let g = &gifs[a.to_string().as_str()];
        let l = g.len();
        g[(b as f32 / 1000.0 * 15.0) as usize % l].clone()
    });
    app.on_start_timer(|| {
        *START.lock().unwrap() = Some(Instant::now());
    });
    app.on_set_pause_timer(|state| {
        *PAUSED.lock().unwrap() = state;
        if state {
            *PAUSED_AT.lock().unwrap() = Some(Instant::now());
        }
        if !state {
            let passed = PAUSED_AT.lock().unwrap().unwrap() - START.lock().unwrap().unwrap();
            *START.lock().unwrap() = Some(Instant::now()-passed);
        }
    });

    app.on_callback_ticker(move || {
        let now = 
        if *PAUSED.lock().unwrap() {PAUSED_AT.lock().unwrap().unwrap()} else {Instant::now()};
        START.lock().unwrap().map_or(0, |a| {(now - a).as_secs() as i32})
    });

    app.run().unwrap();
}
