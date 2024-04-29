fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
use nalgebra::Vector2;

fn cubic_spline(points: &[Vector2<f32>], t: f32) -> Vector2<f32> {
    let n = points.len();
    let i = ((n as f32) * t) as usize;
    let u = (n as f32) * t - (i as f32);

    let a = points[i % n];
    let b = points[(i + 1) % n];
    let c = points[(i + 2) % n];
    let d = points[(i + 3) % n];

    let p = (1.0 - u) * (1.0 - u) * (1.0 - u);
    let q = 3.0 * u * (1.0 - u) * (1.0 - u);
    let r = 3.0 * u * u * (1.0 - u);
    let s = u * u * u;

    a * p + b * q + c * r + d * s
}

// Ease in and out functions:

fn ease_in_quad(t: f32) -> f32 {
    t * t
}

fn ease_out_quad(t: f32) -> f32 {
    t * (2.0 - t)
}

fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = t - 1.0;
        (t * t * t) + 1.0
    }
}

fn ease_in_quart(t: f32) -> f32 {
    t * t * t * t
}

fn ease_out_quart(t: f32) -> f32 {
    let t = t - 1.0;
    1.0 - t * t * t * t
}

fn ease_in_out_quart(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        let t = t - 1.0;
        1.0 - 8.0 * t * t * t * t
    }
}

fn ease_in_quint(t: f32) -> f32 {
    t * t * t * t * t
}

fn ease_out_quint(t: f32) -> f32 {
    let t = t - 1.0;
    1.0 + t * t * t * t * t
}

fn ease_in_out_quint(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        let t = t - 1.0;
        1.0 + 16.0 * t * t * t * t * t
    }
}

// Ease in and out functions with overshoot:

fn ease_in_back(t: f32) -> f32 {
    t * t * t - t * std::f32::consts::PI / 2.0
}

fn ease_out_back(t: f32) -> f32 {
    let t = t - 1.0;
    1.0 + t * t * t + t * std::f32::consts::PI / 2.0
}

fn ease_in_out_back(t: f32) -> f32 {
    if t < 0.5 {
        let t = 2.0 * t;
        0.5 * t * t * t - t * std::f32::consts::PI / 2.0
    } else {
        let t = (2.0 * t) - 2.0;
        0.5 * (t * t * t + 2.0) + (t * std::f32::consts::PI / 2.0)
    }
}

fn ease_in_elastic(t: f32) -> f32 {
    let t = t - 1.0;
    -(2.0f32.powf(10.0 * t) * (t * std::f32::consts::PI / 4.5).sin())
}

fn ease_out_elastic(t: f32) -> f32 {
    2.0f32.powf(-10.0 * t) * (t * std::f32::consts::PI / 4.5).sin() + 1.0
}

fn ease_in_out_elastic(t: f32) -> f32 {
    if t < 0.5 {
        let t = (2.0 * t) - 1.0;
        -0.5 * (2.0f32.powf(10.0 * t) * (t * std::f32::consts::PI / 4.5).sin())
    } else {
        let t = (2.0 * t) - 1.0;
        0.5 * (2.0f32.powf(-10.0 * t) * (t * std::f32::consts::PI / 4.5).sin()) + 1.0
    }
}

fn ease_in_bounce(t: f32) -> f32 {
    1.0 - ease_out_bounce(1.0 - t)
}

fn ease_out_bounce(t: f32) -> f32 {
    if t < 4.0 / 11.0 {
        (121.0 * t * t) / 16.0
    } else if t < 8.0 / 11.0 {
        (363.0 / 40.0 * t * t) - (99.0 / 10.0 * t) + 17.0 / 5.0
    } else if t < 9.0 / 10.0 {
        (4356.0 / 361.0 * t * t) - (35442.0 / 1805.0 * t) + 16061.0 / 1805.0
    } else {
        (54.0 / 5.0 * t * t) - (513.0 / 25.0 * t) + 268.0 / 25.0
    }
}

fn ease_in_out_bounce(t: f32) -> f32 {
    if t < 0.5 {
        0.5 * ease_in_bounce(t * 2.0)
    } else {
        0.5 * ease_out_bounce(t * 2.0 - 1.0) + 0.5
    }
}
