pub mod rover {
    #[derive(Debug)]
    pub struct Rover {
        id: u32,
        alive: bool,
        map: Vec<Vec<String>>,
        path: Vec<Vec<String>>,
        orientation: u32,
        position: [u32; 2],
        mine_number: u32,
        defused: bool,
        dig_time: u128,
    }

    impl Rover {
        pub fn new(id: u32, map: Vec<Vec<String>>, dimensions: (usize, usize)) -> Self {
            Rover {
                id,
                alive: true,
                map,
                path: vec![vec!["".to_string(); dimensions.0]; dimensions.1],
                orientation: 270,
                position: [0, 0],
                mine_number: 0,
                defused: false,
                dig_time: 0,
            }
        }

        pub fn move_rover(&mut self) {}
        fn turn(&mut self) {}

        pub fn dig_mine(&mut self) {}
        fn defuse_mine(&mut self) {}

        pub fn get_path(&self) -> String {
            let path: String;
            path
        }
    }
}
