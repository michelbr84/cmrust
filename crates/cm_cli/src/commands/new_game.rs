//! New game command.

use chrono::NaiveDate;

use cm_core::ids::ClubId;
use cm_data::import::json_importer::JsonWorldImporter;
use cm_engine::config::GameConfig;
use cm_engine::game::Game;
use cm_engine::state::GameState;
use cm_save::snapshot::{GameConfigData, GameStateData, SaveSnapshot};

#[derive(clap::Args, Debug)]
pub struct NewGameArgs {
    /// Directory containing JSON data files
    #[arg(long, default_value = "assets/data")]
    pub data_dir: String,

    /// Output save file path
    #[arg(long, default_value = "saves/slot1.cmsave")]
    pub out: String,

    /// Start date (YYYY-MM-DD)
    #[arg(long, default_value = "2001-07-01")]
    pub start_date: String,

    /// Club ID to manage
    #[arg(long, default_value = "LIV")]
    pub club: String,

    /// Manager name
    #[arg(long, default_value = "Manager")]
    pub manager: String,
}

pub fn run(args: NewGameArgs) -> anyhow::Result<()> {
    let start_date = NaiveDate::parse_from_str(&args.start_date, "%Y-%m-%d")?;

    println!("Loading world data from: {}", args.data_dir);
    let importer = JsonWorldImporter::new(&args.data_dir);
    let world = importer.load_world()?;

    println!("Creating new game...");
    println!("  Manager: {}", args.manager);
    println!("  Club: {}", args.club);
    println!("  Start date: {}", start_date);

    let cfg = GameConfig::default();
    let state = GameState::new(start_date, args.manager.clone(), ClubId::new(&args.club));

    let mut game = Game::new(cfg.clone(), world.clone(), state.clone());
    game.bootstrap_inbox();

    // Create save data
    let config_data = GameConfigData {
        difficulty: cfg.difficulty,
        auto_save: cfg.auto_save,
    };

    let state_data = GameStateData {
        date: start_date.to_string(),
        manager_name: args.manager,
        club_id: args.club,
        inbox: game.state().inbox.clone(),
    };

    let snapshot = SaveSnapshot::new(world, config_data, state_data)?;
    snapshot.write_to_file(&args.out)?;

    println!("✓ New game created: {}", args.out);
    println!("  {} clubs loaded", game.world().clubs.len());
    println!("  {} players loaded", game.world().players.len());

    Ok(())
}
