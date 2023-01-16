mod min_max_lib;
use min_max_lib::array_max_len;
use min_max_lib::matrix_max_len;

pub fn printable_matrix(m: Vec<Vec<String>>, margin: usize) -> Vec<String> {
    let box_corners = ["┏", "┓", "┣", "┫", "┃", "╋", "┗", "┛", "┻"];
    let pad = matrix_max_len(&m);
    let mut lines = vec![];
    let mut sep = String::new();
    let margin_space = " ".repeat(margin);
    let x = m.len();
    let y = m[0].len();
    for i in 0..x {
        let mut corner1 = String::new();
        let mut corner2 = String::new();
        let mut line = String::new();
        if i == 0 {
            corner1 = box_corners[0].to_string();
            corner2 = box_corners[1].to_string();
        } else if i > 0 {
            corner1 = box_corners[2].to_string();
            corner2 = box_corners[3].to_string();
        }
        sep = gen_separator(y, i, pad, 0);
        lines.push(margin_space.to_owned() + &corner1 + &sep + &corner2);
        for j in 0..y {
            let element_to_show = m[i][j].to_string();
            line += &(" ".repeat(pad - element_to_show.len()) + &element_to_show + box_corners[4]);
        }
        lines.push(margin_space.to_owned() + box_corners[4] + &line);
    }
    lines.push(
        margin_space.to_owned()
            + box_corners[6]
            + &sep.replace(box_corners[5], box_corners[8])
            + box_corners[7],
    );
    return lines;
}

pub fn print_array_h(t: Vec<String>, margin: usize, empty: usize) -> () {
    let box_corners = ["┏", "┓", "┃", "┗", "┛"];
    let pad = array_max_len(&t);
    let margin_space = " ".repeat(margin);
    let n = t.len();
    let mut sep = gen_separator(n, 0, pad, 0);
    println!(
        "{}{}{}{}",
        margin_space.to_owned(),
        box_corners[0],
        sep,
        box_corners[1]
    );
    print!("{}{}", margin_space.to_owned(), box_corners[2]);
    for i in 0..n {
        let element_to_show = t[i].to_string();
        print!(
            "{}{}{}",
            " ".repeat(pad - element_to_show.len()),
            element_to_show,
            box_corners[2]
        );
    }
    sep = gen_separator(n, n, pad, empty);
    print!(
        "\n{}{}{}{}",
        margin_space.to_owned(),
        box_corners[3],
        sep,
        box_corners[4]
    )
}
pub fn print_array_h_no_corners(t: Vec<String>, margin: usize) -> () {
    let pad = array_max_len(&t);
    let margin_space = " ".repeat(margin);
    let n = t.len();
    print!("{}", margin_space);
    for i in 0..n {
        let element_to_show = t[i].to_string();
        print!(
            " {}{}",
            " ".repeat(pad - element_to_show.len()),
            element_to_show,
        );
    }
    println!("\n{}  ", margin_space,)
}
pub fn printable_array_v(t: Vec<String>, margin: usize) -> Vec<String> {
    let box_corners = ["━", "┏", "┓", "┃", "┣", "┫", "┗", "┛"];
    let mut lines = vec![];
    let pad = array_max_len(&t);
    let margin_space = " ".repeat(margin);
    let sep = box_corners[0].repeat(pad);
    let n = t.len();
    lines.push(margin_space.to_owned() + box_corners[1] + &sep + box_corners[2]);
    for i in 0..n - 1 {
        let element = t[i].to_string();
        let element_length = element.len();
        lines.push(
            margin_space.to_owned()
                + box_corners[3]
                + &" ".repeat(pad - element_length)
                + &element
                + box_corners[3],
        );
        lines.push(margin_space.to_owned() + box_corners[4] + &sep + box_corners[5]);
    }
    let element = &t[n - 1];
    let element_length = element.len();
    lines.push(
        margin_space.to_owned()
            + box_corners[3]
            + &" ".repeat(pad - element_length)
            + &element
            + box_corners[3],
    );
    lines.push(margin_space.to_owned() + box_corners[6] + &sep + box_corners[7]);
    return lines;
}

pub fn printable_array_v_no_corners(t: Vec<String>, margin: usize) -> Vec<String> {
    let mut lines = vec![];
    let pad = array_max_len(&t);
    let margin_space = " ".repeat(margin);
    let n = t.len();
    lines.push(margin_space.to_owned() + &" ".repeat(2));
    for i in 0..n {
        let element = t[i].to_string();
        let element_length = element.len();
        lines.push(margin_space.to_owned() + &" ".repeat(pad - element_length) + &element + " ");
        lines.push(margin_space.to_owned() + &" ".repeat(2));
    }
    return lines;
}

pub fn gen_separator(n: usize, i: usize, pad: usize, empty: usize) -> String {
    let mut sep = "".to_string();
    let l = n * 2 - 1;
    for s in 0..l {
        let mut line = "━".repeat(pad);
        if s % 2 != 0 && empty == 0 {
            if i == 0 {
                line = "┳".to_string()
            } else if 0 < i && i < n {
                line = "╋".to_string()
            } else if i == n {
                line = "┻".to_string()
            }
        } else if empty != 0 {
            line = " ".repeat(pad)
        }
        sep = sep + &line
    }
    return sep;
}