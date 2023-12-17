use std::collections::VecDeque;


use speedy2d::window::MouseButton;
mod shapebufferstruct;


use {
    shapebufferstruct::*,
    core::f32,
    std::fs::{self},
    egui_speedy2d::{WindowHandler, WindowWrapper},
    speedy2d::{color::Color, window::WindowHelper, Graphics2D, Window, dimen::Vector2, shape::Rectangle},
};

const TASK_REQUIRED_PIXEL_MINIMUM: f32 = 3.0; 
const LINE_THICKNESS: f32 = 5.0;


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // simple_logger::SimpleLogger::new().init().unwrap();
    let window = Window::new_centered("Shape drawing", (640, 540)).unwrap();
    window.run_loop(WindowWrapper::new(MyWindowHandler {
        shape_type: ShapeKind::None,
        mouse_global_position: Vector2::new(0.0, 0.0),
        mouse_click_counter: 1,
        mouse_first_position: Vector2::new(0.0, 0.0),
        shape_buffer:Vec::new(),
    }))
}


struct MyWindowHandler
{
    shape_type: ShapeKind,
    mouse_global_position: Vector2<f32>,
    mouse_click_counter: i32,

    mouse_first_position: Vector2<f32>,
    shape_buffer: Vec<ShapeBufferStruct>
    
}

impl WindowHandler for MyWindowHandler 
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D, egui_ctx: &egui_speedy2d::egui::Context,)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));        
        
        // Panel control
        egui::TopBottomPanel::top("my_panel").show(egui_ctx, |ui|
        {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|
            {
                ui.set_min_height(40.0);

                ui.menu_button("File", |ui|
                {
                    if ui.button("New").clicked() {
                        helper.request_redraw();
                        self.shape_buffer.clear();  
                    }

                    if ui.button("Open").clicked() {
                        let path = std::env::current_dir().unwrap();
                        let res = rfd::FileDialog::new()
                            .add_filter("text", &["txt", "rs"])
                            .add_filter("rust", &["rs", "toml"])
                            .set_directory(&path)
                            .pick_files();
                        
                        // Checking for the path, if multiple paths are provided, get the one on top of the stack
                        let path_string = res.map(|mut vec_buffer| vec_buffer.pop().expect(" ").to_string_lossy().to_string())
                                                    .unwrap_or(String::from("default_path_string"));

                        if path_string != "default_path_string" {
                        
                            let tmp_buffer = ShapeBufferStruct::create_from_file(path_string);
                                
                            self.shape_buffer.clear();
                            self.shape_buffer = tmp_buffer;
                        }
                        
                    }
                    if ui.button("Save").clicked() {
                        let path = std::env::current_dir().unwrap();
                        let res = rfd::FileDialog::new()
                            .set_file_name("foo.txt")
                            .add_filter("text", &["txt", "rs"])
                            .set_directory(&path)
                            .save_file();

                        // Ensure the path will always be valid
                        let path_string: String = res
                            .map(|pathbuf| pathbuf.to_string_lossy().to_string())
                            .unwrap_or(String::from("default_path_string"));

                        println!("Path as String: {}", path_string);

                        if path_string != "default_path_string" {
                            let mut string_for_file = String::new();
                            for shape_data in &self.shape_buffer {
                                string_for_file.push_str(&shape_data.to_string());
                            }
                            fs::write(path_string, string_for_file).expect("Unable to write");
                            // fs::write(path_string, "Loli").expect("Unable to write");
                        }
                    }
                });

                ui.menu_button("Shape", |ui| {
                    if ui.button("Rectangle").clicked() {
                        self.shape_type = ShapeKind::Rectangle;
                    }
                    if ui.button("Circle").clicked() {
                        self.shape_type = ShapeKind::Circle;
                    }
                    if ui.button("Line").clicked() {
                        self.shape_type = ShapeKind::Line;
                    }
                });
            });
         });

        // Drawing logic
        for shape_data in &self.shape_buffer {

            match shape_data.shape_type {
                ShapeKind::Circle => 
                {   
                    let radius = calculate_radius(&shape_data.mouse_click_positions.0, &shape_data.mouse_click_positions.1);
                    // graphics.draw_circle(shape_data.mouse_click_positions.0, radius, Color::RED);
                    graphics.draw_circle(shape_data.mouse_click_positions.0, radius, Color::RED);
                },
                ShapeKind::Rectangle => 
                {
                    let rectangle = Rectangle::new(shape_data.mouse_click_positions.0, shape_data.mouse_click_positions.1);
                    
                    graphics.draw_rectangle(rectangle, Color::BLACK);
                },
                ShapeKind::Line =>
                {

                    graphics.draw_line(shape_data.mouse_click_positions.0, shape_data.mouse_click_positions.1, LINE_THICKNESS, Color::BLACK);
                }
                _ => 
                {
                    println!("Inpossible case");
                },
            }
        }

        helper.request_redraw();
    }


    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2, _egui_ctx: &egui::Context,) 
    {
        self.mouse_global_position = position;
    }


    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: speedy2d::window::MouseButton, _egui_ctx: &egui::Context,)
    {
        
        if button == MouseButton::Left {
            match self.mouse_click_counter {
                1 => {
                    self.mouse_first_position = self.mouse_global_position;
                    self.mouse_click_counter += 1;
                },
    
                2 => {
                    if !matches!(self.shape_type, ShapeKind::None) 
                    {
                        let tmp_var = ShapeBufferStruct {shape_type: self.shape_type,
                                                         mouse_click_positions: (self.mouse_first_position, self.mouse_global_position)};
                        self.shape_buffer.push(tmp_var);
                    }
    
                    self.mouse_click_counter -= 1;   
                } 
    
                _ => print!("Wrong value")
            }
        }
        else {
            // Circle to be removed
            let mut tmp_shape:ShapeBufferStruct = ShapeBufferStruct { shape_type: ShapeKind::None, mouse_click_positions: (Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)) }; 
            
            // Inverted vector for shape removing
            let mut shape_deque: VecDeque<ShapeBufferStruct> = VecDeque::new();
            for shape in &self.shape_buffer {
                shape_deque.push_front(shape.clone());
            }		

            // Loop for checking if the mouse coordinates are on a rectangle or circle
            for shape in &self.shape_buffer {
                match shape.shape_type {
                    ShapeKind::Circle => 
                    {
                        let dx = self.mouse_global_position.x - shape.mouse_click_positions.0.x;
                        let dy = self.mouse_global_position.y - shape.mouse_click_positions.0.y;
                        let distance = (dx * dx + dy * dy).sqrt();
                        let radius = calculate_radius(&shape.mouse_click_positions.0, &shape.mouse_click_positions.1);
                        tmp_shape = if distance <= radius + TASK_REQUIRED_PIXEL_MINIMUM { shape.clone()} else { tmp_shape}  ;
                        break;
                    },
                    ShapeKind::Rectangle => 
                    {
                        let rectangle = Rectangle::new(shape.mouse_click_positions.0, shape.mouse_click_positions.1);
                        if  is_point_near_rectangle(&rectangle, &self.mouse_global_position) {
                            tmp_shape = shape.clone(); 
                            break;
                        }
                    },
                    ShapeKind::Line =>
                    {
                        if is_point_on_line(&shape.mouse_click_positions.0, &shape.mouse_click_positions.1, &self.mouse_global_position, LINE_THICKNESS) {
                            tmp_shape = shape.clone();
                            break;
                        }
                    },
                    _ => todo!(),
                }
            }		
            self.shape_buffer.retain(|&shape| shape.mouse_click_positions != tmp_shape.mouse_click_positions )
        }
    }
    
}


