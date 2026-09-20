use std::future;

use tracing::info;
use zbus::conn::Builder;

use crate::file_chooser::FileChooser;

mod file_chooser;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chooser = FileChooser {};
    let conn = Builder::session()?
        .name("org.freedesktop.impl.portal.desktop.nvim")?
        .serve_at("/org/freedesktop/portal/desktop", chooser)?
        .build()
        .await?;
    info!("Started portal on {}",
        conn.unique_name().map_or("no unique_name", |name| name.as_str()));
    // Just wait forever lol
    future::pending::<()>().await;
    Ok(())
}
