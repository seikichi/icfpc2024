use std::{collections::VecDeque, vec};

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

// 速度の絶対値の制約が無い状態で最適な移動を計算する
pub fn make_move2(f: &Vec<i64>, t: &Vec<i64>, v: &Vec<i64>) -> (Vec<i64>, Vec<char>) {
    let max_dist = 20;
    if (t[0] - f[0]).abs() > max_dist || (t[1] - f[1]).abs() > max_dist {
        // println!("failed");
        return make_move(f, t, v);
    }
    let mut que = VecDeque::new();
    que.push_back((f.clone(), v.clone(), vec![]));
    let mut iter = 0;
    while let Some((cp, cv, cm)) = que.pop_front() {
        if cp == *t {
            // println!("{:?} {:?} {:?}", cp, cv, cm);
            return (cv, cm);
        }
        iter += 1;
        if iter == 100 {
            break;
        }
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nv = vec![cv[0] + dx, cv[1] + dy];
                let np = vec![cp[0] + nv[0], cp[1] + nv[1]];
                let c = (((dy + 1) * 3 + dx + 1) as u8 + '1' as u8) as char;
                let mut ncm = cm.clone();
                if (t[0] - np[0]).abs() > max_dist || (t[1] - np[1]).abs() > max_dist {
                    continue;
                }
                ncm.push(c);
                que.push_back((np, nv, ncm));
            }
        }
    }
    // println!("failed");
    return make_move(f, t, v);
}
