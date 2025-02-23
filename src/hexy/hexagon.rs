use std::fmt::Display;

use tera::Context;

use crate::util::get_template;

const TEAM_COLORS: [&str; 6] = ["#f00", "#0f0", "#00f", "#0ff", "#f0f", "#f00"];

#[derive(Debug, Default)]
pub enum HexState {
    #[default]
    Free,
    Piece {
        team: u32,
        value: u32,
    },
}

#[derive(Debug, Default)]
struct HexagonTemplate {
    piece_style: String,
    value_style: String,
    hex_value: String,
    data_grid_id: String,
    data_hex_id: String,
}

impl Into<Context> for HexagonTemplate {
    fn into(self) -> Context {
        let mut context = Context::new();

        context.insert("piece_style", self.piece_style.as_str());
        context.insert("value_style", self.value_style.as_str());
        context.insert("hex_value", self.hex_value.as_str());
        context.insert("data_grid_id", self.data_grid_id.as_str());
        context.insert("data_hex_id", self.data_hex_id.as_str());

        context
    }
}

/// The coordinates of the hex on the board itself, like on a chess board.
/// On the screen the most top HexId is (0, 0) and the most bottom hex is (n, n) where n = (size * 2) - 1.
/// Going to the left x increases, going to the right y increases.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HexId {
    pub x: u32,
    pub y: u32,
}

impl Display for HexId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// The indices of the hex within the Vec<Vec<Hexagon>> of the HexBoard.
/// On the screen the most top HexId is (0, 0) and the most bottom hex is (0, n) where n = (size * 2) + ((size - 1) * 2) - 1.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GridId {
    x: u32,
    y: u32,
}

impl Display for GridId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl GridId {
    pub fn new(x: u32, y: u32) -> GridId {
        GridId { x, y }
    }

    pub fn to_hex_id(&self, size: u32) -> HexId {
        HexId::from_grid_id(self.x, self.y, size)
    }
}

#[derive(Debug, Default)]
pub struct Hexagon {
    pub state: HexState,
    pub hex_id: HexId,
    pub grid_id: GridId,
}

impl Hexagon {
    pub fn to_html(&self) -> String {
        let mut template = HexagonTemplate {
            piece_style: "display: none;".into(),
            value_style: "display: none;".into(),
            hex_value: "".into(),
            data_grid_id: self.grid_id.to_string(),
            data_hex_id: self.hex_id.to_string(),
        };

        if let HexState::Piece { team, value } = self.state {
            let i_team_color = team % TEAM_COLORS.len() as u32;
            let team_color = TEAM_COLORS[i_team_color as usize];
            template.piece_style = format!("color: {team_color}");

            template.value_style = "".into();

            template.hex_value = if value == 0 {
                "&#x1F542;".to_string()
            } else {
                value.to_string()
            }
        }

        get_template("templates/hexy/hexagon.html", Some(template.into()))
            .expect("Failed to load hexagon template.")
    }
}

impl HexId {
    pub fn new(x: u32, y: u32) -> HexId {
        HexId { x, y }
    }

    pub fn from_grid_id(grid_x: u32, grid_y: u32, size: u32) -> HexId {
        let first_corner_i = size - 1;
        let second_corner_i = first_corner_i * 3 + 2; 

        let mut x = 1;
        let mut y = 1;

        // println!("grid_x={grid_x}, grid_y={grid_y}, size={size}, first_corner_i={first_corner_i}, second_corner_i={second_corner_i}");
        for i in 1..=grid_y {
            // println!("x={x}, y={y}, i={i}");
            if i <= first_corner_i {
                x += 1;
            } else if i <= second_corner_i {
                x += (i - first_corner_i + 1) % 2;
                y += (i - first_corner_i) % 2;
            } else {
                y += 1;
            }
        }
        // println!("x={x}, y={y}");

        x -= grid_x;
        y += grid_x;

        HexId::new(x, y)
    }
}

#[cfg(test)]
mod test {
    use crate::hexy::hexagon::GridId;
    use crate::hexy::hexagon::HexId;

