use std::{env, time::Duration};
use serde_json::json;

use anyhow::Result;
use obws::{Client, requests::{inputs::{self, SetSettings}, scene_items::{SetTransform, SceneItemTransform, Scale, Id, SetPrivateSettings}}, common::Alignment};

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
let mut dd=client.inputs().settings::<serde_json::Value>(&source).await.ok();
let source_id=client.scene_items().id(Id{scene:&scene_name ,source:&source,search_offset:None}).await;
match source_id {
    Ok(id)=>{
        println!("SOURCE ID: {}",id);
        let mut p=client.scene_items().private_settings::<serde_json::Value>(&scene_name, id).await?;
        println!("SOURCE SETTINGS: {} {}",p,"test");
        if input!=&p["input"].to_string() {
        p["input"]=json!(input);
        client.scene_items().set_private_settings(SetPrivateSettings{scene:&scene_name, item_id: id,settings:&p}).await;
        }else {
        println!("SOURCE ALREADY: {}",input);
        }
    },
    Err(e)=>{
        println!("SOURCE NONE {}",e);
let item_id=client
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
    // scene:&scene_name,
    // item_id: item_id,
    let mut t: SceneItemTransform = Default::default();
    t.scale=Some(Scale{x:Some(1.0),y:Some(1.0)});
client.scene_items().set_transform(
     SetTransform {item_id: item_id,scene: &scene_name, transform:t
    //     position: Some(Position {
    //         x: Some(0),
    //         y: Some(0),
    //     }),
    //     rotation: Some(0),
    //     scale: Some(Scale {
    //         x: Some(1),
    //         y: Some(1),
    //     }),
    //     alignment: Some(t.alignment),
    //     bounds: Some(Bounds {
    //         r#type: Some(t.bounds_type),
    //         alignment: Some(t.bounds_alignment),
    //         width: Some(t.bounds_width),
    //         height: Some(t.bounds_height),
    //     }),
    //     crop: Some(Crop {
    //         left: Some(t.crop_left),
    //         right: Some(t.crop_right),
    //         top: Some(t.crop_top),
    //         bottom: Some(t.crop_bottom),
    //     }),
    // }
     }).await?;
    },
}
let mut dd=dbg!(dd);
match dd {
//Some(ref mut x)=>x.settings["input"] = json!(input),
Some(ref mut x)=>x.settings["input"] = json!("dave"),
None=>{}
}
println!("dude: {:?}",&dd);
let s=client.inputs().set_settings(SetSettings{input:input,settings:&dd,overlay:Some(false)}).await;
//client.inputs().remove(source).await.ok();
    Ok(())
}
