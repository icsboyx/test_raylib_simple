use rand::Rng;
use raylib::prelude::*;

struct Entity {
    x: f32,
    y: f32,
    radius: i32,
    color: Color,
    canvas: (i32, i32),
    speed: f32,
    past_xy: (f32, f32),
}
impl Entity {
    fn new(x: f32, y: f32, radius: i32, canvas: (i32, i32), color: Color, speed: f32) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            canvas,
            speed,
            past_xy: (x, y),
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
        self.past_xy.0 = self.x;
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
        self.past_xy.1 = self.y;
    }
    fn move_dir(&mut self, direction: ArrowKey) {
        match direction {
            ArrowKey::Up => self.set_y(self.get_y() - self.speed),
            ArrowKey::Down => self.set_y(self.get_y() + self.speed),
            ArrowKey::Left => self.set_x(self.get_x() - self.speed),
            ArrowKey::Right => self.set_x(self.get_x() + self.speed),
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius as f32, self.color);
    }
}

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

fn main() {
    let w = 640;
    let h = 480;
    let mut rnd = rand::thread_rng();

    let mut entity = Entity::new((h / 2) as f32, (w / 2) as f32, 40, (w, h), Color::RED, 2.0);
    let mut target = Entity::new(
        rnd.gen_range(0..w - 20) as f32,
        rnd.gen_range(0..h - 20) as f32,
        20,
        (w, h),
        Color::BLUE,
        0.0,
    );
    let (mut rl, thread) = raylib::init().size(w, h).title("Hello, World").build();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(1, 1);
        d.clear_background(Color::WHITE);
        entity.draw(&mut d);
        target.draw(&mut d);

        for key in ArrowKey::get_arrows_keys() {
            if d.is_key_down(key.to_raylib_key()) {
                entity.move_dir(key)
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
