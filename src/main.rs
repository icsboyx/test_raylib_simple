use rand::Rng;
use raylib::{ffi::rand, prelude::*};

struct Entity {
    x: f32,
    y: f32,
    radius: i32,
    color: Color,
    canvas: (i32, i32),
}
impl Entity {
    fn new(x: f32, y: f32, radius: i32, canvas: (i32, i32), color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            canvas,
        }
    }
    fn get_x(&self) -> f32 {
        self.x
    }
    fn set_x(&mut self, x: f32) {
        if x > self.canvas.0 as f32 + (self.radius / 2) as f32 {
            self.x = 0.0;
        } else if x < 0.0 - (self.radius / 2) as f32 {
            self.x = self.canvas.0 as f32;
        } else {
            self.x = x;
        }
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn set_y(&mut self, y: f32) {
        if y > self.canvas.1 as f32 + (self.radius / 2) as f32 {
            self.y = 0.0;
        } else if y < 0.0 - (self.radius / 2) as f32 {
            self.y = self.canvas.1 as f32;
        } else {
            self.y = y;
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius as f32, self.color);
    }
}

fn main() {
    let w = 640;
    let h = 480;
    let mut rnd = rand::thread_rng();

    let mut entity = Entity::new((h / 2) as f32, (w / 2) as f32, 40, (w, h), Color::RED);
    let mut target = Entity::new(
        rnd.gen_range(0..w - 20) as f32,
        rnd.gen_range(0..h - 20) as f32,
        20,
        (w, h),
        Color::BLUE,
    );
    let (mut rl, thread) = raylib::init().size(w, h).title("Hello, World").build();
    enum ArrowKey {
        Up,
        Down,
        Left,
        Right,
    }

    impl ArrowKey {
        fn to_raylib_key(&self) -> KeyboardKey {
            match self {
                ArrowKey::Up => KeyboardKey::KEY_UP,
                ArrowKey::Down => KeyboardKey::KEY_DOWN,
                ArrowKey::Left => KeyboardKey::KEY_LEFT,
                ArrowKey::Right => KeyboardKey::KEY_RIGHT,
            }
        }
        fn get_arrows_keys() -> Vec<ArrowKey> {
            vec![
                ArrowKey::Up,
                ArrowKey::Down,
                ArrowKey::Left,
                ArrowKey::Right,
            ]
        }
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        entity.draw(&mut d);
        target.draw(&mut d);

        for key in ArrowKey::get_arrows_keys() {
            if d.is_key_down(key.to_raylib_key()) {
                match key {
                    ArrowKey::Up => entity.set_y(entity.get_y() - 1.0),
                    ArrowKey::Down => entity.set_y(entity.get_y() + 1.0),
                    ArrowKey::Left => entity.set_x(entity.get_x() - 1.0),
                    ArrowKey::Right => entity.set_x(entity.get_x() + 1.0),
                }
            }
        }

        if check_collision_circles(
            Vector2::new(entity.get_x(), entity.get_y()),
            entity.radius as f32,
            Vector2::new(target.get_x(), target.get_y()),
            target.radius as f32,
        ) {
            target.set_x(rnd.gen_range(0..w - 20) as f32);
            target.set_y(rnd.gen_range(0..h - 20) as f32);
        }
    }
}
