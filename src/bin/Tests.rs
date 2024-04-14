use std::collections::HashMap;

#[derive(Copy, PartialEq, Eq, Debug, Clone)]
enum State {
    Grey,
    Yellow,
    Green,
}

impl State {
    pub fn to_char(&self) -> char {
        match self {
            State::Grey => 'R',
            State::Yellow => 'Y',
            State::Green => 'G',
        }
    }
}
fn compare_two_words(input: [char; 5], ans: [char; 5]) -> [State; 5] {
    let mut res: [State; 5] = [State::Grey; 5];
    let mut cnt: [i32; 28] = [0 ; 28];
    for i in 0..5{
        let idx: usize = ans[i] as usize - 97;
        cnt[idx] += 1;
    }
    for i in 0..5{
        if input[i] == ans[i]{
            res[i] = State::Green;
            let idx: usize = input[i] as usize - 97;
            cnt[idx] -= 1;
        }
    }
    for i in 0..5{
        if input[i] != ans[i]{
            let idx: usize = input[i] as usize - 97;
            if cnt[idx] > 0 {
                cnt[idx] -= 1;
                res[i] = State::Yellow;
            }else{
                res[i] = State::Grey;
            }
        }
    }
    res
}
fn read_one_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn trans(s: &str) -> [char; 5] {
    let mut res = ['a'; 5];
    for (i, c) in s.chars().enumerate() {
        res[i] = c;
    }
    res
}

fn main() {
    let n = read_one_line().parse::<usize>().unwrap();
    for _ in 0..n {
        let input = read_one_line();
        let ans = read_one_line();
        let input = trans(&input);
        let ans = trans(&ans);
        let output = compare_two_words(input, ans);
        println!("{}", output.iter().map(|x| x.to_char()).collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test(input: &str, ans: &str, std: &str) {
        let input = trans(input);
        let ans = trans(ans);
        let output = compare_two_words(input, ans);
        assert_eq!(output.iter().map(|x| x.to_char()).collect::<String>(), std);
    }

    #[test]
    fn test_cargo() {
        test("floor", "cargo", "RRYRY");
        test("audio", "cargo", "YRRRG");
        test("crane", "cargo", "GYYRR");
        test("wanna", "cargo", "RGRRR");
        test("hello", "cargo", "RRRRG");
        test("boost", "cargo", "RYRRR");
        test("cargo", "cargo", "GGGGG");
    }

    #[test]
    fn test_boost() {
        test("loops", "boost", "RGGRY");
        test("loose", "boost", "RGGGR");
        test("stood", "boost", "YYGYR");
        test("boost", "boost", "GGGGG");
    }

    #[test]
    fn test_crane() {
        test("abuse", "crane", "YRRRG");
        test("wanna", "crane", "RYRGR");
        test("sleep", "crane", "RRYRR");
        test("crane", "crane", "GGGGG");
    }
}