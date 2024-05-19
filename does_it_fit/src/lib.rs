mod  areas_volumes;
pub use areas_volumes::*;
pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let rec_area = x * y;
    match objects {
        GeometricalShapes::Square => {
            let shape_area = square_area(a);
            return shape_area * times <= rec_area;
        }
        GeometricalShapes::Circle => {
            let shape_area =circle_area(a);
            return shape_area * times as f64 <= rec_area as f64;
        }
        GeometricalShapes::Triangle => {
            let shape_area = triangle_area(a, b);
            return shape_area * times as f64 <= rec_area as f64;
        }
        GeometricalShapes::Rectangle => {
            let shape_area = rectangle_area(a, b);
            return shape_area * times  <= rec_area;
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
    let box_vol = areas_volumes::parallelepiped_volume(x,y,z);
    match objects {
        GeometricalVolumes::Sphere=>{
            let shape_vol = areas_volumes::sphere_volume(a);
            return  shape_vol*times as f64 <= box_vol as f64
        },
        GeometricalVolumes::Cube=>{
            let shape_vol = areas_volumes::cube_volume(a);
            return  shape_vol*times <= box_vol
        }
        GeometricalVolumes::Cone=>{
            let shape_vol = areas_volumes::cone_volume(a,b);
            return  shape_vol*times as f64 <= box_vol as f64
        }
        GeometricalVolumes::Parallelepiped=>{
            let shape_vol = areas_volumes::parallelepiped_volume(a,b,c);
            return  shape_vol*times <= box_vol
        }
        GeometricalVolumes::Pyramid=>{
            let shape_vol = areas_volumes::triangular_pyramid_volume(a as f64,b);
            return  shape_vol*times as f64 <= box_vol as f64
        }
    }
}