    #[test] fn test_x6_y7_size4_test_both() { assert_eq!(HexId::new(6, 7), GridId::new(2, 11).to_hex_id(4)); }


    #[test] fn test_x1_y1_size4_test_both() { assert_eq!(HexId::new(1, 1), GridId::new(0, 0).to_hex_id(4)); }
    #[test] fn test_x2_y1_size4_test_both() { assert_eq!(HexId::new(2, 1), GridId::new(0, 1).to_hex_id(4)); }
    #[test] fn test_x2_y2_size4_test_both() { assert_eq!(HexId::new(2, 2), GridId::new(1, 2).to_hex_id(4)); }
    #[test] fn test_x1_y2_size4_test_both() { assert_eq!(HexId::new(1, 2), GridId::new(1, 1).to_hex_id(4)); }
    #[test] fn test_x3_y1_size4_test_both() { assert_eq!(HexId::new(3, 1), GridId::new(0, 2).to_hex_id(4)); }
    #[test] fn test_x3_y2_size4_test_both() { assert_eq!(HexId::new(3, 2), GridId::new(1, 3).to_hex_id(4)); }
    #[test] fn test_x3_y3_size4_test_both() { assert_eq!(HexId::new(3, 3), GridId::new(1, 4).to_hex_id(4)); }
    #[test] fn test_x2_y3_size4_test_both() { assert_eq!(HexId::new(2, 3), GridId::new(2, 3).to_hex_id(4)); }
    #[test] fn test_x1_y3_size4_test_both() { assert_eq!(HexId::new(1, 3), GridId::new(2, 2).to_hex_id(4)); }
    #[test] fn test_x4_y1_size4_test_both() { assert_eq!(HexId::new(4, 1), GridId::new(0, 3).to_hex_id(4)); }
    #[test] fn test_x4_y2_size4_test_both() { assert_eq!(HexId::new(4, 2), GridId::new(0, 4).to_hex_id(4)); }
    #[test] fn test_x4_y3_size4_test_both() { assert_eq!(HexId::new(4, 3), GridId::new(1, 5).to_hex_id(4)); }
    #[test] fn test_x4_y4_size4_test_both() { assert_eq!(HexId::new(4, 4), GridId::new(1, 6).to_hex_id(4)); }
    #[test] fn test_x3_y4_size4_test_both() { assert_eq!(HexId::new(3, 4), GridId::new(2, 5).to_hex_id(4)); }
    #[test] fn test_x2_y4_size4_test_both() { assert_eq!(HexId::new(2, 4), GridId::new(2, 4).to_hex_id(4)); }
    #[test] fn test_x1_y4_size4_test_both() { assert_eq!(HexId::new(1, 4), GridId::new(3, 3).to_hex_id(4)); }
    #[test] fn test_x5_y2_size4_test_both() { assert_eq!(HexId::new(5, 2), GridId::new(0, 5).to_hex_id(4)); }
    #[test] fn test_x5_y3_size4_test_both() { assert_eq!(HexId::new(5, 3), GridId::new(0, 6).to_hex_id(4)); }
    #[test] fn test_x5_y4_size4_test_both() { assert_eq!(HexId::new(5, 4), GridId::new(1, 7).to_hex_id(4)); }
    #[test] fn test_x5_y5_size4_test_both() { assert_eq!(HexId::new(5, 5), GridId::new(1, 8).to_hex_id(4)); }
    #[test] fn test_x4_y5_size4_test_both() { assert_eq!(HexId::new(4, 5), GridId::new(2, 7).to_hex_id(4)); }
    #[test] fn test_x3_y5_size4_test_both() { assert_eq!(HexId::new(3, 5), GridId::new(2, 6).to_hex_id(4)); }
    #[test] fn test_x2_y5_size4_test_both() { assert_eq!(HexId::new(2, 5), GridId::new(3, 5).to_hex_id(4)); }
    #[test] fn test_x6_y3_size4_test_both() { assert_eq!(HexId::new(6, 3), GridId::new(0, 7).to_hex_id(4)); }
    #[test] fn test_x6_y4_size4_test_both() { assert_eq!(HexId::new(6, 4), GridId::new(0, 8).to_hex_id(4)); }
    #[test] fn test_x6_y5_size4_test_both() { assert_eq!(HexId::new(6, 5), GridId::new(1, 9).to_hex_id(4)); }
    #[test] fn test_x6_y6_size4_test_both() { assert_eq!(HexId::new(6, 6), GridId::new(1, 10).to_hex_id(4)); }
    #[test] fn test_x5_y6_size4_test_both() { assert_eq!(HexId::new(5, 6), GridId::new(2, 9).to_hex_id(4)); }
    #[test] fn test_x4_y6_size4_test_both() { assert_eq!(HexId::new(4, 6), GridId::new(2, 8).to_hex_id(4)); }
    #[test] fn test_x3_y6_size4_test_both() { assert_eq!(HexId::new(3, 6), GridId::new(3, 7).to_hex_id(4)); }
    #[test] fn test_x7_y4_size4_test_both() { assert_eq!(HexId::new(7, 4), GridId::new(0, 9).to_hex_id(4)); }
    #[test] fn test_x7_y5_size4_test_both() { assert_eq!(HexId::new(7, 5), GridId::new(0, 10).to_hex_id(4)); }
    #[test] fn test_x7_y6_size4_test_both() { assert_eq!(HexId::new(7, 6), GridId::new(1, 11).to_hex_id(4)); }
    #[test] fn test_x7_y7_size4_test_both() { assert_eq!(HexId::new(7, 7), GridId::new(1, 12).to_hex_id(4)); }
    #[test] fn test_x5_y7_size4_test_both() { assert_eq!(HexId::new(5, 7), GridId::new(2, 10).to_hex_id(4)); }
    #[test] fn test_x4_y7_size4_test_both() { assert_eq!(HexId::new(4, 7), GridId::new(3, 9).to_hex_id(4)); }


