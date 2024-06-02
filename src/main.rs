fn main() {
    println!("Hello, world!");
    // sort array of any size take array as input from user and take size of array from user as well
    let mut size = String::new();
    println!("Enter size of array:");
    std::io::stdin().read_line(&mut size).unwrap();
    let size: i32 = size.trim().parse().unwrap();
    let mut arr = vec![0; size as usize];
    for i in 0..5 {
        println!("Enter element {}:", i);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();
        arr[i] = input;
    }
    println!("Array before sorting: {:?}", arr);
    arr.sort();    
}
