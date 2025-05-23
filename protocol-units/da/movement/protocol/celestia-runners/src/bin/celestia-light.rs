use godfig::{backend::config_file::ConfigFile, Godfig};
use movement_celestia_da_light_node_runners::{celestia_light::CelestiaLight, Runner};
use movement_da_util::CelestiaDaLightNodeConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	use tracing_subscriber::EnvFilter;

	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	let dot_movement = dot_movement::DotMovement::try_from_env()?;
	let config_file = dot_movement.try_get_or_create_config_file().await?;

	// get a matching godfig object
	let godfig: Godfig<CelestiaDaLightNodeConfig, ConfigFile> =
		Godfig::new(ConfigFile::new(config_file), vec![]);
	let config = godfig.try_wait_for_ready().await?;

	let celestia_light = CelestiaLight {};
	celestia_light.run(dot_movement, config).await?;

	Ok(())
}
