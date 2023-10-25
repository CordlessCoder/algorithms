use std::{
    collections::VecDeque,
    error::Error,
    io::{stdin, BufRead, BufReader},
    time::Instant,
};

type INT = usize;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: INT,
    y: INT,
}

macro_rules! pos {
    ($x:expr, $y:expr) => {
        crate::Pos { x: $x, y: $y }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Wall,
    Target,
}

#[derive(Clone)]
struct Room {
    start: Pos,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Room {
    /// Creates a new room of given size with all cells set to free
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Free; width * height],
            start: pos!(0, 0),
        }
    }
    fn index_for_pos(&self, pos: Pos) -> Option<usize> {
        (self.width > pos.x && self.height > pos.y).then(|| pos.y * self.width + pos.x)
    }
    fn set_start(&mut self, start: Pos) {
        self.start = start;
    }
    fn get(&self, pos: Pos) -> Option<Cell> {
        self.cells.get(self.index_for_pos(pos)?).copied()
    }
    fn get_mut(&mut self, pos: Pos) -> Option<&mut Cell> {
        if pos.x >= self.width {
            return None;
        };
        let idx = self.index_for_pos(pos)?;
        self.cells.get_mut(idx)
    }
    fn set(&mut self, pos: Pos, val: Cell) {
        let idx = self
            .index_for_pos(pos)
            .expect("Attempted to set out of bounds");
        self.cells[idx] = val;
    }
}

fn solve(mut room: Room, depth_limit: u32, queue: &mut VecDeque<(Pos, u32)>) -> Option<u32> {
    use Cell::*;
    let pos = room.start;
    room.set(pos, Wall);
    queue.clear();
    queue.push_back((pos, 0));

    while let Some((pos, mut step)) = queue.pop_front() {
        step += 1;

        let mut try_move = |pos| match room.get(pos)? {
            Wall => None,
            Free => {
                if step + 1 >= depth_limit {
                    return None;
                }
                queue.push_back((pos, step));
                // No point ever going on the same cell twice
                room.set(pos, Wall);
                None
            }
            Target => Some(step),
        };

        let Pos { x, y } = pos;
        if let Some(step) = [
            (x != 0).then(|| try_move(pos!(x - 1, y))).flatten(),
            (y != 0).then(|| try_move(pos!(x, y - 1))).flatten(),
            try_move(pos!(x + 1, y)),
            try_move(pos!(x, y + 1)),
        ]
        .into_iter()
        .find_map(|step| step)
        {
            return Some(step);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_line(&mut buf)?;
    let tests: u32 = buf.trim().parse()?;
    let mut queue = VecDeque::with_capacity(64);
    for _ in 0..tests {
        let room = parse_room(&mut reader, &mut buf)?;
        let start = Instant::now();
        match solve(room, 60, &mut queue) {
            Some(steps) => println!("{steps}"),
            None => println!("#notworth"),
        }
        let took = start.elapsed();
        println!("Took {took:?}");
    }
    Ok(())
}
fn parse_room<R: BufRead>(mut reader: R, buf: &mut String) -> Result<Room, Box<dyn Error>> {
    buf.clear();
    reader.read_line(buf)?;
    let mut numbers = buf.split(' ').map(|n| n.trim().parse());
    let width = numbers.next().ok_or("No width provided")??;
    let height = numbers.next().ok_or("No height provided")??;

    let mut room = Room::new(width, height);

    for y in 0..height {
        buf.clear();
        reader.read_line(buf)?;
        let input = buf.trim_end();
        if input.len() != width {
            return Err("Room input not as wide as expected".into());
        };
        for (x, b) in input.bytes().enumerate() {
            let cell = match b {
                // This would technically be a no-op as we initialize all cells to Open/Free
                b'O' => continue,
                b'X' => Cell::Wall,
                b'W' => Cell::Target,
                b'C' => {
                    room.set_start(pos!(x, y));
                    continue;
                }
                b => {
                    return Err(
                        format!("Encountered invalid byte {b} or char {c}", c = b as char).into(),
                    )
                }
            };
            *room.get_mut(pos!(x, y)).unwrap() = cell;
        }
    }
    Ok(room)
}

#[cfg(test)]
mod test {
    use crate::{parse_room, Cell};
    use std::io::Cursor;

    #[test]
    fn room_parsing_1() {
        let input = "27 5
OOCOOOOOOOOOOOOOOOOOOOOOOOO
XXXXXXXXXXXXXXXXXXXXXXXXXXO
OOOOOOOOOOOOOOOOOOOOOOOOOOO
OXXXXXXXXXXXXXXXXXXXXXXXXXX
OOOOOOOOOOOOOOOOOOOOOOOOOWO";
        let mut buf = String::new();
        let room = parse_room(Cursor::new(input), &mut buf).unwrap();
        assert_eq!(room.get(pos!(25, 4)), Some(Cell::Target))
    }

    #[test]
    fn room_parsing_2() {
        let input = "5 5
OOOOO
OOOOO
OXXOO
OWXCO
OOXOO";
        let mut buf = String::new();
        let room = parse_room(Cursor::new(input), &mut buf).unwrap();
        assert_eq!(room.get(pos!(1, 3)), Some(Cell::Target))
    }
}
