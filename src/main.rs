mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;



fn write_ppm(w:i32, h:i32, max_value:i32){
}


fn main() {

    let width: i32 = 200;
    let height: i32 = 100;
    let max_value: i32 = 255;

    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev(){
        for i in 0..width {
            let r: f32 = i as f32 / width as f32;
            let g: f32 = j as f32 / height as f32;
            let b: f32 = 0.25;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir , ig, ib);
        }
    }

    let p1 = Vec3::new(1f32, 2f32, 3f32);
    let p2 = Vec3::new(3.0, 3.0,3.0);
    println!("{:?}", p1 + p2)

}
