use std::fs;
use crate::model::board::Board;
use crate::model::column::Column;
use crate::model::column_id::ColumnId;
use crate::assets::colour::Colour;

use serde::Deserialize;

pub struct BoardLoader;

#[derive(Deserialize)]
struct BoardConfig {
    columns: Vec<ColumnConfig>
}

#[derive(Deserialize)]
struct ColumnConfig {
    id: String,
    name: String,
    colour: Colour,
}

impl BoardLoader {
    pub fn load() -> Board {
        if Self::custom_board_exists(&BoardLoader) {
            Self::load_custom_board()
        } else {
            Self::load_default_board()
        }
    }

    fn load_default_board() -> Board {
        let json = fs::read_to_string(
            "./src/assets/default_board.json"
        )
            .expect("Failed to read default board");
        let config: BoardConfig = serde_json::from_str(
            &json
        )
            .expect("Failed to parse default board");

        let mut board = Board::empty();

        for column in config.columns {
            board.add_column(
                Column::new(
                    ColumnId::from_static(
                        &column.id
                    ),
                    column.name,
                    column.colour
                )
            );
        }
        board
    }

    fn load_custom_board() -> Board {
        todo!()
    }
    fn custom_board_exists(&self) -> bool {
        false
    }
}