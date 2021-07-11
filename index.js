const Discord = require('discord.js');
const fs = require("fs");
const bn = require("./util/bonkNum.js")
const fc = require("./util/filterCatch.js")
const is = require("./util/isSuper.js")
require('dotenv').config();

const client = new Discord.Client();
client.commands = new Discord.Collection();

const prefix = process.env.PREFIX;

const commandFiles = fs.readdirSync('./commands').filter(file => file.endsWith('.js'));

for (const file of commandFiles) {
	const command = require(`./commands/${file}`);
	client.commands.set(command.name, command);
}

client.on('message', message => {

	if (!message.content.startsWith(prefix) || message.author.bot) {
		let iscb = function(isSuper) {
			if(!isSuper) {
				let cb = function() {
					message.delete();
					message.channel.send("Hi **" + message.author.username + "#" + message.author.discriminator + "**! Something in your message is in our spam filter. If you beleive this is a mistake, please contact one of the " + process.env.PREFIX + " supers. Thanks!").then(reply => {
						setTimeout(() => {
							reply.delete();
						}, 5000);
					})
	
				}
				fc.catch(message.content, cb);
			}
		}

		is.isSuper(message.author, iscb)
		
		
	} else {
		const args = message.content.slice(prefix.length).trim().split(/ +/);
		const command = args.shift().toLowerCase();

		if (!client.commands.has(command)) return;

		try {
			client.commands.get(command).execute(message, args, client);
		} catch (error) {
			console.error(error);
			message.reply('Sorry, there was an issue trying to run that command!');
		}
	}

	
});


client.on("ready", () => {
    console.log(`Banshee logged in as ${client.user.tag}`);
    bn.setNum(client);
});

client.login(process.env.BOT_TOKEN);
