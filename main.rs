fn calculate_area(width: u32, height: u32) -> u32 {
    width * height

}

fn calculate_perimeter(width: u32, height: u32) -> u32 {
     2 * (height + width)
}


fn main() {
let rect_width = 10;
let rect_height = 5;
let area = calculate_area(rect_width, rect_height);
let perimeter = calculate_perimeter(rect_width, rect_height);
println!("Rectangle dimensions: {}x{}\nArea: {}\nPerimeter: {}", rect_width, rect_height, area, perimeter);
}