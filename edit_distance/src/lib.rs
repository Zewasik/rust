use std::cmp::min;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let n = source.len() + 1;
    let m = target.len() + 1;
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; m];

    for i in 0..m {
        matrix[i][0] = i
    }

    for j in 0..n {
        matrix[0][j] = j
    }

    for i in 1..m {
        for j in 1..n {
            matrix[i][j] = min(
                matrix[i - 1][j] + 1,
                min(
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1]
                        + if source[j - 1..j] == target[i - 1..i] {
                            0
                        } else {
                            1
                        },
                ),
            )
        }
    }

    matrix[m - 1][n - 1]
}
