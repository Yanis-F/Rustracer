use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use egui::Color32;

use crate::{color::Rgb, math::*, raytracer, scene::Scene};

pub struct Renderer {
    thread_handles: Vec<JoinHandle<()>>,
    renderer_data: Arc<RendererData>,
}

struct RendererData {
    pub size: [usize; 2],
    scene: Scene,
    data: Mutex<RefCell<Vec<Color32>>>,

    bail_threads: AtomicBool,

    virtual_screen_pixel_size_x: Vector3,
    virtual_screen_pixel_size_y: Vector3,
    virtual_screen_relative_topleft_pixel_center: Vector3,
}

impl Renderer {
    pub fn new(size: [usize; 2], scene: Scene) -> Renderer {
        let mut renderer = Renderer {
            thread_handles: vec![],
            renderer_data: Arc::new(RendererData::new(
                size,
                scene,
                Mutex::new(RefCell::new(vec![Color32::GRAY; size[0] * size[1]])),
                AtomicBool::new(false),
            )),
        };

        let num_cpus = num_cpus::get();
        let num_pixels = size[0] * size[1];

        for i in 0..num_cpus {
            let renderer_data_arc = renderer.renderer_data.clone();
            let starting_pixel_inclusive =
                (num_pixels as f32 * (i as f32 / num_cpus as f32)).floor() as usize;
            let ending_pixel_exclusive =
                (num_pixels as f32 * ((1.0 + i as f32) / num_cpus as f32)).floor() as usize;

            let starting_coordinate_inclusive = [
                starting_pixel_inclusive % size[0],
                starting_pixel_inclusive / size[0],
            ];
            let ending_coordinate_exclusive = [
                ending_pixel_exclusive % size[0],
                ending_pixel_exclusive / size[0] + 1,
            ];

            renderer.thread_handles.push(thread::spawn(move || {
                (*renderer_data_arc)
                    .render_thread_run(starting_coordinate_inclusive, ending_coordinate_exclusive);
            }));
        }

        renderer
    }

    /// Returns an image of `self.get_image_size()` size
    pub fn get_image(&self) -> Vec<Color32> {
        let refcell_mutexguard = self.renderer_data.data.lock().unwrap();
        let refcell_ref = (*refcell_mutexguard).clone(); // clones entire image :(
        refcell_ref.take()
    }

    /// Returns f32 between `0` (not started) and `1` (finished)
    pub fn get_progress(&self) -> f32 {
        0.42
    }

    pub fn get_image_size(&self) -> [usize; 2] {
        self.renderer_data.size
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.renderer_data
            .bail_threads
            .store(true, Ordering::Relaxed);

        let handles = std::mem::take(&mut self.thread_handles);
        handles.into_iter().for_each(|thread| {
            thread.join().unwrap();
        });
    }
}

impl RendererData {
    pub fn new(
        size: [usize; 2],
        scene: Scene,
        data: Mutex<RefCell<Vec<Color32>>>,
        bail_threads: AtomicBool,
    ) -> Self {
        let axis_up = rotate_vector(scene.camera.orientation, VECTOR3_UP);
        let axis_forward = rotate_vector(scene.camera.orientation, VECTOR3_FORWARD);
        let axis_right = rotate_vector(scene.camera.orientation, VECTOR3_RIGHT);

        let virtual_screen_center = axis_forward;
        let width_height_ratio = size[0] as f64 / size[1] as f64;

        // topleft corner of topleft-most pixel of the virtual screen
        let virtual_screen_topleft = {
            let relative_up = vec3_scale(axis_up, 0.5);
            let relative_left = vec3_scale(axis_right, -0.5 * width_height_ratio);

            let result = virtual_screen_center;
            let result = vec3_add(result, relative_up);
            let result = vec3_add(result, relative_left);

            #[allow(clippy::let_and_return)]
            result
        };

        let virtual_screen_pixel_size_x = vec3_scale(axis_right, width_height_ratio / size[0] as f64);
        let virtual_screen_pixel_size_y = vec3_scale(axis_up, -1.0 / size[1] as f64);

		let virtual_screen_relative_topleft_pixel_center = 
			vec3_add(
				virtual_screen_topleft,
				vec3_scale(
					vec3_add(virtual_screen_pixel_size_x, virtual_screen_pixel_size_y),
					0.5,
				)
			);

        Self {
            size,
            scene,
            data,
            bail_threads,
            virtual_screen_pixel_size_x,
            virtual_screen_pixel_size_y,
            virtual_screen_relative_topleft_pixel_center,
        }
    }

    pub fn render_thread_run(
        &self,
        starting_coordinate_inclusive: [usize; 2],
        ending_coordinate_exclusive: [usize; 2],
    ) {
        for y in starting_coordinate_inclusive[1]..ending_coordinate_exclusive[1] {
            let start_x = if y == starting_coordinate_inclusive[1] {
                starting_coordinate_inclusive[0]
            } else {
                0
            };
            let end_x = if y + 1 == ending_coordinate_exclusive[1] {
                ending_coordinate_exclusive[0]
            } else {
                self.size[0]
            };

            for x in start_x..end_x {
                let color = self.get_pixel_color(x, y);

                self.data.lock().unwrap().get_mut()[x + y * self.size[0]] = color.into();

                if self.bail_threads.load(Ordering::Relaxed) {
                    return;
                }
            }
        }
    }

    /// x is rightwards
    /// y is upwards
    fn get_pixel_color(&self, x: usize, y: usize) -> Rgb {
        let target_pixel_center_relative_position = 
			vec3_add(
				self.virtual_screen_relative_topleft_pixel_center,
				vec3_add(
					vec3_scale(self.virtual_screen_pixel_size_x, x as f64),
					vec3_scale(self.virtual_screen_pixel_size_y, y as f64),
				)
			);

        let ray = Ray {
            origin: self.scene.camera.position,
            direction: vec3_normalized(target_pixel_center_relative_position),
        };

        raytracer::raytracer(&self.scene, &ray)
    }
}
