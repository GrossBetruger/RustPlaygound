fn main() {
    let a = vec![1, 2, 3];
    let iter = a.iter().skip(1);
    let rest = iter.size_hint().0;
    let taken = iter.take(rest);

    println!("toke: {:?}", taken.into_iter().collect::<Vec<&u32>>());

    let padded: Vec<u32> = vec![0, 0, 0, 1, 2, 3, 0, 0];

    println!("got: {:?}", padded
        .iter()
        .skip_while(|x| **x == 0)
        .collect::<Vec<&u32>>()
        .iter()
        .rev()
        .skip_while(|x| ***x == 0)
        .collect::<Vec<&&u32>>());

}