    #[test] fn test_x1_y1_size4_test_x() { assert_eq!(1, GridId::new(0, 0).to_hex_id(4).x); }
    #[test] fn test_x2_y1_size4_test_x() { assert_eq!(2, GridId::new(0, 1).to_hex_id(4).x); }
    #[test] fn test_x2_y2_size4_test_x() { assert_eq!(2, GridId::new(1, 2).to_hex_id(4).x); }
    #[test] fn test_x1_y2_size4_test_x() { assert_eq!(1, GridId::new(1, 1).to_hex_id(4).x); }
    #[test] fn test_x3_y1_size4_test_x() { assert_eq!(3, GridId::new(0, 2).to_hex_id(4).x); }
    #[test] fn test_x3_y2_size4_test_x() { assert_eq!(3, GridId::new(1, 3).to_hex_id(4).x); }
    #[test] fn test_x3_y3_size4_test_x() { assert_eq!(3, GridId::new(1, 4).to_hex_id(4).x); }
    #[test] fn test_x2_y3_size4_test_x() { assert_eq!(2, GridId::new(2, 3).to_hex_id(4).x); }
    #[test] fn test_x1_y3_size4_test_x() { assert_eq!(1, GridId::new(2, 2).to_hex_id(4).x); }
    #[test] fn test_x4_y1_size4_test_x() { assert_eq!(4, GridId::new(0, 3).to_hex_id(4).x); }
    #[test] fn test_x4_y2_size4_test_x() { assert_eq!(4, GridId::new(0, 4).to_hex_id(4).x); }
    #[test] fn test_x4_y3_size4_test_x() { assert_eq!(4, GridId::new(1, 5).to_hex_id(4).x); }
    #[test] fn test_x4_y4_size4_test_x() { assert_eq!(4, GridId::new(1, 6).to_hex_id(4).x); }
    #[test] fn test_x3_y4_size4_test_x() { assert_eq!(3, GridId::new(2, 5).to_hex_id(4).x); }
    #[test] fn test_x2_y4_size4_test_x() { assert_eq!(2, GridId::new(2, 4).to_hex_id(4).x); }
    #[test] fn test_x1_y4_size4_test_x() { assert_eq!(1, GridId::new(3, 3).to_hex_id(4).x); }
    #[test] fn test_x5_y2_size4_test_x() { assert_eq!(5, GridId::new(0, 5).to_hex_id(4).x); }
    #[test] fn test_x5_y3_size4_test_x() { assert_eq!(5, GridId::new(0, 6).to_hex_id(4).x); }
    #[test] fn test_x5_y4_size4_test_x() { assert_eq!(5, GridId::new(1, 7).to_hex_id(4).x); }
    #[test] fn test_x5_y5_size4_test_x() { assert_eq!(5, GridId::new(1, 8).to_hex_id(4).x); }
    #[test] fn test_x4_y5_size4_test_x() { assert_eq!(4, GridId::new(2, 7).to_hex_id(4).x); }
    #[test] fn test_x3_y5_size4_test_x() { assert_eq!(3, GridId::new(2, 6).to_hex_id(4).x); }
    #[test] fn test_x2_y5_size4_test_x() { assert_eq!(2, GridId::new(3, 5).to_hex_id(4).x); }
    #[test] fn test_x6_y3_size4_test_x() { assert_eq!(6, GridId::new(0, 7).to_hex_id(4).x); }
    #[test] fn test_x6_y4_size4_test_x() { assert_eq!(6, GridId::new(0, 8).to_hex_id(4).x); }
    #[test] fn test_x6_y5_size4_test_x() { assert_eq!(6, GridId::new(1, 9).to_hex_id(4).x); }
    #[test] fn test_x6_y6_size4_test_x() { assert_eq!(6, GridId::new(1, 10).to_hex_id(4).x); }
    #[test] fn test_x5_y6_size4_test_x() { assert_eq!(5, GridId::new(2, 9).to_hex_id(4).x); }
    #[test] fn test_x4_y6_size4_test_x() { assert_eq!(4, GridId::new(2, 8).to_hex_id(4).x); }
    #[test] fn test_x3_y6_size4_test_x() { assert_eq!(3, GridId::new(3, 7).to_hex_id(4).x); }
    #[test] fn test_x7_y4_size4_test_x() { assert_eq!(7, GridId::new(0, 9).to_hex_id(4).x); }
    #[test] fn test_x7_y5_size4_test_x() { assert_eq!(7, GridId::new(0, 10).to_hex_id(4).x); }
    #[test] fn test_x7_y6_size4_test_x() { assert_eq!(7, GridId::new(1, 11).to_hex_id(4).x); }
    #[test] fn test_x7_y7_size4_test_x() { assert_eq!(7, GridId::new(1, 12).to_hex_id(4).x); }
    #[test] fn test_x6_y7_size4_test_x() { assert_eq!(6, GridId::new(2, 11).to_hex_id(4).x); }
    #[test] fn test_x5_y7_size4_test_x() { assert_eq!(5, GridId::new(2, 10).to_hex_id(4).x); }
    #[test] fn test_x4_y7_size4_test_x() { assert_eq!(4, GridId::new(3, 9).to_hex_id(4).x); }


