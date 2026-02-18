use crate::collections::Vec;

#[test]
fn make_empty() {
    let _v: Vec<&str> = Vec::new();
}

#[test]
fn insert_10() {
    let mut v = Vec::new();
    v.push("one");
    v.push("two");
    v.push("three");
    v.push("four");
    v.push("five");
    v.push("six");
    v.push("seven");
    v.push("eight");
    v.push("nine");
    v.push("ten");

    assert_eq!(v.remove(0), "one");
    assert_eq!(v.remove(0), "two");
    assert_eq!(v.remove(0), "three");
    assert_eq!(v.remove(0), "four");
    assert_eq!(v.remove(0), "five");
    assert_eq!(v.remove(0), "six");
    assert_eq!(v.remove(0), "seven");
    assert_eq!(v.remove(0), "eight");
    assert_eq!(v.remove(0), "nine");
    assert_eq!(v.remove(0), "ten");
}

#[test]
fn sort() {
    let mut v = Vec::new();
    v.push(5);
    v.push(2);
    v.push(1);
    v.push(8);
    v.push(6);
    v.push(4);
    v.push(2);
    v.push(9);

    v.sort();

    assert_eq!(&[1, 2, 2, 4, 5, 6, 8, 9], &*v);
}
