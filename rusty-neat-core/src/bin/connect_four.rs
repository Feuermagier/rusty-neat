use std::{
    borrow::BorrowMut,
    fmt::{write, Display},
    path::Path,
};

use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use rusty_neat_core::{gene_pool::GenePool, organism::Organism, population::Population};

const FIELD_WIDTH: usize = 7;
const FIELD_HEIGHT: usize = 6;

const GROUP_COUNT: usize = 4;

pub fn main() {
    neat();
}

fn neat() {
    let pool = GenePool::new_dense(FIELD_WIDTH * FIELD_HEIGHT, FIELD_WIDTH);
    let mut population = Population::new(pool, Path::new("connect_four.json")).unwrap();

    let best_organism = population.evolve(
        |organisms| {
            for organism in organisms.iter_mut() {
                organism.fitness = Some(0.0);
            }

            let groups = create_chunks(
                organisms,
                GROUP_COUNT,
                organisms.len() / GROUP_COUNT,
                |_| (&mut rand::thread_rng()).gen_range(0..GROUP_COUNT),
            );

            groups.par_iter().for_each(|group| {
                println!("Evaluating group of size {}", group.len());
                for i in 0..group.len() {
                    for j in i + 1..group.len() {
                        let result = play(group.get(i), group.get(j));
                        match result {
                            GameResult::FIRST_PLAYER_WON => {
                                group.get(i).fitness = Some(group.get(i).fitness.unwrap() + 1.0);
                            }
                            GameResult::SECOND_PLAYER_WON => {
                                group.get(j).fitness = Some(group.get(j).fitness.unwrap() + 1.0);
                            }
                            GameResult::TIE => {
                                group.get(i).fitness = Some(group.get(i).fitness.unwrap() + 0.5);
                                group.get(j).fitness = Some(group.get(j).fitness.unwrap() + 0.5);
                            }
                        }
                    }
                    group.get(i).fitness =
                        Some(group.get(i).fitness.unwrap() / ((group.len() - 1) as f64));
                }
            });
        },
        Path::new("connect_four_out"),
    );

    println!("==========================================");
    println!("{:?}", best_organism);
}

fn interactive() {
    let mut board = Board::new();

    let mut current_turn = 0;
    while current_turn < FIELD_WIDTH * FIELD_HEIGHT {
        println!("\nPlayer {}:", board.next_player);
        println!("{}\n", board);
        let mut input = "".to_string();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.replace("\r\n", "");

        let column = input.parse().unwrap();
        if board.place_in_column(column) {
            if board.check_win(column) {
                break;
            }

            board.next_player = -1.0 * board.next_player;
            current_turn += 1;
        }
    }
    if current_turn != FIELD_WIDTH * FIELD_HEIGHT {
        println!("Player {} won!", board.next_player);
    } else {
        println!("Tie");
    }
}

fn play(first: &mut Organism, second: &mut Organism) -> GameResult {
    let mut board = Board::new();

    let mut current_turn = 0;
    while current_turn < FIELD_WIDTH * FIELD_HEIGHT {
        let mut max_prob = 0.0;
        let mut max_index = 0;
        if board.next_player == 1.0 {
            for (i, prob) in first.evaluate(&board.board).iter().enumerate() {
                if *prob > max_prob {
                    max_prob = *prob;
                    max_index = i;
                }
            }
        } else {
            for (i, prob) in second.evaluate(&board.board).iter().enumerate() {
                if *prob > max_prob {
                    max_prob = *prob;
                    max_index = i;
                }
            }
        }

        let column = max_index;
        if board.place_in_column(column) {
            if board.check_win(column) {
                break;
            }

            board.next_player = -1.0 * board.next_player;
            current_turn += 1;
        } else {
            if board.next_player == 1.0 {
                return GameResult::SECOND_PLAYER_WON;
            } else {
                return GameResult::FIRST_PLAYER_WON;
            }
        }
    }
    if current_turn != FIELD_WIDTH * FIELD_HEIGHT {
        if board.next_player == 1.0 {
            GameResult::FIRST_PLAYER_WON
        } else {
            GameResult::SECOND_PLAYER_WON
        }
    } else {
        GameResult::TIE
    }
}

