pub fn split(sentence: &String) -> &str {
  let string_as_bytes = sentence.as_bytes();

  for (i, &item) in string_as_bytes.iter().enumerate() {
    if item == b' ' {
      return &sentence[0..i];
    }
  }

  &sentence[0..]
}