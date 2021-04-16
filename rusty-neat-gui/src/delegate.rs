use std::sync::Arc;

use druid::{AppDelegate, Handled};

use crate::{commands, model::GUIModel, reader};

pub struct Delegate;

impl AppDelegate<GUIModel> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut GUIModel,
        _env: &druid::Env,
    ) -> druid::Handled {

        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
          let result = reader::read(&file_info.path());
          if let Ok(neat_model) = result {
            data.neat = Some(neat_model);
          } else if let Err(err) = result {
            println!("{}", err);
          }
          return Handled::Yes;

        } else if let Some(generation) = cmd.get(commands::SELECT_GENERATION) {
          if data.neat.is_some() {
            data.neat.as_mut().unwrap().current_generation = Some(Arc::clone(generation));
            data.neat.as_mut().unwrap().current_species = None;
            data.neat.as_mut().unwrap().current_genome = None;
            return Handled::Yes;
          }

        } else if let Some(species) = cmd.get(commands::SELECT_SPECIES) {
          if data.neat.is_some() {
            data.neat.as_mut().unwrap().current_species = Some(Arc::clone(species));
            data.neat.as_mut().unwrap().current_genome = None;
            return Handled::Yes;
          }

        } else if let Some(genome) = cmd.get(commands::SELECT_GENOME) {
          if data.neat.is_some() {
            data.neat.as_mut().unwrap().current_genome = Some(Arc::clone(genome));
            return Handled::Yes;
          }
        };

        Handled::No
    }
}