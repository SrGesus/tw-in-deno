use tw_in_deno::TailwindBuilder;

fn main() {
    let mut builder = TailwindBuilder::default();
    builder.add_classes("bg-blue-500 text-white p-4");

    println!("{}", builder.bundle());
}
