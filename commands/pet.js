import { MessageFlags } from '@discordjs/core'
import { SlashCommandBuilder } from '@discordjs/builders'

const data = new SlashCommandBuilder()
        .setName("pet")
        .setDescription("Pet the Banshee Bot")
const execute = async function (interaction, api) {
        api.interactions.reply(interaction.id, interaction.token, { content: "What am I to you, a cow to pet?\nhttps://tenor.com/view/chika-fujiwara-hit-cute-kawaii-anime-gif-13583613", flags: MessageFlags.Ephemeral });
    }

export { data, execute }