enum GameResult {
    FIRST_PLAYER_WON,
    SECOND_PLAYER_WON,
    TIE,
}

struct Board {
    pub board: [f64; (FIELD_HEIGHT + 2) * (FIELD_WIDTH + 2)],
    pub heights: [usize; FIELD_WIDTH],
    pub next_player: f64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [0.0; (FIELD_HEIGHT + 2) * (FIELD_WIDTH + 2)],
            heights: [1; FIELD_WIDTH],
            next_player: 1.0,
        }
    }

    pub fn place_in_column(&mut self, column: usize) -> bool {
        if self.heights[column] != FIELD_HEIGHT {
            self.board[Self::index(column + 1, self.heights[column])] = self.next_player;
            self.heights[column] += 1;
            true
        } else {
            false
        }
    }

    pub fn check_win(&self, last_column: usize) -> bool {
        let height = self.heights[last_column] - 1;

        let mut depth = 1 as usize;
        let mut adjacent = 0;

        // Horizontal nach rechts
        while self.board[Self::index(last_column + 1 + depth, height)] == self.next_player {
            depth += 1;
            adjacent += 1;
        }
        depth = 1;
        if adjacent >= 3 {
            return true;
        }

        // Horizontal nach links
        while self.board[Self::index(last_column + 1 - depth, height)] == self.next_player {
            depth += 1;
            adjacent += 1;
        }
        depth = 1;
        if adjacent >= 3 {
            return true;
        }

        adjacent = 0;

        // Nach rechts unten
        while self.board[Self::index(last_column + 1 + depth, height - depth)] == self.next_player {
            depth += 1;
            adjacent += 1;
        }
        depth = 1;
        if adjacent >= 3 {
            return true;
        }

        // Nach links oben
        while self.board[Self::index(last_column + 1 - depth, height + depth)] == self.next_player {
            depth += 1;
            adjacent += 1;
        }
        depth = 1;
        if adjacent >= 3 {
            return true;
        }

        adjacent = 0;

        // Nach rechts oben
        while self.board[Self::index(last_column + 1 + depth, height + depth)] == self.next_player {
            depth += 1;
            adjacent += 1;
        }
        depth = 1;
        if adjacent >= 3 {
            return true;
        }

        // Nach links unten
        while self.board[Self::index(last_column + 1 - depth, height - depth)] == self.next_player {
            depth += 1;
            adjacent += 1;
        }
        if adjacent >= 3 {
            return true;
        }

        // Senkrecht nach unten
        if height >= 4 {
            if self.board[Self::index(last_column + 1, height - 1)] == self.next_player {
                if self.board[Self::index(last_column + 1, height - 2)] == self.next_player {
                    if self.board[Self::index(last_column + 1, height - 3)] == self.next_player {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn index(column: usize, row: usize) -> usize {
        row * (FIELD_WIDTH + 2) + column
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in (0..FIELD_HEIGHT).rev() {
            for i in 0..FIELD_WIDTH {
                let player = self.board[Self::index(i + 1, j + 1)];
                if player == -1.0 {
                    write!(f, "{} ", 2)?;
                } else {
                    write!(f, "{} ", player)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct IndexedChunk<'a, T> {
    values: &'a [T],
    indices: Vec<usize>,
}

impl<'a, T> IndexedChunk<'a, T> {
    pub fn get(&self, index: usize) -> &mut T {
        let value = &self.values[self.indices[index]];
        unsafe { (value as *const T as *mut T).as_mut().unwrap() }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }
}

fn create_chunks<'a, T, F: Fn(&T) -> usize>(
    values: &'a mut [T],
    chunk_count: usize,
    expected_chunk_size: usize,
    assignment: F,
) -> Vec<IndexedChunk<'a, T>> {
    let values = &(*values);
    let mut indices: Vec<Vec<usize>> = std::iter::repeat(Vec::with_capacity(expected_chunk_size))
        .take(chunk_count)
        .collect();

    for (i, value) in values.iter().enumerate() {
        indices[assignment(value)].push(i);
    }

    let mut chunks = Vec::with_capacity(chunk_count);
    for index in indices {
        chunks.push(IndexedChunk {
            values,
            indices: index,
        });
    }

    chunks
}
