mod shapebufferstruct;

use {
    shapebufferstruct::*,
    core::f32,
    std::fs::{self},
    egui_speedy2d::{WindowHandler, WindowWrapper},
    speedy2d::{color::Color, window::WindowHelper, Graphics2D, Window, dimen::Vector2, shape::Rectangle},
};

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

                ui.menu_button("Shpae", |ui| {
                    if ui.button("Rectangle").clicked() {
                        self.shape_type = ShapeKind::Rectangle;
                    }
                    if ui.button("Circle").clicked() {
                        self.shape_type = ShapeKind::Circle;
                    }
                });
            });
         });

        // Drawing logic
        for shape_data in &self.shape_buffer {

            match shape_data.shape_type {
                ShapeKind::Circle => 
                {
                    // Radius calculation from the two clicks
                    let mut tmp_coordinate_holder:Vector2<f32> = Vector2::new(0.0, 0.0);
    
                    if shape_data.mouse_click_positions.0.x > shape_data.mouse_click_positions.1.x {
                        tmp_coordinate_holder.x = shape_data.mouse_click_positions.0.x - shape_data.mouse_click_positions.1.x;
                    }
                    else {
                        tmp_coordinate_holder.x = shape_data.mouse_click_positions.1.x - shape_data.mouse_click_positions.0.x;
                    }
    
                    if shape_data.mouse_click_positions.0.y > shape_data.mouse_click_positions.1.y {
                        tmp_coordinate_holder.y = shape_data.mouse_click_positions.0.y - shape_data.mouse_click_positions.1.y;
                    }
                    else {
                        tmp_coordinate_holder.y = shape_data.mouse_click_positions.1.y - shape_data.mouse_click_positions.0.y;
                    }
    
                    let circle_radius = if tmp_coordinate_holder.x > tmp_coordinate_holder.y {tmp_coordinate_holder.x} else {tmp_coordinate_holder.y};
    
                    graphics.draw_circle(shape_data.mouse_click_positions.0, circle_radius, Color::RED);
                },
                ShapeKind::Rectangle => 
                {
                    let rectangle = Rectangle::new(shape_data.mouse_click_positions.0, shape_data.mouse_click_positions.1);
                    graphics.draw_rectangle(rectangle, Color::BLACK);
                },
                ShapeKind::None => 
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


    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, _button: speedy2d::window::MouseButton, _egui_ctx: &egui::Context,)
    {
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
    
}