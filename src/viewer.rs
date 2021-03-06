//! The Kiss3D App is created.
use crate::renderer::PointCloudRenderer;
use kiss3d::camera::Camera;
use kiss3d::event::{Key, WindowEvent};
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
use na::{Point2, Point3};
use nalgebra as na;
use std::path::Path;

/// Global State for the app
pub struct AppState {
    /// The Renderer for the App to use
    pub point_cloud_renderer: PointCloudRenderer,
}

impl State for AppState {
    // Return the custom renderer that will be called
    // at each render loop.
    #[allow(clippy::type_complexity)]
    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (None, None, Some(&mut self.point_cloud_renderer), None)
    }

    fn step(&mut self, window: &mut Window) {
        let num_points_text = format!("Number of points: {}", self.point_cloud_renderer.num_points());
        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            50.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        );
        for event in window.events().iter() {
            if let WindowEvent::Key(key, action, modif) = event.value {
                println!("key event {:?} on {:?} with {:?}", key, action, modif);
                if key == Key::Q {
                    window.close();
                } else if key == Key::S {
                    let img = window.snap_image();
                    let img_path = Path::new("screenshot.png");
                    img.save(img_path).unwrap();
                    println!("Screeshot saved to `screenshot.png`");
                }
            }
        }
    }
}
