use blog_shared::Post;

fn main() {
    let post = Post::new(
        String::from("Post on the website"),
        String::from("Lets Get Rusty"),
    );

    println!("{post:?}");
}
