mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalShapes::Circle => {
            rectangle_area(x, y) as f64 >= circle_area(a) * times as f64
        }
        areas_volumes::GeometricalShapes::Rectangle => {
            rectangle_area(x, y) >= rectangle_area(a, b) * times
        }
        areas_volumes::GeometricalShapes::Square => rectangle_area(x, y) >= square_area(a) * times,
        areas_volumes::GeometricalShapes::Triangle => {
            rectangle_area(x, y) as f64 >= triangle_area(a, b) * times as f64
        }
    }
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    match objects {
        areas_volumes::GeometricalVolumes::Cone => {
            parallelepiped_volume(x, y, z) as f64 >= cone_volume(a, b) * times as f64
        }
        areas_volumes::GeometricalVolumes::Cube => {
            parallelepiped_volume(x, y, z) >= cube_volume(a) * times
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            parallelepiped_volume(x, y, z) >= parallelepiped_volume(a, b, c) * times
        }
        areas_volumes::GeometricalVolumes::Pyramid => {
            parallelepiped_volume(x, y, z) as f64
                >= triangular_pyramid_volume(a as f64, b) * times as f64
        }
        areas_volumes::GeometricalVolumes::Sphere => {
            parallelepiped_volume(x, y, z) as f64 >= sphere_volume(a) * times as f64
        }
    }
}
