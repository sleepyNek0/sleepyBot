use serenity::{
    model::{
        channel::{Reaction, ReactionType},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
};

pub struct Handler;

impl EventHandler for Handler {
    /*
    fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member){

    }

    fn guild_member_removal(&self, _ctx: Context, _guild: GuildId, _user: User, _member_data_if_available: Option<Member>)
    */





    // prints a message to the console when ready
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}