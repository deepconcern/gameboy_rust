fn p(v: u8) {
    println!("{}, {:#010b}, {:#04x}", v, v, v);
}

fn main() {
    let t1 = -33i8;
    let t2 = 33i8;

    let o1 = t1 as u8;
    let o2 = t2 as u8;

    p(o1);
    p(o2);
}