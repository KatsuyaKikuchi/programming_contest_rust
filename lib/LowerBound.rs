// return max index that f(value,v[index])==true
fn lower_bound<T>(value: T, v: &Vec<T>, f: Box<dyn Fn(&T, &T) -> bool>) -> Option<usize> {
    let n = v.len();
    if n == 0 {
        return None;
    }
    if !f(&value, &v[0]) {
        return None;
    }

    let (mut ret, mut ng) = (0, n);
    while (ng - ret) > 1 {
        let mid = (ng + ret) / 2;
        if f(&value, &v[mid]) {
            ret = mid;
        } else {
            ng = mid;
        }
    }
    Some(ret)
}