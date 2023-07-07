use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Reversed").build();

    let mut camera =
        Camera3D::perspective(Vector3::zero(), Vector3::forward(), Vector3::up(), 60.0);
    rl.set_camera_mode(camera, CameraMode::CAMERA_FIRST_PERSON);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            rl.disable_cursor();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGREEN);

        {
            let mut d = d.begin_mode3D(camera);
            d.draw_cube(Vector3::one(), 1.0, 1.0, 1.0, Color::RED);
            d.draw_cube_wires(Vector3::one(), 1.0, 1.0, 1.0, Color::MAROON);
        }

        d.draw_fps(12, 12);
    }
}
