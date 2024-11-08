pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, row)| {
            let row = row.iter();
            let &row_max = row.clone().max().unwrap_or(&0);

            for (j, &cell) in row.enumerate() {
                let col: Vec<u64> = input.iter().filter_map(|row| row.get(j)).cloned().collect();
                let &col_min = col.iter().min().unwrap_or(&u64::MAX);

                if cell == row_max && cell == col_min {
                    acc.push((i, j));
                }
            }
            acc
        })
}
