use opengl_graphics::TextureSettings;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use find_folder::Search;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    texture: Texture, // The PNG texture.
    x: f64, // X position of the image.
    y: f64, // Y position of the image.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            let transform = c.transform.trans(self.x, self.y);

            Image::new().draw(&self.texture, &DrawState::default(), transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Update game state
    }

    fn pressed(&mut self, args: &Button) {
        const MOVE_SPEED: f64 = 10.0;
        
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => self.y -= MOVE_SPEED,
                Key::S => self.y += MOVE_SPEED,
                Key::A => self.x -= MOVE_SPEED,
                Key::D => self.x += MOVE_SPEED,
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Simple Moving PNG", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let texture = Texture::from_path(assets.join("Gordon_Freeman.png"), &TextureSettings::new()).unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        texture,
        x: 0.0,
        y: 0.0,
    };

    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.pressed(&Button::Keyboard(key));
        }
    }
}