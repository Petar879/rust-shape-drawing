
use {
    std::{fs::File, io::{BufRead, BufReader}, collections::VecDeque},
    speedy2d::dimen::Vector2,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeKind
{
    Circle,
    Rectangle,
    Line,
    None
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeBufferStruct
{
    pub shape_type: ShapeKind,
    pub mouse_click_positions: (Vector2<f32>, Vector2<f32>)
}

impl ShapeBufferStruct {

    pub fn new(shape_kind: String, mouse_position_1_x: String, mouse_position_1_y: String, mouse_position_2_x: String, mouse_position_2_y: String) -> Self {
        let mut tmp_shape_kind = ShapeKind::None;

        match shape_kind.as_str() {
            "C" => tmp_shape_kind = ShapeKind::Circle,
            "R" => tmp_shape_kind = ShapeKind::Rectangle,
            "L" => tmp_shape_kind = ShapeKind::Line,
            _ => println!("idk"),
        };

        ShapeBufferStruct{
            shape_type: tmp_shape_kind,
            mouse_click_positions: (Vector2::new(mouse_position_1_x.parse::<f32>().unwrap(),
                                                 mouse_position_1_y.parse::<f32>().unwrap()),
                                    Vector2::new(
                                        mouse_position_2_x.parse::<f32>().unwrap(),
                                        mouse_position_2_y.parse::<f32>().unwrap()),),
        }

    }

    pub fn to_string(&self) -> String {
        let mut shape_type_string: String = String::new();
        match self.shape_type {
            ShapeKind::Circle =>
            {
                shape_type_string = "C".to_owned();
            },
            ShapeKind::Rectangle =>
            {
                shape_type_string = "R".to_owned();
            },
            ShapeKind::Line =>
            {
                shape_type_string = "L".to_owned();
            },
            ShapeKind::None => todo!(),
        }
        return format!("{} {} {} {} {}\n", 
                    shape_type_string,
                    self.mouse_click_positions.0.x.to_string(),
                    self.mouse_click_positions.0.y.to_string(),
                    self.mouse_click_positions.1.x.to_string(),
                    self.mouse_click_positions.1.y.to_string());
    }

    pub fn create_from_file(file_path: String) -> Vec<ShapeBufferStruct> {
        let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));
        let mut final_vec: Vec<ShapeBufferStruct> = Vec::new();

        for line in reader.lines()
        {
            //VecDeque for reaching the front side of the collection
            let mut tmp_whitespace: VecDeque<String> = line.unwrap().split_whitespace().map(str::to_string).collect();
            
            final_vec.push(ShapeBufferStruct::new(tmp_whitespace.pop_front().unwrap(),
                                          tmp_whitespace.pop_front().unwrap(),
                                          tmp_whitespace.pop_front().unwrap(),
                                          tmp_whitespace.pop_front().unwrap(),
                                          tmp_whitespace.pop_front().unwrap()))
        }

        return final_vec;
    }

}