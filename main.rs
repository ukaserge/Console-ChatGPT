use openai::OpenAI;
use utils::get_input;

fn main() {
    let openai = OpenAI::new();

    loop {
        let input = get_input("You: ");
        let response = openai.get_response(&input);
        println!("ChatGPT: {}", response);
    }
}
