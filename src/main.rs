use ra_build_files_missing::protos::example;

fn main() {
    println!("Hello, world!");
    let mut example_msg = example::ExampleMsg::new();
    example_msg.set_test(5);
    println!("{}", example_msg);
}
