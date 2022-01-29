use proc_macro;

#[some_attribute]

pub fn some_name(input: TokenStream) -> TokenStream {
  let input = input.to_string();
  let output = input.replace("some_attribute", "proc_macro_attribute");
  output.parse().unwrap()
}
