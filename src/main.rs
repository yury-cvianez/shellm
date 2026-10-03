mod cli;
mod completion;

fn main() {
    // activate the repl
    cli::repl::activate();

    // test autcomplete with qwen and llama.cp
    // let qwen = completion::request::RequestSlm::new();

    // let completions = qwen
    //     .complete("cd")
    //     .expect("completion failed");

    // for completion in completions {
    //     println!("{:?}", completion.text);
    // }
    

}
