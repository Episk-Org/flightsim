use cgmath::{Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Vector3, Vector4};
use glow_glyph::{GlyphBrushBuilder, ab_glyph, Section, Text};

use crate::{camera::Camera, input_handler::KeyState, model::Model, window_handler::GlContext};

pub struct Plane {
    body: Model,
    propeller: Model,

    scale: f32,
    position: Vector3<f32>,

    forward: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,

    pitch_velocity: f32,
    roll_velocity: f32,

    velocity: Vector3<f32>,

    propeller_rotation: f32,
    camera_offset: Vector3<f32>,
}

impl Plane {
    pub const PITCH_SPEED: f32 = 0.00005;
    pub const ROLL_SPEED: f32 = 0.0001;
    pub const GRAVITY: f32 = 0.001;
    pub const DRAG: f32 = 0.003;
    pub const LIFT: f32 = 0.0005;

    pub fn new(body: Model, propeller: Model, position: Vector3<f32>) -> Self {
        Self {
            body,
            propeller,
            scale: 0.25,
            position,

            forward: Vector3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            right: Vector3::new(-1.0, 0.0, 0.0),

            pitch_velocity: 0.0,
            roll_velocity: 0.0,

            velocity: Vector3::new(0.0, 0.0, 1.0),
            propeller_rotation: 0.0,
            camera_offset: Vector3::new(0.0, 3.0, -12.0),
        }
    }

    // Solution based on https://cs.lmu.edu/~ray/notes/flightsimulator/
    fn pitch(&mut self, angle: f32) {
        self.forward = (self.forward * angle.cos() + self.up * angle.sin()).normalize();
        self.up = self.right.cross(self.forward);
    }

    fn roll(&mut self, angle: f32) {
        self.right = (self.right * angle.cos() + self.up * angle.sin()).normalize();
        self.up = self.right.cross(self.forward);
    }

    
    pub fn update(&mut self, key_state: &KeyState) {
        //pitch
        if key_state.up {
            self.pitch_velocity -= Self::PITCH_SPEED;
        }
        if key_state.down {
            self.pitch_velocity += Self::PITCH_SPEED;
        }

        if key_state.left {
            self.roll_velocity += Self::ROLL_SPEED;
        }

        if key_state.right {
            self.roll_velocity -= Self::ROLL_SPEED;
        }

        self.pitch_velocity *= 1.0 - 0.01;
        self.roll_velocity *= 1.0 - 0.01;

        self.pitch(self.pitch_velocity);
        self.roll(self.roll_velocity);

        

        let acc = if key_state.turbo {
            0.05
        } else if key_state.accelerate {
            0.005
        } else {
            0.0005
        };

        let steer = (self.forward - self.velocity) * 0.003;
        //println!("{}", steer.magnitude());

        self.velocity += self.forward * acc;
        self.velocity += steer;

        
        self.velocity.y -= (Self::GRAVITY - Self::LIFT * self.forward.magnitude()).clamp(0.0, Self::GRAVITY);

        self.velocity.y -= Self::GRAVITY;
        let hor = self.velocity.xz();
        self.velocity.y += hor.magnitude() * Self::LIFT;

        self.velocity *= 1.0 - Self::DRAG;

        self.position += self.velocity;
        //println!("{:?}",self.forward);
        //println!("{}", self.velocity.magnitude());
    }

    pub fn render(&mut self, gl: &GlContext, time:&f32, camera: &mut Camera) {
        camera.perspective.fovy = Deg((70.0 + self.velocity.magnitude() * 40.0).clamp(70.0, 105.0)).into();
        let translation = Matrix4::from_translation(self.position);

        let right = self.right;
        let forward = self.forward;
        let up = self.up;

        let plane_rot = Matrix4::new(
            right.x, right.y, right.z, 0.0, up.x, up.y, up.z, 0.0, forward.x, forward.y, forward.z,
            0.0, 0.0, 0.0, 0.0, 1.0,
        );

        let scale_matrix = Matrix4::from_scale(self.scale);
        let mut matrix = translation * plane_rot * scale_matrix;
        let camera_position = (matrix
            * Matrix4::from_translation(self.camera_offset)
            * Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0))
        .xyz();
        camera.eye = Point3::from_vec(camera_position);
        camera.direction = self.forward;
        camera.up = self.up;
        camera.update_view();


        let inconsolata = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../assets/Inconsolata-Regular.ttf"
        )).unwrap();

        let mut glyph_brush = GlyphBrushBuilder::using_font(inconsolata).build(gl);

        glyph_brush.queue(Section {
            screen_position: (10.0, 10.0),
            bounds: (800.0, 600.0),
            text: vec![Text::default()
                .with_text(&format!("fps:{}", (1.0/time) as u16))
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(10.0)],
            ..Section::default()
        });

        glyph_brush
        .draw_queued(gl, 800, 600)
        .expect("Draw queued");

        self.body.render(gl, matrix, time, camera);

        let offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, -0.1935, 0.0));
        let rev_offset = Matrix4::from_translation(Vector3::<f32>::new(0.0, 0.1935, 0.0));
        self.propeller_rotation += *time * 20.0 * self.velocity.magnitude();

        let rotation = Matrix4::from_angle_z(Rad(self.propeller_rotation));

        let spin = offset * rotation * rev_offset;
        matrix = matrix * spin;

        self.propeller
            .render(gl, matrix, time, camera);
    }
}
