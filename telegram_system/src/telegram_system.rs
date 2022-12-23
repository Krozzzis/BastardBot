use std::sync::Arc;
use timetable_system::*;

use teloxide::{
    dispatching::dialogue::{
        InMemStorage,
    },
    prelude::*,
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<BotState, InMemStorage<BotState>>;
type MyStorage = Arc<InMemStorage<BotState>>;
type MyError = Box<dyn std::error::Error + Send + Sync>;
type HandlerResult = Result<(), MyError>;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum IdleCommands {
    #[command(description = "Commands")]
    Help,
    #[command(description = "Понедельник")]
    Monday,
    #[command(description = "Вторник")]
    Tuesday,
    #[command(description = "Среда")]
    Wednesday,
    #[command(description = "Четверг")]
    Thursday,
    #[command(description = "Пятница")]
    Friday,
    #[command(description = "Суббота")]
    Saturday,

    #[command(description = "Сегодня")]
    Today,
    #[command(description = "Завтра")]
    Tomorrow,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
enum BotState {
    #[default]
    Idle,
}

pub struct TelegramSystem {
    timetable: Arc<TimeTableProvider>,
}

impl TelegramSystem {
    pub fn new(timetable_provider: TimeTableProvider) -> Self {
        Self {
            timetable: Arc::new(timetable_provider),
        }
    }

    pub async fn run(&self, token: String) {
        let bot = Bot::new(token);
        bot.set_my_commands(IdleCommands::bot_commands()).await;
        
        let storage: MyStorage = InMemStorage::new();
        let handler = Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<BotState>, BotState>()
            .branch(
                dptree::case![BotState::Idle]
                    .branch(dptree::entry().filter_command::<IdleCommands>().endpoint(Self::idle_command_handler)),
            );

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![storage, self.timetable.clone()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    async fn idle_command_handler(
        bot: Bot,
        dialogue: MyDialogue,
        msg: Message,
        provider: Arc<TimeTableProvider>,
        cmd: IdleCommands,
    ) -> HandlerResult {
        let mut day = Day::WeekDay(WeekDay::Monday);
        match cmd {
            IdleCommands::Monday => day = Day::WeekDay(WeekDay::Monday),
            IdleCommands::Tuesday => day = Day::WeekDay(WeekDay::Tuesday),
            IdleCommands::Wednesday => day = Day::WeekDay(WeekDay::Wednesday),
            IdleCommands::Thursday => day = Day::WeekDay(WeekDay::Thursday),
            IdleCommands::Friday => day = Day::WeekDay(WeekDay::Friday),
            IdleCommands::Saturday => day = Day::WeekDay(WeekDay::Saturday),
            _ => {},
        };
        match cmd {
            IdleCommands::Help => bot.send_message(msg.chat.id, IdleCommands::descriptions().to_string()).await?,
            _ => {
                let tt = match provider.get_formatted(day).await {
                    Ok(x) => x,
                    Err(err) => format!("Error: {}", err.to_string()),
                };

                bot.send_message(msg.chat.id, tt).await?
            },
        };
        Ok(())
    }
}
