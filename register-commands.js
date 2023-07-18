import { readdirSync } from 'fs';
import path from 'path'
import 'dotenv/config'
import { REST } from '@discordjs/rest';
import { ApplicationCommandsAPI } from '@discordjs/core';

const token = process.env.BOT_TOKEN;

const rest = new REST({ version: '10' }).setToken(token);
const commandApi = new ApplicationCommandsAPI(rest);

const __dirname = String(import.meta.url).slice(0, -("register-commands.js".length)).slice("file:".length);
console.log(__dirname);

const commandPath = path.join(__dirname, "commands")
const commandFiles = readdirSync(commandPath).filter(fileName => fileName.endsWith(".js"));

const commands = [];

for (const file of commandFiles) {
    const filePath = path.join(commandPath, file);
    let command = await(import(filePath));

    if("data" in command && "execute" in command) {
        commands.push(command.data.toJSON());
    } else {
        console.log(command);
        console.log("WARNING: Command %s incorrectly formatted.", file);
    }
}

(async () => {
    console.log("Hello, world!")
    try {
        console.log("Registering commands with discord.");

        let result = await commandApi.bulkOverwriteGuildCommands(process.env.CLIENT_ID, process.env.GUILD_ID, commands);
        console.log(result)

        console.log("Finished.");
    } catch(e) {
        console.log("There was an error.")
        console.log(e)
    }
})();