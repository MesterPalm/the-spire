use sfml::graphics::{Color, RectangleShape, CircleShape, RenderWindow, RenderTarget, RenderStates, PrimitiveType, Vertex, Drawable};
use sfml::system::{Clock, Vector2f};
use std::ops;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub center : Point3d,
    pub dimensions : Point3d,
    pub rotations : Point3d,
    pub camera : Camera,
    pub resolution : (u32, u32)
}

impl Cube {
    pub fn default(camera : Camera, resolution : (u32, u32)) -> Cube {
        return Cube{
            center : Point3d::origin(),
            dimensions : Point3d::ones(),
            rotations : Point3d::origin(),
            camera,
            resolution
        };
    }
}


impl Drawable for Cube {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target : &mut dyn RenderTarget,
        states : &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let point1 = self.center + self.dimensions/2.;
        let point2 = point1 - Point3d{x : self.dimensions.x, y : 0., z : 0.};
        let point3 = point1 - Point3d{x : 0., y : self.dimensions.y, z : 0.};
        let point4 = point1 - Point3d{x : self.dimensions.x, y : self.dimensions.y, z : 0.};
        let point5 = point1 - Point3d{x : 0., y : 0., z : self.dimensions.z};
        let point6 = point1 - Point3d{x : self.dimensions.x, y : 0., z : self.dimensions.z};
        let point7 = point1 - Point3d{x : 0., y : self.dimensions.y, z : self.dimensions.z};
        let point8 = point1 - Point3d{x : self.dimensions.x, y : self.dimensions.y, z : self.dimensions.z};

        let line1 = Line::from_point3d(point1, point2, &self.camera, self.resolution);
        let line2 = Line::from_point3d(point1, point3, &self.camera, self.resolution);
        let line3 = Line::from_point3d(point1, point5, &self.camera, self.resolution);
        let line4 = Line::from_point3d(point8, point7, &self.camera, self.resolution);
        let line5 = Line::from_point3d(point8, point6, &self.camera, self.resolution);
        let line6 = Line::from_point3d(point8, point4, &self.camera, self.resolution);
        let line7 = Line::from_point3d(point7, point3, &self.camera, self.resolution);
        let line8 = Line::from_point3d(point6, point2, &self.camera, self.resolution);
        let line9 = Line::from_point3d(point4, point3, &self.camera, self.resolution);
        let line10 = Line::from_point3d(point4, point2, &self.camera, self.resolution);
        let line11 = Line::from_point3d(point6, point5, &self.camera, self.resolution);
        let line12 = Line::from_point3d(point5, point7, &self.camera, self.resolution);
        
        target.draw(&line1);
        target.draw(&line2);
        target.draw(&line3);
        target.draw(&line4);
        target.draw(&line5);
        target.draw(&line6);
        target.draw(&line7);
        target.draw(&line8);
        target.draw(&line9);
        target.draw(&line10);
        target.draw(&line11);
        target.draw(&line12);
    }
}



// Represents a Point in 3D space, used in to convert a 3D point to 2D point on screen
#[derive(Debug, Clone, Copy)]
pub struct Point3d {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl ops::Add<Point3d> for Point3d {
    type Output = Point3d;

    fn add(self, rhs : Point3d) -> Point3d {
        return Point3d{x : self.x + rhs.x, y : self.y + rhs.y, z : self.z + rhs.z};
    }
}

impl ops::AddAssign<Point3d> for Point3d {
    fn add_assign(&mut self, rhs : Point3d) {
        *self = *self + rhs; 
    }
}

impl ops::Sub<Point3d> for Point3d {
    type Output = Point3d;

    fn sub(self, rhs : Point3d) -> Point3d {
        return Point3d{x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z};
    }
}

impl ops::SubAssign<Point3d> for Point3d {
    fn sub_assign(&mut self, rhs : Point3d) {
        *self = *self - rhs; 
    }
}

impl ops::Div<f32> for Point3d {
    type Output = Point3d;
    
    fn div(self, rhs : f32) -> Point3d {
        return Point3d{x : self.x/rhs, y : self.y/rhs, z : self.z/rhs};
    }

}

impl ops::DivAssign<f32> for Point3d {
    fn div_assign(&mut self, rhs : f32) {
        *self = *self / rhs;
    }

}

impl Point3d {
    pub fn origin() -> Point3d {
        return Point3d{x : 0., y : 0., z : 0.};
    }

    pub fn ones() -> Point3d {
        return Point3d{x : 1., y : 1., z : 1.};
    }

