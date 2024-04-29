use macroquad::*;

// Custom component to store animation curve data
#[derive(Clone, Debug)]
struct AnimationCurve {
    // List of points
    points: Vec<(f32, f32)>,
    // Animation duration in seconds
    duration: f32,
    // Elapsed time in seconds
    elapsed: f32,
}

impl AnimationCurve {
    // Update the elapsed time and return the interpolated position
    fn update(&mut self, dt: f32) -> (f32, f32) {
        self.elapsed += dt;
        let t = self.elapsed / self.duration;
        if t >= 1.0 {
            self.elapsed = 0.0;
            self.points[0]
        } else {
            // Find the two points surrounding t
            let n = self.points.len();
            let i = (t * n as f32).floor() as usize;
            let x1 = self.points[i].0;
            let y1 = self.points[i].1;
            let x2 = self.points[(i + 1) % n].0;
            let y2 = self.points[(i + 1) % n].1;
            // Interpolate between the two points
            let t_ = t * n as f32 - i as f32;
            (x1 + (x2 - x1) * t_, y1 + (y2 - y1) * t_)
        }
    }
}

// Custom system to update entities with the AnimationCurve component
struct AnimationCurveSystem;

impl<'a> System<'a> for AnimationCurveSystem {
    type SystemData = (WriteStorage<'a, AnimationCurve>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut curves, mut positions): Self::SystemData) {
        for (curve, pos) in (&mut curves, &mut positions).join() {
            let (x, y) = curve.update(time::delta_time());
            pos.0 = x;
            pos.1 = y;
        }
    }
}

#[macroquad::main("Animation Curves")]
async fn main() {
    // Create an entity with an AnimationCurve component and a Position component
    let mut entity = world::create_entity();
    entity.add(
        AnimationCurve {
            points: vec![(0.0, 0.0), (100.0, 100.0), (200.0, 0.0)],
            duration: 4.0,
            elapsed: 0.0,
        },
    );
    entity.add(Position(0.0, 0.0));

    // Add the custom AnimationCurveSystem to the world
    world::add_system(AnimationCurveSystem);

    loop {
        clear_background(BLACK);

        // Draw the entity at its current position
        let pos = positions.get(entity).unwrap();
        draw_circle(pos.0, pos.1, 10.0, WHITE);

        end_frame();
    }
}
