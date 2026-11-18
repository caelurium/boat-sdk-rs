use boat_sdk::agents::PromptRequest;
use boat_sdk::Client;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox_id = std::env::args()
        .nth(1)
        .ok_or("usage: prompt <sandbox-id>")?;
    let client = Client::new(&std::env::var("BOAT_API_KEY")?)?;
    let request = PromptRequest {
        new_conversation: Some(true),
        ..PromptRequest::new("Describe the workspace in one sentence.")
    };
    let queued = client.prompt(&sandbox_id, &request).await?;
    println!("Prompt: {}", queued.prompt_id);
    if let Some(id) = queued.conversation_id {
        println!("Conversation: {id}");
    }

    // Timing out stops local polling, not the remote turn. Use the printed ID to resume polling.
    let status = tokio::time::timeout(Duration::from_secs(300), async {
        loop {
            let run = client
                .prompt_status(&sandbox_id, &queued.prompt_id)
                .await?
                .prompt_run;
            if run.done {
                return Ok::<_, boat_sdk::Error>(run.status);
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    })
    .await??;
    println!("Final status: {status}");
    if status != "finished" {
        return Err("prompt did not finish successfully".into());
    }
    Ok(())
}
