let broadcaster = require("../util/bc.js")
let is = require("../util/isSuper.js")
let tdb = require("../util/toggleDB.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'super',
	description: 'Add a superuser.',
	execute(message, args, client) {

        let iscb = function(isSuper) {
            if(isSuper) {
                if(args.length >= 1) {
                    if(message.mentions.users.first()) {
                        let newSuper = message.mentions.users.first();
                        let cb = function(added) {
                            if(added) {
                                message.channel.send("Added **" + newSuper.username + "#" + newSuper.discriminator + "** as a new superuser.");
                                broadcaster.bc("Hello, this is a Banshee transparency message.\n**" + newSuper.username + "#" + newSuper.discriminator + "** has been added as a new superuser. This means they have the power to add users to the master banlist and add words to the filter.\nThe person who made this action was **" + message.author.username + "#" + message.author.discriminator + "**.", client)
                            } else {
                                message.channel.send("Removed **" + newSuper.username + "#" + newSuper.discriminator + "** as a superuser.");
                                broadcaster.bc("Hello, this is a Banshee transparency message.\n**" + newSuper.username + "#" + newSuper.discriminator + "** has been removed as a superuser.\nThe person who made this action was **" + message.author.username + "#" + message.author.discriminator + "**.", client)
                            }
                        }
                        tdb.toggle(newSuper, "superusers", cb)
                    } else {
                        message.channel.send("That is not a valid user.")
                    }
                } else {
                    message.channel.send("You must include a user.");
                }
            } else {
                message.channel.send("You must be a superuser to run this command.");
            }
        }

        is.isSuper(message.author, iscb)

	},
};