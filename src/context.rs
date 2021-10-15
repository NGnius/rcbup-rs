use std::convert::From;

use libfj::robocraft::{FactoryAPI, RoboShopItemsInfo, FactoryTextSearchType, FactoryRobotListInfo, FactoryRobotGetInfo};

use crate::args::CliArguments;

pub struct Context {
    args: CliArguments,
    api: FactoryAPI,
}

impl Context {
    pub async fn get_search(&self) -> Result<RoboShopItemsInfo, ()> {
        let mut builder = self.api.list_builder();
        if let Some(search) = &self.args.search {
            builder = builder.text(search.to_string());
        }
        if self.args.player {
            builder = builder.text_search_type(FactoryTextSearchType::Player);
        }
        match builder.send().await {
            Err(e) => {
                eprintln!("Factory search failed: {}", e);
                Err(())
            },
            Ok(result) => Ok(result.response)
        }
    }
    
    pub async fn get_extra_info(&self, bot: &FactoryRobotListInfo) -> Result<FactoryRobotGetInfo, ()> {
        match self.api.get(bot.item_id).await {
            Err(e) => {
                eprintln!("Factory bot retrieval failed: {}", e);
                Err(())
            },
            Ok(result) => Ok(result.response)
        }
    }
}

impl From<CliArguments> for Context {
    fn from(args: CliArguments) -> Self {
        Self {
            api: args.configure_api().unwrap(),
            args: args,
        }
    }
}
