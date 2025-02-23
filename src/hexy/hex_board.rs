use super::hexagon::{GridId, HexId, Hexagon};

fn is_dead(x: u32, y: u32, size: u32, row_count: u32) -> bool {
    if y < size {
        return x > y;
    }

    if y > row_count - size {
        return x >= row_count - y;
    }

    let is_even = size % 2;
    if y % 2 == is_even {
        return x == size - 1;
    } else {
        return false;
    }
}

pub struct HexBoard {
    size: u32, // The amount of hexes on a side.
    hex_board: Vec<Vec<Hexagon>>,
}

impl HexBoard {
    pub fn new(size: u32) -> HexBoard {
        let row_count = (size * 2 - 1) * 2 - 1;
        let mut hex_board = Vec::new();
        for y in 0..row_count {
            let mut hex_row = Vec::new();

            for x in 0..size {
                if !is_dead(x, y, size, row_count) {
                    let grid_id = GridId::new(x, y);
                    let hex_id: HexId = HexId::from_grid_id(x, y, size);
                    hex_row.push(Hexagon {
                        grid_id,
                        hex_id,
                        ..Default::default()
                    });
                }
            }

            hex_board.push(hex_row);
        }

        HexBoard { size, hex_board }
    }

    pub fn to_html(&self) -> String {
        let mut board_html = String::from("<div class=\"hex_grid\">");
        for row in self.hex_board.iter() {
            let mut row_html = String::from("<div class=\"hex_row\">");
            for hexagon in row.iter() {
                row_html.push_str(hexagon.to_html().as_str());
            }

            row_html.push_str("</div>");
            board_html.push_str(&row_html);
        }

        board_html.push_str("</div>");
        board_html
    }
}
