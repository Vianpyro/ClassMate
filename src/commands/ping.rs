use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Replies with Pong!")
}

pub async fn execute(ctx: &Context, interaction: &CommandInteraction) {
    let response = CreateInteractionResponseMessage::new().content("Pong!").ephemeral(true);
    let response = CreateInteractionResponse::Message(response);

    let message = interaction.create_response(&ctx.http, response).await;

    if let Err(why) = message {
        println!("Error responding to ping command: {why:?}");
    }
}
