// tpに着いた時の速度の絶対値を1以下にする前提で最適な移動を計算する
pub fn calc_move(f: i64, t: i64, v: i64) -> i64 {
    let d = t - f;
    if v != 0 && d.signum() != v.signum() {
        return v.signum() * -1;
    }
    let length1 = v.abs() * (v.abs() + 1) / 2;
    if d.abs() < length1 {
        return v.signum() * -1;
    }
    let length2 = (v.abs() + 1) * (v.abs() + 2) / 2;
    if d.abs() < length2 {
        return 0;
    }
    return d.signum();
}

// tに着いたときの速度と移動方法を返す
pub fn make_move(f: &Vec<i64>, t: &Vec<i64>, v: &Vec<i64>) -> (Vec<i64>, Vec<char>) {
    let mut p = f.clone();
    let mut v = v.clone();
    let mut moves = vec![];
    loop {
        if p == *t {
            break;
        }
        let mut m = vec![1, 1];
        for dir in 0..2 {
            let a = calc_move(p[dir], t[dir], v[dir]);
            v[dir] += a;
            m[dir] = 1 + a;
            p[dir] += v[dir];
        }
        // println!("{:?} {:?} {:?}", p, v, m);
        let c = ((m[1] * 3 + m[0] + 1) as u8 + '0' as u8) as char;
        moves.push(c);
    }
    return (v, moves);
}
