use std::{env, time::Duration};

use anyhow::Result;
use obws::{Client, requests::{inputs}};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: <scene_name> <source>");
        return Ok(())
    }

    let scene_name = &args[1];
    let source = &args[2];
    let input = &args[3];

    // Now you can use scene_name and source in your program.
    println!("Scene Name: {}", scene_name);
    println!("Source: {}", source);

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;
    // get all supported input kinds
println!("{:#?}", client.inputs().list_kinds(false).await?);
// get the settings for a specific kind
println!("{:#?}", client.inputs().default_settings::<serde_json::Value>("vlc_source").await?);
    // Get all supported input kinds
    let kinds = client.inputs().list_kinds(false).await?;

    for kind in kinds {
        // Get the settings for the current kind
        let default_settings = client.inputs().default_settings::<serde_json::Value>(&kind).await?;

        println!("Kind: {}", kind);
        println!("Default Settings: {:#?}", default_settings);
    }

    let c=client.scenes().list().await?;
   client.scenes().create(&scene_name).await.ok();
        //client.scene_collections().create(&scene_name).await?;
    client
            .scenes()
            .set_current_program_scene(&scene_name)
            .await.ok();
        //client.inputs().remove(&source).await?;
let dd=client.inputs().settings::<serde_json::Value>(source).await.ok();
let dd=dbg!(dd);
client.inputs().remove(source).await.ok();
        println!("dude: {:?}",&dd);
client
    .inputs()
    .create(inputs::Create {
        scene: &scene_name,
        input: &source,
        kind: "ffmpeg_source",
        settings: Some(serde_json::json! {{
            "input": input,
            "is_local_file": false,
            "clear_on_media_end": false,
            "restart_on_activate": false,
        }}),
        enabled: None,
    })
    .await?;
    Ok(())
}
