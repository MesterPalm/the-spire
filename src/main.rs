pub mod graphics_3d;
pub mod control;

use graphics_3d::{Point3d, Cube};
use sfml::graphics::{Color, RenderWindow, RenderTarget};
use sfml::window::{ContextSettings, Style};

fn main() {
    let resolution = (900, 900);
    let mut window = RenderWindow::new(
        resolution,
        "Hideo bames",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut camera = graphics_3d::Camera::default();
    let mut cont = control::ControllerState::default();
    let player_speed = 0.7;
    let player_look = 0.03;
    let mut tick = 0.;

    while window.is_open() {
        tick += 0.001;

        cont.update(&mut window);

        if cont.move_forward.held {
             camera.position += Point3d{x : -camera.rotation.y.sin()*player_speed, y : 0., z :  camera.rotation.y.cos()*player_speed};
        }
        if cont.move_backward.held {
             camera.position -= Point3d{x : -camera.rotation.y.sin()*player_speed, y : 0., z :  camera.rotation.y.cos()*player_speed};
        }
        if cont.move_down.held {
             camera.position += Point3d{x : 0., y : player_speed, z : 0.};
        }
        if cont.move_up.held {
             camera.position -= Point3d{x : 0., y : player_speed, z : 0.};
        }
        if cont.move_left.held {
             camera.position -= Point3d{x : camera.rotation.y.cos()*player_speed, y : 0., z :  camera.rotation.y.sin()*player_speed};
        }
        if cont.move_right.held {
             camera.position += Point3d{x : camera.rotation.y.cos()*player_speed, y : 0., z :  camera.rotation.y.sin()*player_speed};
        }
        if cont.look_left.held {
             camera.rotation += Point3d{x : 0., y : player_look, z : 0.};
        }
        if cont.look_right.held {
             camera.rotation -= Point3d{x : 0., y : player_look, z : 0.};
        }
        if cont.look_up.held {
             camera.rotation -= Point3d{x : player_look, y : 0., z : 0.};
        }
        if cont.look_down.held {
             camera.rotation += Point3d{x : player_look, y : 0., z : 0.};
        }
        if cont.look_cw.held {
             camera.rotation -= Point3d{x : 0., y : 0., z : player_look};
        }
        if cont.look_ccw.held {
             camera.rotation += Point3d{x : 0., y : 0., z : player_look};
        }
        if cont.quit.press {
            window.close();
        }


        const N : usize = 1000;
        const DIST : f32 = 7.;
        const RAD : f32 = 0.25;
        const START_HEIGHT : f32 = 5000.;
        
        let mut cubes = [Cube::default(camera, resolution); N];
        for i in 0..N {
            cubes[i] = Cube{
                center : Point3d {x : (i as f32*RAD + tick).cos()*100., y : i as f32 * DIST - START_HEIGHT, z : (i as f32*RAD + tick).sin()*100.},
                dimensions : Point3d{x : 10., y : 10., z : 10.},
                rotations : Point3d::origin(),
                camera,
                resolution
            };
        }

        window.clear(Color::BLACK);
        for cube in cubes {
            window.draw(&cube);
        }
        window.display();
    }

}
