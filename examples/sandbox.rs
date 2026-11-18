use boat_sdk::commands::CommandRequest;
use boat_sdk::sandbox::requests::CreateSandboxRequest;
use boat_sdk::sandbox::AutoStop;
use boat_sdk::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(&std::env::var("BOAT_API_KEY")?)?;
    let options = CreateSandboxRequest {
        no_env: Some(true),
        ttl_seconds: Some(AutoStop::AfterSeconds(300)),
        ..Default::default()
    };
    let sandbox = client.create_sandbox(&options, None).await?.sandbox;
    println!("Created {}", sandbox.id);

    // Request a stop even if the command fails. Auto-stop also covers an interrupted example.
    let result = client
        .run_command(&sandbox.id, &CommandRequest::new("pwd"))
        .await;
    let stop = client
        .stop_sandbox(&sandbox.id, &Default::default())
        .await?;
    println!("Stop status: {}", stop.status);

    let output = result?;
    if !output.success {
        return Err(format!("pwd failed (exit code {:?})", output.exit_code).into());
    }
    print!("{}", output.stdout);
    Ok(())
}
