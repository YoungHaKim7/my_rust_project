mod canvas;
mod tuple;

use std::fs::File;
use std::io::prelude::*;

fn virtual_cannon() {
    use tuple::{Color, Point, Vector};

    struct Projectile {
        position: Point,
        velocity: Vector,
    }

    struct Environment {
        gravity: Vector,
        wind: Vector,
    }

    fn tick(env: &Environment, projectile: Projectile) -> Projectile {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + env.gravity + env.wind;
        Projectile { position, velocity }
    }

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 7.85,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = canvas::Canvas::new(500, 300);

    while projectile.position.y > 0.0 {
        projectile = tick(&env, projectile);
        let pos = projectile.position;
        let color = Color::new(1.0, 0.0, 1.0);
        let pos_y = canvas.height - (pos.y as i32);
        if pos_y <= canvas.height {
            canvas.write_pixel(pos.x as i32, pos_y, color);
        }
    }

    let ppm = canvas.to_ppm();
    let mut file = File::create("images/cannon.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}

fn main() {
    virtual_cannon()
}
