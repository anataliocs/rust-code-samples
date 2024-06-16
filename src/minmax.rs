use std::arch::aarch64::vqtbx3q_u8;


fn main() {
    let xs: [i32; 5] = [1, 3, 5, 7, 9];
    let len = xs.len();
    println!("length: {len}");

    let mut list: Vec<i64> = vec![];

    for n in 0..len {
        let &sum = &xs
            .iter()
            .enumerate()
            .filter(|(index, &y)| !index.eq(&n))
            .map(|(i, &y)| i64::from(y))
            .sum::<i64>();

        list.push(sum);
    }

    let min = list.iter()
        .min_by(|x, y| x.cmp(y))
        .unwrap();

    let max = list.iter()
        .max_by(|x, y| x.cmp(y))
        .unwrap();

    println!("{min} {max}");
}
