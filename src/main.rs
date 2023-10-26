const WIDTH: usize = 5;
const DEPTH: usize = 5;
const HEIGHT: usize = 5;

const CONFIGURATIONS: [[[i32; 3]; 5]; 24] = [
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [1, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [2, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [2, 0, 1]],
    [[0, 0, 0], [-2, 1, 0], [-1, 1, 0], [0, 1, 0], [1, 1, 0]],
    [[0, 0, 0], [-1, 1, 0], [0, 1, 0], [1, 1, 0], [2, 1, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [-1, 1, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [0, 1, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [1, 1, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [-1, 2, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [0, 2, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [1, 2, 0]],
    [[0, 0, 0], [0, 0, 1], [-2, 0, 1], [-1, 0, 1], [1, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [-1, 0, 1], [1, 0, 1], [2, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [0, -2, 1], [0, -1, 1], [0, 1, 1]],
    [[0, 0, 0], [0, 0, 1], [0, -1, 1], [0, 1, 1], [0, 2, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [-1, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, -1, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [1, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [-1, 0, 2]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, -1, 2]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [1, 0, 2]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 2]]
];

fn main() {
    let cube: [[[u8; WIDTH]; DEPTH]; HEIGHT] = [[[0u8; WIDTH]; DEPTH]; HEIGHT];

    let mut solver = CubeSolver::new(cube);

    solver.solve();

    solver.print_cube();
}

pub struct CubeSolver {
    cube: [[[u8; WIDTH]; DEPTH]; HEIGHT],
}

impl CubeSolver {
    pub fn new(cube: [[[u8; WIDTH]; DEPTH]; HEIGHT]) -> CubeSolver {
        CubeSolver { cube }
    }

    pub fn print_cube(&self) {
        for level in self.cube {
            println!("{:?}", level)
        }
        println!();
    }

    pub fn solve(&mut self) {
        let piece_id: u8 = 1;
        self.find_solution(piece_id);
    }

    fn find_solution(&mut self, piece_id: u8) {
        if piece_id > 25 {
            self.print_cube();
            return;
        }

        let (x, y, z) = self.find_position();

        for config_id in 0..CONFIGURATIONS.len() {
            if self.insert_piece(x, y, z, config_id, piece_id) {
                self.find_solution(piece_id + 1u8);
                self.remove_piece(piece_id);
            }
        }
    }

    fn find_position(&self) -> (i32, i32, i32) {
        for z in 0..HEIGHT {
            for y in 0..DEPTH {
                for x in 0..WIDTH {
                    if self.cube[z][y][x] == 0u8 {
                        return (x as i32, y as i32, z as i32);
                    }
                }
            }
        }
        (0, 0, 0)
    }


    fn insert_piece(&mut self, x: i32, y: i32, z: i32, config_id: usize, piece_id: u8) -> bool {
        let config: [[i32; 3]; 5] = CONFIGURATIONS[config_id];

        // Check if piece fits
        for block in config {
            let block_x: i32 = x + block[0];
            let block_y: i32 = y + block[1];
            let block_z: i32 = z + block[2];

            // Out of bounds
            if (block_x < 0) || (block_x >= WIDTH as i32) || (block_y < 0) || (block_y >= DEPTH as i32) || (block_z < 0) || (block_z >= HEIGHT as i32) {
                return false;
            }
            // Slot taken
            if self.cube[block_x as usize][block_y as usize][block_z as usize] != 0 {
                return false;
            }
        }

        // Insert piece
        for block in config {
            self.cube[(z + block[2]) as usize][(y + block[1]) as usize][(x + block[0]) as usize] = piece_id
        }

        true
    }


    fn remove_piece(&mut self, piece_id: u8) {
        for z in 0..HEIGHT {
            for y in 0..DEPTH {
                for x in 0..WIDTH {
                    if self.cube[z][y][x] == piece_id {
                        self.cube[z][y][x] = 0u8;
                    }
                }
            }
        }
    }
}
