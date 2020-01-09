#[macro_use]
extern crate log;
extern crate platter;

// Example 3: The RGB Triangle
// Open a window, and draw the standard GPU triangle
use platter::load_file;
use quicksilver::{
    geom::Vector,
    graphics::{Color, Graphics},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};
use std::future::Future;
use std::string::String;

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Custom Events Example",
            ..Settings::default()
        },
        app,
    );
}

#[cfg(target_arch = "wasm32")]
async fn echo_request() -> Result<String> {
    let url = "https://postman-echo.com/get?foo1=bar1";
    let response = load_file(url).await?;
    Ok(format!("Response payload: {} bytes", response.len()))
}

async fn handle_all_ready_quicksilver_events(events: &mut EventStream) -> Result<()> {
    while let Some(ev) = events.next_event().await {
        info!("Got event: {:?}", ev)
    }
    Ok(())
}

async fn app(_window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Clear the screen to a blank, black color
    gfx.clear(Color::BLACK);

    let echo = Box::new(echo_request());

    let mut pending: Vec<Box<dyn Future<Output = Result<String>>>> = vec![echo];

    let _ = pending.pop(); // make it empty for now

    loop {
        if pending.is_empty() {
            handle_all_ready_quicksilver_events(&mut events).await?;
        } else {
            // let select_pending = select_all(pending.iter());
            // select! {
                // 
            // }
        }
    }
}
