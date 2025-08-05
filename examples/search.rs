use futures::prelude::*;
use ssdp_client::{SearchTarget, URN};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), ssdp_client::Error> {
    println!("Searching for devices...");
    // let search_target = URN::device("schemas-upnp-org", "%", 1).into();
    let search_target = SearchTarget::RootDevice;
    let timeout = Duration::from_secs(3);
    let mut responses =
        ssdp_client::search_on_addr(&search_target, timeout, 2, Some([192, 168, 104, 1])).await?;

    while let Some(response) = responses.next().await {
        let response = response?;
        println!("- {}", response.search_target());
        println!("  - location: {}", response.location());
        println!("  - usn: {}", response.usn());
        println!("  - server: {}", response.server());
    }

    Ok(())
}