fn calculate_radius (first_vector: &Vector2<f32>, second_vector : &Vector2<f32>) -> f32 {

        let mut tmp_coordinate_holder:Vector2<f32> = Vector2::new(0.0, 0.0);

        if first_vector.x > second_vector.x {
            tmp_coordinate_holder.x = first_vector.x - second_vector.x;
        }
        else {
            tmp_coordinate_holder.x = second_vector.x - first_vector.x;
        }

        if first_vector.y > second_vector.y {
            tmp_coordinate_holder.y = first_vector.y - second_vector.y;
        }
        else {
            tmp_coordinate_holder.y = second_vector.y - first_vector.y;
        }

        let radius = if tmp_coordinate_holder.x > tmp_coordinate_holder.y {tmp_coordinate_holder.x} else {tmp_coordinate_holder.y};

        return radius;
}


fn is_point_near_rectangle(rect: &Rectangle, point: &Vector2<f32>) -> bool {
    let within_x = point.x + TASK_REQUIRED_PIXEL_MINIMUM >= rect.top_left().x
        && point.x - TASK_REQUIRED_PIXEL_MINIMUM <= rect.bottom_right().x;

    let within_y = point.y + TASK_REQUIRED_PIXEL_MINIMUM  >= rect.top_left().y
        && point.y - TASK_REQUIRED_PIXEL_MINIMUM  <= rect.bottom_right().y;

    within_x && within_y

}

fn is_point_on_line(starting_point: &Vector2<f32>, end_point: &Vector2<f32>, point: &Vector2<f32>, thickness: f32) -> bool {

    // Calculate the equation of the line Ax + By + C = 0
    let a = end_point.y - starting_point.y;
    let b = starting_point.x - end_point.x;
    let c = (end_point.x - starting_point.x) * starting_point.y + (starting_point.y - end_point.y) * starting_point.x;

    // Calculate distance from the point to the line
    let distance = (a * point.x + b * point.y + c).abs() / f32::sqrt(a.powi(2) + b.powi(2));

    // Check if the point is within the proximity threshold or if the distance is within the width
    distance <= LINE_THICKNESS / 2.0 || distance <= thickness
}
