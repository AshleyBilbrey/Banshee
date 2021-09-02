require('dotenv').config({ path: '../' })
let is = require("../util/isSuper.js")

module.exports = {
	name: 'help',
	description: 'help',
	execute(message, args, client) {
		message.channel.send("__**Banshee**__\n\nBanshee will ban known spammers for you automatically!. For more bot information, how to add the bot to your own server, reporting a spammer, or help, please join this server. https://discord.gg/b8h9aKsGrT\n\n**Commands**\n\n- !b enroll - Allows you to enroll/unenroll in transparency broadcasts. Banshee will DM you when someone new is banned, when there are changes to the super user list, and more.- !b help - View information about Banshee.\n- !b refresh - For server owners to add all existing users from Banshee's ban list to the server bans.\n- !b supers - View the list of current super users.\n💖")
		let iscb = function(isSuper) {
			if(isSuper) {

				message.channel.send("**Super User Commands**\n\n- !b ban :user: :reason: - Accepts @ or user ID. This will toggle a ban of a user, which will propagate to all servers Banshee is in. Reasons will be added to server ban reasons. Bans will be broadcasted to enrolled users.\n- !b broadcast :message: - Have Banshee broadcast a message to all enrolled users.\n- !b filter :word: - Add a word to a global filter. Typically for server join links of known spammers.\n- !b filterlist - List all filtered words. Be careful where you use this command as it may be spammy.\n- !b super :user: - Toggle's super user status on user. This sends a broadcast for transparency.\n\n")

			}
		}

		is.isSuper(message.author, iscb)
	},
};