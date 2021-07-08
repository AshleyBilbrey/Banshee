let broadcaster = require("../util/bc.js")
let is = require("../util/isSuper.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'broadcast',
	description: 'Broadcast a message to all enrolled users.',
	execute(message, args, client) {
        
        if(is.isSuper(message.author) || message.author.id == process.env.BOT_OWNER) {
            let str = message.content.slice(2 + process.env.PREFIX.length + this.name.length);
            str += "\nThe person who made this broadcast was **" + message.author.username + "#" + message.author.discriminator + "**."
            broadcaster.bc(str, client);
            message.channel.send("Broadcast sent!");
        } else {
            message.channel.send("You must be a superuser to run this command.");
        }

	},
};