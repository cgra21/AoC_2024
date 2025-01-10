use std::collections::VecDeque;

fn main() {
    let input = "12345";



    let mut files: VecDeque<(usize, usize, usize)> = VecDeque::new(); // File List (pos, size, file_id)
    let mut spaces: VecDeque<(usize, usize)> = VecDeque::new(); // Space list (pos, size)
    let mut Final: Vec<Option<usize>> = Vec::new(); // Final list file_id or none

    let mut file_id: usize = 0;
    let mut pos: usize = 0;

    for (idx, ch) in input.chars().enumerate() {

        let size = ch.to_string().parse::<usize>().expect("Invalid digit");

        if idx%2 == 0 {
            files.push_back((pos, size, file_id));

            for _ in 0..size {
                Final.push(Some(file_id));
                pos += 1;
            }

            file_id += 1;
        } else {
            spaces.push_back((pos, size));

            for _ in 0..size {
                Final.push(None);
                pos += 1;
            }
        }
    }
    
    println!("{Final:?}");
}
