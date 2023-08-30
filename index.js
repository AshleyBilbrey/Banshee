import { readdirSync } from 'fs';
import path from 'path'
import 'dotenv/config'
import { REST } from '@discordjs/rest';
import { WebSocketManager } from '@discordjs/ws';
import { GatewayDispatchEvents, GatewayIntentBits, InteractionType, MessageFlags, Client } from '@discordjs/core';
import { Collection } from '@discordjs/collection'
import { DatabaseClient } from './util/databaseClient.js';

const token = process.env.BOT_TOKEN;

const rest = new REST({ version: '10' }).setToken(token);

const gateway = new WebSocketManager({
    token,
    intents: GatewayIntentBits.DirectMessages,
    rest,
});

const client = new Client({ rest, gateway });

client.commands = new Collection();

const __dirname = String(import.meta.url).slice(0, -("index.js".length)).slice("file:".length);

const commandPath = path.join(__dirname, "commands")
const commandFiles = readdirSync(commandPath).filter(fileName => fileName.endsWith(".js"));

for (const file of commandFiles) {
    const filePath = path.join(commandPath, file);
    let command = await (import(filePath));

    if ("data" in command && "execute" in command) {
        client.commands.set(command.data.name, command)
    } else {
        console.log(command);
        console.log("WARNING: Command %s incorrectly formatted.", file);
    }
}

client.on(GatewayDispatchEvents.InteractionCreate, async ({ data: interaction, api }) => {
    console.log("LOG: %s (%s) ran command %s", interaction.member.user.username, interaction.member.user.id, interaction.data.name);
    let currentCommand = client.commands.get(interaction.data.name);
    if (InteractionType.ApplicationCommand && currentCommand) {
        currentCommand.execute(interaction, api);
    } else {
        api.interactions.reply(interaction.id, interaction.token, { content: "Hmm, for some reason I don't know what to do with this command. Please contact a dev.", flags: MessageFlags.Ephemeral });
    }
});

client.once(GatewayDispatchEvents.Ready, () => {
    console.log('Ready!');
    let d = new DatabaseClient();
    d.printInfo();
});

gateway.connect();