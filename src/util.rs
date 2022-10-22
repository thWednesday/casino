pub fn removeVI(vector: Vec<String>, index: i32, element: String) -> Vec<String> {
    let mut new: Vec<String> = vector.clone();
    new.push(element);
    new.swap(index.try_into().unwrap(), vector.len());
    new.remove(new.len() - 1);

    return new;
}
