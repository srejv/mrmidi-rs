fn interpolate(x: f32, points: &[(f32, f32)]) -> f32 {
    // Find the two points that x falls between
    let (lower, upper) = points
        .iter()
        .zip(points.iter().skip(1))
        .find(|((x1, _), (x2, _))| x1 <= &x && x < x2)
        .unwrap();

    // Interpolate between the two points
    let (x1, y1) = lower;
    let (x2, y2) = upper;
    y1 + (y2 - y1) * (x - x1) / (x2 - x1)
}

let mut points = vec![(2.0, 3.0), (1.0, 5.0), (3.0, 7.0)];
points.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());

let y = interpolate(2.0, &points);
println!("Interpolated y value: {}", y);



fn lagrange_interpolate(x: f32, points: &[(f32, f32)]) -> f32 {
    let n = points.len() - 1;

    let mut y = 0.0;
    for i in 0..=n {
        let (x_i, y_i) = points[i];
        let mut term = y_i;
        for j in 0..=n {
            if i != j {
                let (x_j, _) = points[j];
                term *= (x - x_j) / (x_i - x_j);
            }
        }
        y += term;
    }

    y
}


use macroquad::*;

// Custom component to store animation curve data
#[derive(Clone, Debug)]
struct AnimationCurve {
    // Start and end positions
    start: (f32, f32),
    end: (f32, f32),
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
            self.start
        } else {
            let (x1, y1) = self.start;
            let (x2, y2) = self.end;
            (x1 + (x2 - x1) * t, y1 + (y2 - y1) * t)
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
            start: (0.0, 0.0),
            end: (100.0, 100.0),
            duration: 2.0,
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
