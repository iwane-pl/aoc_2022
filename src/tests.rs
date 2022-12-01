use super::*;

#[test]
fn test_day01_p1() {
    let contents = load(r"..\..\..\..\tests\day01_p1.txt");
    
    let calories = day01_p1(&contents);

    assert_eq!(calories, 24000);
}

#[test]
fn test_day01_p2() {
    let contents = load(r"..\..\..\..\tests\day01_p1.txt");
    
    let calories = day01_p2(&contents);

    assert_eq!(calories, 45000);
}