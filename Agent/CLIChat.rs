use std::io::{self, Write};
use crate::completion::{Chat, Message, PromptError};

/// Creates a simple REPL (Read-Eval-Print Loop) CLI chatbot using a type that implements the `Chat` trait.
/// 
/// This function runs an interactive chatbot session where the user can type prompts,
/// receive responses, and see the chat log evolve. Type 'exit' to quit the session.
/// 
/// # Arguments
/// - `chatbot`: An instance of a type that implements the `Chat` trait.
/// 
/// # Returns
/// - `Result<(), PromptError>`: Returns `Ok` on successful completion or an error of type `PromptError`.
pub async fn cli_chatbot(chatbot: impl Chat) -> Result<(), PromptError> {
    let stdin = io::stdin(); // Standard input for user prompts
    let mut stdout = io::stdout(); // Standard output for displaying messages
    let mut chat_log = vec![]; // Chat history log to keep track of interactions

    // Initial welcome message
    println!("Welcome to the chatbot! Type 'exit' to quit.");

    // Main loop for REPL
    loop {
        print!("> "); // Prompt symbol for user input
        stdout.flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        
        // Read user input from the standard input
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // Remove leading and trailing whitespace

                // Exit condition
                if input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }

                tracing::info!("Prompt:\n{}\n", input); // Log the user's input for debugging

                // Obtain chatbot response asynchronously
                match chatbot.chat(input, chat_log.clone()).await {
                    Ok(response) => {
                        // Add user input and chatbot response to the chat log
                        chat_log.push(Message {
                            role: "user".to_string(),
                            content: input.to_string(),
                        });
                        chat_log.push(Message {
                            role: "assistant".to_string(),
                            content: response.clone(),
                        });

                        // Display the chatbot's response in a formatted block
                        println!("========================== Response ============================");
                        println!("{}", response);
                        println!("================================================================\n");

                        tracing::info!("Response:\n{}\n", response); // Log the chatbot's response for debugging
                    }
                    Err(error) => {
                        // Handle errors from the chatbot
                        eprintln!("Error generating response: {}", error);
                    }
                }
            }
            Err(error) => {
                // Handle errors reading user input
                eprintln!("Error reading input: {}", error);
            }
        }
    }

    Ok(())
}
