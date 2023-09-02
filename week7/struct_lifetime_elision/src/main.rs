struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    let mut tweet = Tweet {
        content: "example",
    };

    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);

    println!("{}", take_and_return_content("hello"));

}

// lifetime elision
fn take_and_return_content(content: &str) -> &str {
    content
}