fn press_range(total_time: f32, distance: f32) -> (f32, f32) {
    let a = (-total_time * -total_time) - (4.0 * distance);
    let hold1 = ((total_time) + a.sqrt()) / 2.0;
    let hold2 = ((total_time) - a.sqrt()) / 2.0;
    return (hold1, hold2)

}
pub fn run() {
    println!("Day 6");

    let inputs = [
        (7.0, 9.0),
        (15.0, 40.0),
        (30.0, 200.0)
    ];

    let mut results:Vec<f32> = vec![];
    for (time, distance) in inputs {
        let (res1, res2) = press_range(time, distance);
        let result = res1.ceil() - res2.floor() - 1.0;
        dbg!(res1.floor());
        dbg!(res2.ceil());
        dbg!(result);
        results.push(result);
    }

    println!("{}", results.iter().product::<f32>());
}