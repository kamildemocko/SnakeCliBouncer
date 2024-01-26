use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use std::thread::sleep;
use crate::consts::{BULLET, EMPTY, TAIL_LEN, UPDATE_MS};

pub struct Land<'a> {
    vector: Vec<Vec<&'a str>>,
    columns: u8,
    rows: u8,
    bullet_position: (usize, usize),
    bullet_direction: (u8, u8),
    snake:LinkedList<(usize, usize)>,
}

impl Land<'_> {
    pub fn new(columns: u8, rows: u8) -> Land<'static> {
        Land {
            vector: vec![vec![EMPTY; rows as usize]; columns as usize],
            columns,
            rows,
            bullet_position: (0, 0),
            bullet_direction: (1, 1),
            snake: LinkedList::new(),
        }
    }

    fn update_positional_variables(&mut self) {
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

    fn remove_last_tail_block(&mut self) {
        let block = self.snake.pop_back().unwrap();
        self.vector[block.0][block.1] = EMPTY;
    }

    pub fn run(&mut self)
    {
        loop {
            print!("\r");
            print!("{}", self);

            self.vector[self.bullet_position.0][self.bullet_position.1] = BULLET;

            self.snake.push_front(self.bullet_position);
            if self.snake.len() as u8 > TAIL_LEN {
                self.remove_last_tail_block();
            }

            self.update_positional_variables();

            print!("\x1b[{}A", self.rows);
            print!("\x1b[?25h");

            sleep(UPDATE_MS);
        }
    }
}

impl Display for Land<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        for row in 0..self.rows
        {
            for col in 0..self.columns {
                write!(f, "{} ", &self.vector[col as usize][row as usize])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}
