pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut matrix_with_marks: Vec<Vec<i32>> = minefield.iter().
        map(|x| {
            x.chars().map(|c| {
                if c.to_string() == "*" {
                    -1
                } else {
                    0
                }
            }).collect()
        }).
        collect();
    let matrix_with_marks_clone = matrix_with_marks.clone();

    for (i_y, v_y) in matrix_with_marks_clone.iter().enumerate() {
        for (i_x, v_x) in v_y.iter().enumerate() {
            if *v_x == -1 {
                mark_square(&mut matrix_with_marks, (i_y, i_x));
            }
        }
    }

    matrix_with_marks.iter().map(|v| {
        v.iter().map(|&y| {
            match y {
                -1 => String::from("*"),
                0 => String::from(" "),
                _ => y.to_string(),
            }
        }).collect::<Vec<String>>().join("")
    }).collect()
}

fn mark_square(matrix: &mut [Vec<i32>], position: (usize, usize)) {
    let y_position = position.0 as i32;
    let x_position = position.1 as i32;
    let square_positions: [(i32, i32); 9] = [
        (y_position - 1, x_position - 1), (y_position - 1, x_position), (y_position - 1, x_position + 1),
        (y_position, x_position - 1), (y_position, x_position), (y_position, x_position + 1),
        (y_position + 1, x_position - 1), (y_position + 1, x_position), (y_position + 1, x_position + 1),
    ];

    for y_x in square_positions.iter() {
        let y = y_x.0 as usize;
        let x = y_x.1 as usize;

        let value = match matrix.get(y) {
            Some(y_v) => {
                match y_v.get(x) {
                    Some(x_v) => x_v,
                    None => continue,
                }
            },
            None => continue,
        };

        if *value != -1 {
            matrix[y][x] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rows() {
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn no_columns() {
        let input = &[""];
        let expected = &[""];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn no_mines() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "   ",
            "   ",
            "   ",
        ], &[
            "   ",
            "   ",
            "   ",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn minefield_with_only_mines() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "***",
            "***",
            "***",
        ], &[
            "***",
            "***",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "   ",
            " * ",
            "   ",
        ], &[
            "111",
            "1*1",
            "111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "***",
            "* *",
            "***",
        ], &[
            "***",
            "*8*",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn horizontal_line() {
        let input = &[" * * "];
        let expected = &["1*2*1"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn horizontal_line_mines_at_edges() {
        let input = &["*   *"];
        let expected = &["*1 1*"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            " ",
            "*",
            " ",
            "*",
            " ",
        ], &[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "*",
            " ",
            " ",
            " ",
            "*",
        ], &[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn cross() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            "  *  ",
            "  *  ",
            "*****",
            "  *  ",
            "  *  ",
        ], &[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn large_minefield() {
        #[rustfmt::skip]
        let (input, expected) = (&[
            " *  * ",
            "  *   ",
            "    * ",
            "   * *",
            " *  * ",
            "      ",
        ], &[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
}
