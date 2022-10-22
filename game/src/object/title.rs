use termtools::Position;

use crate::{
    object::{Text, Title},
    Game,
};

const PONG: &str = "######  ####### #     #  ##### 
#     # #     # ##    # #     #
#     # #     # # #   # #      
######  #     # #  #  # #  ####
#       #     # #   # # #     #
#       #     # #    ## #     #
#       ####### #     #  ##### 
How to play:
W: Up, S: Down";

impl Title {
    pub fn new(game: &mut Game, leftup: Position) -> Self {
        let text = Text::new(game, leftup, PONG);
        Self { inner: text }
    }
}
