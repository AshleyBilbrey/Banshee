require('dotenv').config({ path: '../' })

module.exports = {
	name: 'help',
	description: 'help',
	execute(message, args, client) {
		message.channel.send("Hello, my name is Banshee!\n\nI will automically start banning new spammers. To ban the list of previous spammers, use " + process.env.PREFIX + " refresh. This should usually be run the first time you add me to your server.\nYou may unban any user you wish and they will not be rebanned unless you use the refresh command.\n\nWe try to all of these bot mechanics as transparent as possible.\nTo view users who have permission to add new users to the ban list, use " + process.env.PREFIX + " supers\nEnroll in ban and transparency notifications by using " + process.env.PREFIX + " enroll\nView the source code and the link to add the bot to your server here. https://github.com/AshleyBilbrey/Banshee\n\nPlease help me bonk spammers! Report spammers to the admins of the Directory server.\nhttps://discord.gg/ucf44wN\nIt would be helpful to have the spammer's userid, which you can turn on in developer settings, then right click on their name and then click 'Copy ID'. Thank you!")
	},
};