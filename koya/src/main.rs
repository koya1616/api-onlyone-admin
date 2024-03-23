// https://github1s.com/Apress/beginning-rust-2e/blob/main/Source%20Code/10-20.rs
fn main() {
    check_match(Result1::Success(0));
    println!("{}", add(1, 2));

    for item in vec![10, 20, 30].into_iter() {
        print!("{} ", item + 1);
    }

    let v = vec![10, 20, 30];
    for mut item in v.into_iter() {
        item += 1;
        print!("{} ", item);
    }

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.iter().map(|x| x * 2) {
        print!("{} ", n);
    }

    for (index, ch) in ['a', 'b', 'c'].iter().enumerate() {
        print!("{} {}, ", index, ch);
    }

    for a in std::env::args() {
        print!("[{}]", a);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_test() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test1() {
    // バイト数を数える
    assert_eq!(str::len("koya"), 4);
    assert_eq!(str::len("こうや"), 9);
    assert_eq!("koya".len(), 4);
    assert_eq!("こうや".len(), 9);

    // 文字数を数える
    assert_eq!("koya".chars().count(), 4);
    assert_eq!("こうや".chars().count(), 3);

    // オブジェクトのサイズを調べる
    assert_eq!(std::mem::size_of::<i32>(), 4);
    assert_eq!(std::mem::size_of_val(&12), 4);
    assert_eq!(std::mem::size_of_val(&vec![1, 2, 3]), 24);

    // 配列をソートする
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort_by(|a, b| b.cmp(a));
    assert_eq!(arr, [45, 12, 10, 8, 7, 4, 1, 0]); // 降順
    arr.sort_by(|a, b| a.cmp(b));
    assert_eq!(arr, [0, 1, 4, 7, 8, 10, 12, 45]); // 昇順

    let vs = ["Hello", ", ", "world", "!"];
    let mut result = String::new();
    for s in vs {
        result += s;
    }
    assert_eq!(result, "Hello, world!");

    // anyコンシューマー（イテレーターコンシューマー）
    assert_eq!("Hello, world!".chars().any(|c| c == 'o'), true);
    assert_eq!("Hello, world!".chars().any(|c| c == 'a'), false);
    assert_eq!([45, 8, 2, 6].into_iter().any(|n| n < 0), false);
    assert_eq!([45, 8, -2, 6].into_iter().any(|n| n < 0), true);
    assert_eq!([45, 8, 2, 6].into_iter().any(|n: i32| -> bool { n < 0 }), false);
    assert_eq!([45, 8, -2, 6].into_iter().any(|n: i32| -> bool { n < 0 }), true);

    // allコンシューマー（イテレーターコンシューマー）
    assert_eq!("Hello, world!".chars().all(|c| c.is_ascii()), true);
    assert_eq!("Hello, world!".chars().all(|c| c.is_alphabetic()), false);
    assert_eq!([45, 8, 2, 6].into_iter().all(|n| n > 0), true);
    assert_eq!([45, 8, -2, 6].into_iter().all(|n| n > 0), false);
    assert_eq!([45, 8, 2, 6].into_iter().all(|n: i32| -> bool { n > 0 }), true);
    assert_eq!([45, 8, -2, 6].into_iter().all(|n: i32| -> bool { n > 0 }), false);

    // collectコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
    let v: Vec<i32> = (0..5).map(|n| n * 2).collect();
    assert_eq!(v, vec![0, 2, 4, 6, 8]);
    let v: Vec<i32> = (0..5).filter(|n| n % 2 == 0).collect();
    assert_eq!(v, vec![0, 2, 4]);
    let v: Vec<i32> = (0..5).filter(|n| n % 2 == 0).map(|n| n * 2).collect();
    assert_eq!(v, vec![0, 4, 8]);
    let v: Vec<i32> = (0..5).filter(|n| n % 2 == 0).map(|n| n * 2).rev().collect();
    assert_eq!(v, vec![8, 4, 0]);

    // countコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).count(), 5);
    assert_eq!((0..5).filter(|n| n % 2 == 0).count(), 3);
    assert_eq!((0..5).filter(|n| n % 2 == 0).map(|n| n * 2).count(), 3);
    assert_eq!((0..5).filter(|n| n % 2 == 0).map(|n| n * 2).rev().count(), 3);
    assert_eq!("€èe".chars().count(), 3); // 文字数
    assert_eq!("€èe".len(), 6); // バイト数

    // findコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).find(|n| n % 2 == 0), Some(0));
    assert_eq!((0..5).find(|n| n % 2 == 1), Some(1));
    assert_eq!((0..5).find(|n| n % 2 == 2), None);
    assert_eq!("Hello, world!".chars().find(|c| *c == 'o'), Some('o'));
    assert_eq!("Hello, world!".chars().find(|c| *c == 'a'), None);

    // foldコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).fold(0, |acc, n| acc + n), 10);
    assert_eq!((0..5).fold(1, |acc, n| acc * n), 0);

    // for_eachコンシューマー（イテレーターコンシューマー）
    let mut sum = 0;
    (0..5).for_each(|n| sum += n);
    assert_eq!(sum, 10);
    let mut sum = 0;
    (0..5).for_each(|n| sum += n * 2);
    assert_eq!(sum, 20);

    // lastコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).last(), Some(4));

    // mapコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).map(|n| n * 2).collect();
    assert_eq!(v, vec![0, 2, 4, 6, 8]);

    // maxコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).max(), Some(4));

    // minコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).min(), Some(0));

    // nthコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).nth(2), Some(2));
    assert_eq!((0..5).nth(5), None);

    // positionコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).position(|n| n == 2), Some(2));
    assert_eq!((0..5).position(|n| n == 5), None);

    // productコンシューマー（イテレーターコンシューマー）
    assert_eq!((1..5).product::<i32>(), 24); // 1 * 2 * 3 * 4 = 24

    // revコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).rev().collect();
    assert_eq!(v, vec![4, 3, 2, 1, 0]);

    // skipコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).skip(2).collect();
    assert_eq!(v, vec![2, 3, 4]);

    // skip_whileコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).skip_while(|n| *n < 2).collect();
    assert_eq!(v, vec![2, 3, 4]);

    // sumコンシューマー（イテレーターコンシューマー）
    assert_eq!((0..5).sum::<i32>(), 10);

    // takeコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).take(2).collect();
    assert_eq!(v, vec![0, 1]);

    // take_whileコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).take_while(|n| *n < 2).collect();
    assert_eq!(v, vec![0, 1]);

    // zipコンシューマー（イテレーターコンシューマー）
    let v: Vec<(i32, i32)> = (0..5).zip(5..10).collect();
    assert_eq!(v, vec![(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]);
    let v: Vec<(i32, i32)> = (0..5).zip(5..9).collect();
    assert_eq!(v, vec![(0, 5), (1, 6), (2, 7), (3, 8)]);

    // chainコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).chain(5..10).collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // cycleコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).cycle().take(10).collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4, 0, 1, 2, 3, 4]);

    // enumerateコンシューマー（イテレーターコンシューマー）
    let v: Vec<(usize, i32)> = (0..5).enumerate().collect();
    assert_eq!(v, vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)]);
    let v: Vec<(usize, i32)> = (0..5).enumerate().map(|(i, n)| (i * 2, n * 2)).collect();
    assert_eq!(v, vec![(0, 0), (2, 2), (4, 4), (6, 6), (8, 8)]);

    // filter_mapコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).filter_map(|n| if n % 2 == 0 { Some(n) } else { None }).collect();
    assert_eq!(v, vec![0, 2, 4]);

    // flat_mapコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).flat_map(|n| vec![n, n * 2]).collect();
    assert_eq!(v, vec![0, 0, 1, 2, 2, 4, 3, 6, 4, 8]);

    // fuseコンシューマー（イテレーターコンシューマー）
    let mut iter = (0..3).fuse();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // inspectコンシューマー（イテレーターコンシューマー）
    let mut sum = 0;
    let v: Vec<i32> = (0..5).inspect(|n| sum += n).collect();
    assert_eq!(sum, 10);
    assert_eq!(v, vec![0, 1, 2, 3, 4]);

    // peekableコンシューマー（イテレーターコンシューマー）
    let mut iter = (0..5).peekable();
    assert_eq!(iter.peek(), Some(&0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.peek(), Some(&1));

    // skip_whileコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..5).skip_while(|n| *n < 2).collect();
    assert_eq!(v, vec![2, 3, 4]);

    // step_byコンシューマー（イテレーターコンシューマー）
    let v: Vec<i32> = (0..10).step_by(2).collect();
    assert_eq!(v, vec![0, 2, 4, 6, 8]);
}

#[allow(dead_code)]
#[allow(clippy::needless_return)]
#[allow(clippy::let_unit_value)]
fn type_check() {
    let x: () = ();
    return x;
}

// 空タプル
#[test]
fn type_check_test() {
    assert_eq!(type_check(), ());
}

#[allow(dead_code)]
enum Result1 {
    Success(u8),
    Failure(u16, char),
    Uncertainty,
}

fn check_match(result: Result1) -> &'static str {
    match result {
        Result1::Success(0) => "Success: 0",
        Result1::Success(1) => "Success: 1",
        Result1::Success(_) => "Success: other",
        Result1::Failure(10, 'X') => "Failure: 10 X",
        Result1::Failure(10, _) => "Failure: 10",
        Result1::Failure(_, 'X') => "Failure: X",
        Result1::Failure(_, _) => "Failure: other",
        Result1::Uncertainty => "Uncertainty",
    }
}

#[test]
fn check_match_test() {
    assert_eq!(check_match(Result1::Success(0)), "Success: 0");
    assert_eq!(check_match(Result1::Success(1)), "Success: 1");
    assert_eq!(check_match(Result1::Success(2)), "Success: other");
    assert_eq!(check_match(Result1::Failure(10, 'X')), "Failure: 10 X");
    assert_eq!(check_match(Result1::Failure(10, 'Y')), "Failure: 10");
    assert_eq!(check_match(Result1::Failure(20, 'X')), "Failure: X");
    assert_eq!(check_match(Result1::Failure(20, 'Y')), "Failure: other");
    assert_eq!(check_match(Result1::Uncertainty), "Uncertainty");
}

#[allow(dead_code)]
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err("Divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

#[test]
fn divide_test() {
    assert_eq!(divide(8., 2.), Ok(4.));
    assert_eq!(divide(8., 0.), Err("Divide by zero".to_string()));

    let r1 = divide(8., 2.);
    let r2 = divide(8., 0.);
    match r1 {
        Ok(value) => assert_eq!(value, 4.),
        Err(_) => assert!(false),
    }
    match r2 {
        Ok(_) => assert!(false),
        Err(message) => assert_eq!(message, "Divide by zero"),
    }

    let r3 = divide(8., 2.);
    let r4 = divide(8., 0.);
    assert_eq!(r3.is_ok(), true);
    assert_eq!(r4.is_err(), true);
    assert_eq!(r3.unwrap(), 4.);
    assert_eq!(r4.unwrap_err(), "Divide by zero");
}
