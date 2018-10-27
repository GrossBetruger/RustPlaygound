fn main() {
    let a = vec![1, 2, 3];
    let iter = a.iter().skip(1);
    let rest = iter.size_hint().0;
    let taken = iter.take(rest);

    println!("toke: {:?}", taken.into_iter().collect::<Vec<&u32>>());

    let padded: Vec<u32> = vec![0, 0, 0, 1, 2, 0, 3, 0, 0];

    println!("got: {:?}", padded
        // cut head padding
        .iter()
        .skip_while(|x| **x == 0)
        .collect::<Vec<&u32>>()
        // cut tail padding
        .iter()
        .rev()
        .skip_while(|x| ***x == 0)
        .collect::<Vec<&&u32>>()
        // return to original order
        .iter()
        .rev()
        .collect::<Vec<&&&u32>>());

    // scanners

    let set = 0..10000;

    let (evens, odds): (Vec<u32>, Vec<u32>) = set.partition(|x| x % 2 == 0);

    let evens_scanner = evens.iter().scan(0, |acc, x| {*acc += x; Some(*acc)});
    let odds_scanner = odds.iter().scan(0, |acc, x| {*acc += x; Some(*acc)});

    println!("odds remain larger than evens: {:?}", evens_scanner.zip(odds_scanner)
        .all(|(even_partial_sum, odd_partial_sum)| {
            odd_partial_sum > even_partial_sum
    }));

}
