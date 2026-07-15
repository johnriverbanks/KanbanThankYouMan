use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use crate::model::board::Board;
use crate::assets::colour::Colour;
use crate::model::column::Column;
use crate::model::column_id::ColumnId;

pub struct BoardLoader;

#[derive(Deserialize)]
struct BoardConfig {
    default_column: String,
    columns: Vec<ColumnConfig>,
    column_order: Vec<String>,
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

        let mut columns = HashMap::new();

        for column in config.columns {
            let id = ColumnId::parse_column_id(&column.id);
            columns.insert(
                id.clone(),
                Column::new(
                    id,
                    column.name,
                    column.colour,
                ),
            );
        }

        let column_order = config
            .column_order
            .into_iter()
            .map(|id| ColumnId::parse_column_id(&id))
            .collect();

        let default_column = ColumnId::parse_column_id(
            &config.default_column,
        );

        Board::from_config(
            columns,
            column_order,
            default_column,
        )
            .expect("Invalid board configuration")
    }

    fn load_custom_board() -> Board {
        todo!()
            }
            fn custom_board_exists(&self) -> bool {
                std::path::Path::new(
                    "./board.json"
                ).exists()
            }
        }