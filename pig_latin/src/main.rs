fn main() {
    let vowel_list = vec!['a', 'e', 'i', 'o', 'u', 'w', 'y'];
    // panic, when ""
    let input = vec!["First", "Apple"];
    let mut output: Vec<String> = Vec::new();

    for (i, world) in input.iter().enumerate() {
        let mut lowercase = world.to_lowercase();
        let first_letter = lowercase.chars().next().unwrap();
        if vowel_list.contains(&first_letter) {
            output.push(lowercase + "-hay");
        } else {
            lowercase.remove(0);
            lowercase += "-";
            lowercase.push(first_letter);
            output.push(lowercase + "ay");
        }

        println!("{:?} -> {:?}", world, output[i]);
    }
}
