use prompt_testing_lib::{self, DobbiKovModels};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //test_on_local_gemma().await;
    test_gemma_by_api().await;
    Ok(())
}

async fn test_on_local_gemma() {
    let prompt = prompt_testing_lib::chunker::read_string_file(
        "/Users/dobbikov/Desktop/stage/prompts/prompt3",
    );
    let fixer_prompt = prompt_testing_lib::chunker::read_string_file(
        "/Users/dobbikov/Desktop/stage/prompts/prompt_fixer",
    );

    let mut message = "".to_string();
    //message.push_str("<document>\n");
    message.push_str(
        prompt_testing_lib::chunker::read_string_file(
            "/Users/dobbikov/Desktop/stage/comparing_results/into_python/french/file.md",
        )
        .as_str(),
    );
    //
    //message.push_str("\n</document>");

    let model = prompt_testing_lib::OllamaModelBuilder::new(DobbiKovModels::Gemma312b).build();
    let output_path = "./output.md";

    println!("Started processing your demand!");

    prompt_testing_lib::ask_estract_contents_and_write_responses_to_file(
        model,
        prompt,
        fixer_prompt,
        message,
        output_path,
        10,
    )
    .await;
    println!("Finished processing!");
}

async fn test_gemma_by_api() {
    let prompt = prompt_testing_lib::chunker::read_string_file(
        "/Users/dobbikov/Desktop/stage/prompts/prompt2",
    );

    let mut message = "".to_string();
    message.push_str(prompt.as_str());
    message.push_str("<document>\n");
    message.push_str(
        prompt_testing_lib::chunker::read_string_file(
            "/Users/dobbikov/Desktop/stage/comparing_results/into_python/french/file_short.md",
        )
        .as_str(),
    );
    message.push_str("\n</document>");

    let response = prompt_testing_lib::gemma::ask_gemma_model(message).await;
    println!("{}", response);
}