    #[test] fn test_x1_y1_size4_test_y() { assert_eq!(1, GridId::new(0, 0).to_hex_id(4).y); }
    #[test] fn test_x2_y1_size4_test_y() { assert_eq!(1, GridId::new(0, 1).to_hex_id(4).y); }
    #[test] fn test_x2_y2_size4_test_y() { assert_eq!(2, GridId::new(1, 2).to_hex_id(4).y); }
    #[test] fn test_x1_y2_size4_test_y() { assert_eq!(2, GridId::new(1, 1).to_hex_id(4).y); }
    #[test] fn test_x3_y1_size4_test_y() { assert_eq!(1, GridId::new(0, 2).to_hex_id(4).y); }
    #[test] fn test_x3_y2_size4_test_y() { assert_eq!(2, GridId::new(1, 3).to_hex_id(4).y); }
    #[test] fn test_x3_y3_size4_test_y() { assert_eq!(3, GridId::new(1, 4).to_hex_id(4).y); }
    #[test] fn test_x2_y3_size4_test_y() { assert_eq!(3, GridId::new(2, 3).to_hex_id(4).y); }
    #[test] fn test_x1_y3_size4_test_y() { assert_eq!(3, GridId::new(2, 2).to_hex_id(4).y); }
    #[test] fn test_x4_y1_size4_test_y() { assert_eq!(1, GridId::new(0, 3).to_hex_id(4).y); }
    #[test] fn test_x4_y2_size4_test_y() { assert_eq!(2, GridId::new(0, 4).to_hex_id(4).y); }
    #[test] fn test_x4_y3_size4_test_y() { assert_eq!(3, GridId::new(1, 5).to_hex_id(4).y); }
    #[test] fn test_x4_y4_size4_test_y() { assert_eq!(4, GridId::new(1, 6).to_hex_id(4).y); }
    #[test] fn test_x3_y4_size4_test_y() { assert_eq!(4, GridId::new(2, 5).to_hex_id(4).y); }
    #[test] fn test_x2_y4_size4_test_y() { assert_eq!(4, GridId::new(2, 4).to_hex_id(4).y); }
    #[test] fn test_x1_y4_size4_test_y() { assert_eq!(4, GridId::new(3, 3).to_hex_id(4).y); }
    #[test] fn test_x5_y2_size4_test_y() { assert_eq!(2, GridId::new(0, 5).to_hex_id(4).y); }
    #[test] fn test_x5_y3_size4_test_y() { assert_eq!(3, GridId::new(0, 6).to_hex_id(4).y); }
    #[test] fn test_x5_y4_size4_test_y() { assert_eq!(4, GridId::new(1, 7).to_hex_id(4).y); }
    #[test] fn test_x5_y5_size4_test_y() { assert_eq!(5, GridId::new(1, 8).to_hex_id(4).y); }
    #[test] fn test_x4_y5_size4_test_y() { assert_eq!(5, GridId::new(2, 7).to_hex_id(4).y); }
    #[test] fn test_x3_y5_size4_test_y() { assert_eq!(5, GridId::new(2, 6).to_hex_id(4).y); }
    #[test] fn test_x2_y5_size4_test_y() { assert_eq!(5, GridId::new(3, 5).to_hex_id(4).y); }
    #[test] fn test_x6_y3_size4_test_y() { assert_eq!(3, GridId::new(0, 7).to_hex_id(4).y); }
    #[test] fn test_x6_y4_size4_test_y() { assert_eq!(4, GridId::new(0, 8).to_hex_id(4).y); }
    #[test] fn test_x6_y5_size4_test_y() { assert_eq!(5, GridId::new(1, 9).to_hex_id(4).y); }
    #[test] fn test_x6_y6_size4_test_y() { assert_eq!(6, GridId::new(1, 10).to_hex_id(4).y); }
    #[test] fn test_x5_y6_size4_test_y() { assert_eq!(6, GridId::new(2, 9).to_hex_id(4).y); }
    #[test] fn test_x4_y6_size4_test_y() { assert_eq!(6, GridId::new(2, 8).to_hex_id(4).y); }
    #[test] fn test_x3_y6_size4_test_y() { assert_eq!(6, GridId::new(3, 7).to_hex_id(4).y); }
    #[test] fn test_x7_y4_size4_test_y() { assert_eq!(4, GridId::new(0, 9).to_hex_id(4).y); }
    #[test] fn test_x7_y5_size4_test_y() { assert_eq!(5, GridId::new(0, 10).to_hex_id(4).y); }
    #[test] fn test_x7_y6_size4_test_y() { assert_eq!(6, GridId::new(1, 11).to_hex_id(4).y); }
    #[test] fn test_x7_y7_size4_test_y() { assert_eq!(7, GridId::new(1, 12).to_hex_id(4).y); }
    #[test] fn test_x6_y7_size4_test_y() { assert_eq!(7, GridId::new(2, 11).to_hex_id(4).y); }
    #[test] fn test_x5_y7_size4_test_y() { assert_eq!(7, GridId::new(2, 10).to_hex_id(4).y); }
    #[test] fn test_x4_y7_size4_test_y() { assert_eq!(7, GridId::new(3, 9).to_hex_id(4).y); }
}
