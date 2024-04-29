use macroquad::*;

fn main() {
    // Initialize the game window and set up the 3D scene
    let mut win = Window::new("My Game");
    let mut scene = Scene::default();

    // Import the FBX model
    let model = Model::load_fbx("path/to/model.fbx").unwrap();

    // Create materials for the model
    let mut materials = Vec::new();
    for material in model.materials() {
        let mut m = Material::new();
        m.set_color(Color::rgb(0.5, 0.5, 0.5));
        m.set_specular(Color::rgb(0.5, 0.5, 0.5));
        m.set_shininess(32.0);
        materials.push(m);
    }
    model.set_materials(&materials);

    // Create an entity for the model and add it to the scene
    let mut entity = Entity::new();
    entity.add_component(model);
    scene.add_entity(entity);

    // Set up animations for the model
    let mut animator = Animator::new(model);
    let animation = model.animation(0).unwrap();
    animator.play(animation, LoopMode::Loop);

    // Main game loop
    while !win.should_close() {
        // Update the scene
        scene.update();

        // Update the animator
        animator.update();

        // Render the scene
        win.draw_3d(&scene, Camera::standard_3d(win.width(), win.height()));
    }
}
