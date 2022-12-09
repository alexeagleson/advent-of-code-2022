pub mod input;

pub fn day_8_part_1(input: &'static str) -> String {
    let forest = parse_input(input);

    let visible_trees = count_visible_trees(&forest);

    visible_trees.to_string()
}

pub fn day_8_part_2(input: &'static str) -> String {
    let forest = parse_input(input);

    let mut scenic_score = get_scenic_scores(&forest);

    scenic_score.sort();

    let highest_score = scenic_score[scenic_score.len() - 1];

    highest_score.to_string()
}

type Tree = i32;

type Trees = Vec<Vec<Tree>>;

#[derive(Debug)]
pub struct Forest {
    width: usize,
    height: usize,
    trees: Trees,
}

fn is_visible<'a>(
    current_row_idx: usize,
    current_column_idx: usize,
    forest: &Forest,
    tree: Tree,
    direction: Option<&'static str>,
    mut visible_trees: i32,
) -> (bool, i32) {
    if current_row_idx == 0
        || current_column_idx == 0
        || current_row_idx == forest.width - 1
        || current_column_idx == forest.height - 1
    {
        return (true, visible_trees);
    }

    let left = current_row_idx - 1;
    let right = current_row_idx + 1;
    let up = current_column_idx - 1;
    let down = current_column_idx + 1;

    visible_trees += 1;

    if forest.trees[left][current_column_idx] < tree && direction == Some("left") {
        return is_visible(
            left,
            current_column_idx,
            forest,
            tree,
            direction,
            visible_trees,
        );
    } else if forest.trees[right][current_column_idx] < tree && direction == Some("right") {
        return is_visible(
            right,
            current_column_idx,
            forest,
            tree,
            direction,
            visible_trees,
        );
    } else if forest.trees[current_row_idx][up] < tree && direction == Some("up") {
        return is_visible(current_row_idx, up, forest, tree, direction, visible_trees);
    } else if forest.trees[current_row_idx][down] < tree && direction == Some("down") {
        return is_visible(
            current_row_idx,
            down,
            forest,
            tree,
            direction,
            visible_trees,
        );
    } else {
        return (false, visible_trees);
    };
}

pub fn count_visible_trees(forest: &Forest) -> String {
    let mut visibility_count: i32 = 0;
    for i in 0..forest.width {
        for j in 0..forest.height {
            let tree = forest.trees[i][j];
            let (u, _) = is_visible(i, j, &forest, tree, Some("up"), 0);
            let (r, _) = is_visible(i, j, &forest, tree, Some("right"), 0);
            let (d, _) = is_visible(i, j, &forest, tree, Some("down"), 0);
            let (l, _) = is_visible(i, j, &forest, tree, Some("left"), 0);

            if u || r || d || l {
                visibility_count += 1;
            }
        }
    }
    visibility_count.to_string()
}

pub fn get_scenic_scores(forest: &Forest) -> Vec<i32> {
    let mut scenic_scores: Vec<i32> = vec![];
    for i in 0..forest.width {
        for j in 0..forest.height {
            let tree = forest.trees[i][j];
            let (_, up_score) = is_visible(i, j, &forest, tree, Some("up"), 0);
            let (_, right_score) = is_visible(i, j, &forest, tree, Some("right"), 0);
            let (_, down_score) = is_visible(i, j, &forest, tree, Some("down"), 0);
            let (_, left_score) = is_visible(i, j, &forest, tree, Some("left"), 0);

            scenic_scores.push(up_score * right_score * down_score * left_score);
        }
    }
    scenic_scores
}

pub fn parse_input(input: &str) -> Forest {
    let mut trees: Trees = Vec::new();

    for (row_idx, line) in input.lines().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line == "" {
            continue;
        }

        trees.push(Vec::new());
        for tree_char in trimmed_line.chars() {
            trees[row_idx - 1].push(
                i32::from_str_radix(&tree_char.to_string(), 10)
                    .expect("Failed to parse tree digit"),
            );
        }
    }

    let width = trees.len();

    let height = trees[0].len();

    Forest {
        width,
        height,
        trees,
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static TEST_INPUT: &str = r#"
        30373
        25512
        65332
        33549
        35390
    "#;

    #[test]
    fn part_1() {
        let forest = parse_input(TEST_INPUT);

        assert_eq!(forest.width, 5);

        assert_eq!(forest.height, 5);

        assert_eq!(forest.trees[1][1], 5);

        let visible_trees = count_visible_trees(&forest);

        assert_eq!(visible_trees, "21".to_string());
    }

    #[test]
    fn part_2() {
        let forest = parse_input(TEST_INPUT);

        let mut scenic_score = get_scenic_scores(&forest);

        scenic_score.sort();

        let highest_score = scenic_score[scenic_score.len() - 1];

        dbg!("{:?}", &scenic_score);

        dbg!("{:?}", &highest_score);

        assert_eq!(highest_score, 8);
    }
}
