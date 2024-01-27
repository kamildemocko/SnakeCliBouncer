use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::thread::sleep;
use colored::Colorize;
use rand::Rng;
use crate::configer::Config;

pub struct Land<'a>
{
    config: &'a Config,
    vector: Vec<Vec<&'a str>>,
    columns: u8,
    rows: u8,
    bullet_position: (usize, usize),
    bullet_direction: (u8, u8),
    snake:LinkedList<(usize, usize)>,
    tail_len: u8,
}

impl Land<'_> {
    pub fn new(config: &Config, columns: u8, rows: u8) -> Land
    {
        Land {
            config,
            vector: vec![vec![&config.empty; rows as usize]; columns as usize],
            columns,
            rows,
            bullet_position: (0, 0),
            bullet_direction: (1, 1),
            snake: LinkedList::new(),
            tail_len: 3,
        }
    }

    fn update_positional_variables(&mut self)
    {
        if self.bullet_position.0 >= self.columns as usize - 1 { self.bullet_direction.0 = 0 }
        if self.bullet_position.0 <= 0 { self.bullet_direction.0 = 1 }

        match self.bullet_direction.0 {
            1 => self.bullet_position.0 += 1,
            _ => self.bullet_position.0 -= 1,
        }

        if self.bullet_position.1 >= self.rows as usize - 1 { self.bullet_direction.1 = 0 }
        if self.bullet_position.1 <= 0 { self.bullet_direction.1 = 1 }

        match self.bullet_direction.1 {
            1 => self.bullet_position.1 += 1,
            _ => self.bullet_position.1 -= 1,
        }
    }

    fn remove_last_tail_block(&mut self)
    {
        let block = self.snake.pop_back().unwrap();

        let is_tail = self.snake
            .iter()
            .any(|&(x, y)| (x, y) == (block.0, block.1));

        if is_tail {
            self.vector[block.0][block.1] = &self.config.bullet;
        } else {
            self.vector[block.0][block.1] = &self.config.empty;
        }
    }

    pub fn add_food(&mut self)
    {
        let max_per_row = match self.columns {
            0..=9 => 2,
            10..=15 => 3,
            _ => 4,
        };
        let mut rand_thread = rand::thread_rng();

        for col in &mut self.vector {
            let row_rand = rand_thread.gen_range(0..=max_per_row);
            if row_rand == 0 { continue; }

            for _ in 0..row_rand {
                let pos_rand = rand_thread.gen_range(0..self.rows);
                col[pos_rand as usize] = &self.config.food;
            }
        }
    }

    fn test(&self, col: usize, row: usize) -> &str {
        self.vector[col][row]
    }

    pub fn run(&mut self)
    {
        loop {
            print!("\r");
            print!("{}", self);

            let next_value = self.test(self.bullet_position.0, self.bullet_position.1);

            if next_value == &self.config.empty {
                // acknowledge snake crossed itself
                self.vector[self.bullet_position.0][self.bullet_position.1] = &self.config.bullet;
            } else if next_value == &self.config.food {
                // if eaten, prolong the tail
                self.vector[self.bullet_position.0][self.bullet_position.1] = &self.config.bullet;
                self.tail_len += 1;
            } else {
                self.vector[self.bullet_position.0][self.bullet_position.1] = &self.config.bullet_hit;
            }

            self.snake.push_front(self.bullet_position);
            self.update_positional_variables();

            // keep snake short or not if eaten
            if self.snake.len() as u8 > self.tail_len {
                self.remove_last_tail_block();
            }

            print!("\x1b[{}A", self.rows + 2);
            print!("\x1b[?25h");

            sleep(self.config.update_ms);
        }
    }
}

impl Display for Land<'_>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let top_line = format!(
            "\u{250F}{}\u{2513}\n", "\u{2501}".repeat(self.columns as usize * 2 + 1)
        );
        let bottom_line = format!(
            "\u{2517}{}\u{251B}\n", "\u{2501}".repeat(self.columns as usize * 2 + 1)
        );
        let side_line = "\u{2503}";

        write!(f, "{}", top_line.yellow())?;

        for row in 0..self.rows
        {
            for col in 0..self.columns {
                if col == 0  { write!(f, "{} ", side_line.yellow())? }

                write!(f, "{} ", &self.vector[col as usize][row as usize])?;

                if col == self.columns - 1 { write!(f, "{}", side_line.yellow())? }
            }
            write!(f, "\n")?
        }


        // write!(f, "\u{2517}{}\u{251B}\n", "\u{2501}".repeat(self.columns as usize * 2 + 1))?;
        write!(f, "{}", bottom_line.yellow())?;

        Ok(())
    }
}