    pub fn project_2d(&self, fov : f32) -> (f32, f32) {
        // return (x,y) as portion of view 0 in the middle, -1 to the left/up and 1 right/down 
        // Assume camera at (0,0,0) and looking straight into the z line
        let theta = (PI - fov) / 2.; // angle of edge of fov area to x axis
        let hypot = self.z/theta.sin(); // hypothenuse 
        let view = hypot*theta.cos();
        if self.z < 0. {
            return (-self.x/view, -self.y/view);
        }
        return (self.x/view, self.y/view);
    }

    pub fn rotate_around_x(&self, radians : f32, offset : Point3d) -> Point3d {
        let off_y = self.y - offset.y;
        let off_z = self.z - offset.z;
        let rot_y = off_y*radians.cos() - off_z*radians.sin();
        let rot_z = off_y*radians.sin() + off_z*radians.cos();
        return Point3d{x : self.x, y : rot_y + offset.y, z : rot_z + offset.z};
    }

    pub fn rotate_around_y(&self, radians : f32, offset : Point3d) -> Point3d {
        let off_x = self.x - offset.x;
        let off_z = self.z - offset.z;
        let rot_x = off_x*radians.cos() + off_z*radians.sin();
        let rot_z = -off_x*radians.sin() + off_z*radians.cos();
        return Point3d{x : rot_x + offset.x, y : self.y, z : rot_z + offset.z};
    }

    pub fn rotate_around_z(&self, radians : f32, offset : Point3d) -> Point3d {
        let off_x = self.x - offset.x;
        let off_y = self.y - offset.y;
        let rot_x = off_x*radians.cos() - off_y*radians.sin();
        let rot_y = off_x*radians.sin() + off_y*radians.cos();
        return Point3d{x : rot_x - offset.x, y : rot_y - offset.y, z : self.z};
    }
}


// Line between two points space
pub struct Line {
    vertices : [Vertex; 4]
}

impl Line {
    pub fn new(x1 : f32, y1 : f32, x2 : f32, y2 : f32) -> Line {
        let mut line = Line{ vertices : [Vertex::default(); 4]};
        line.vertices = [
            Vertex::new(Vector2f::new(x1-1., y1-1.), Color::RED, Vector2f::new( 0.,  0.)),
            Vertex::new(Vector2f::new(x1+1., y1+1.), Color::RED, Vector2f::new( 0., 10.)),
            Vertex::new(Vector2f::new(x2+1., y2+1.), Color::RED, Vector2f::new(10., 10.)),
            Vertex::new(Vector2f::new(x2-1., y2-1.), Color::RED, Vector2f::new(10.,  0.))
        ];
        return line;
    }
    
    pub fn from_point3d(p1 : Point3d, p2 : Point3d, camera : &Camera, resolution :  (u32, u32)) -> Line {
        let x_resolution = resolution.0 as f32;
        let y_resolution = resolution.1 as f32;
        let p1 = p1 - camera.position;
        let p1 = p1.rotate_around_y(camera.rotation.y, Point3d::origin());
        let p1 = p1.rotate_around_x(camera.rotation.x, Point3d::origin());
        let p1 = p1.rotate_around_z(camera.rotation.z, Point3d::origin());
        let p2 = p2 - camera.position;
        let p2 = p2.rotate_around_y(camera.rotation.y, Point3d::origin());
        let p2 = p2.rotate_around_x(camera.rotation.x, Point3d::origin());
        let p2 = p2.rotate_around_z(camera.rotation.z, Point3d::origin());
        let (x1, y1) = p1.project_2d(camera.fov);
        let (x1, y1) = (x1*(x_resolution/2.) + x_resolution/2., y1*(y_resolution/2.) + y_resolution/2.);
        let (x2, y2) = p2.project_2d(camera.fov);
        let (x2, y2) = (x2*(x_resolution/2.) + x_resolution/2., y2*(y_resolution/2.) + y_resolution/2.);
        return Line::new(x1,y1,x2,y2);
    }
}

impl Drawable for Line {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target : &mut dyn RenderTarget,
        states : &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_primitives(&self.vertices, PrimitiveType::QUADS, &RenderStates::DEFAULT);
    }
}

// Represents the camera settings
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub fov : f32,
    pub position : Point3d,
    pub rotation : Point3d //pitch (x), yaw (y), roll (z)
}

impl Camera {
    pub fn default() -> Camera {
        return Camera{
            fov : PI/2., 
            position : Point3d{x : 0., y : 0., z : 0.}, 
            rotation : Point3d{x : 0., y : 0., z : 0.}
        };
    }
}
