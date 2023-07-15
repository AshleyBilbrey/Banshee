let is = require("../util/isSuper.js")
let tdb = require("../util/toggleFilter.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'filter',
	description: 'Add a link or word to the filter.',
	execute(message, args, client) {

        let iscb = function(isSuper) {
            if(isSuper) {
                if(args.length >= 1) {
                    let str = message.content.slice(2 + process.env.PREFIX.length + 6);
                    let cb = function(added) {
                        if(added) {
                            message.channel.send("Added that to the filter!");
                        } else {
                            message.channel.send("Removed that from the filter!");
                        }
                    }
    
                    tdb.toggle(str, cb)
                } else {
                    message.channel.send("You need to include something to add to the filter.")
                }
                
            } else {
                message.channel.send("You must be a superuser to run this command.");
            }
        }

        is.isSuper(message.author, iscb)

	},
};