pub fn gcd(n: u64, m: u64) -> Option<u64> {
    if n == 0 || m == 0 {
        return None;
    }

    let mut m1: u64 = m;
    let mut n1: u64 = n;

    while m1 != 0 {
        if m1 < n1 {
            std::mem::swap(&mut m1, &mut n1);
        }
        m1 %= n1;
    }

    return Some(n1);
}

pub fn gcd_vec(n: &Vec<u64>) -> Option<u64> {
    if n.iter().any(|&it| it == 0) {
        return None;
    }

    if n.len() == 1 {
        return Some(n.first().unwrap().to_owned());
    }

    let acc = gcd(*n.get(0).unwrap(), *n.get(1).unwrap());
    if n.len() == 2 {
        return acc;
    }

    return Some(n.iter().fold(acc.unwrap().to_owned(), | acc, it | gcd(acc, *it).unwrap()))
}

pub fn lcm(n: u64, m: u64) -> Option<u64> {
    if n == 0 || m == 0 {
        return None;
    }
    return Some(n * m / gcd(n, m).unwrap())
}

pub fn lcm_vec(n: &Vec<u64>) -> Option<u64> {
    if n.iter().any(|&it| it == 0) {
        return None;
    }

    if n.len() == 1 {
        return Some(n.first().unwrap().to_owned());
    }

    let acc = lcm(*n.get(0).unwrap(), *n.get(1).unwrap());
    if n.len() == 2 {
        return acc;
    }

    return Some(n.iter().fold(acc.unwrap().to_owned(), | acc, it | lcm(acc, *it).unwrap()))
}