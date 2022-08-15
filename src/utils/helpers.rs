pub fn covert_to_2d_catesian_coord(x: f32, y: f32, width: f32, height: f32) -> Vec<f32> {
    let x_max: f32 = 1.0;
    let x_min: f32 = -1.0;
    let y_max: f32 = 1.0;
    let y_min: f32 = -1.0; 
    
    let new_x: f32 = x_min + ((x * (x_max - x_min)) / width);
    let new_y: f32 = y_max + ((y * (y_max - y_min)) / -height);

    vec![new_x, new_y, 0.0]
}