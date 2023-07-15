let ib = require("../util/isBanned.js")
let is = require("../util/isSuper.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'userinfo',
	description: 'Provides info about a user.',
	execute(message, args, client) {
        if(args.length >= 1) {
            let str = args[0];
            if(str.startsWith('<@') && str.endsWith('>')) {
                str = str.slice(2, -1);
                if (str.startsWith('!')) {
                    str = str.slice(1);
                }
            }
            if(/^\d+$/.test(str)) {
                        
                userinfostr = "**User info for __" + str + "__:**\n"

                let iscb = function(isSuper) {
                    userinfostr += "Super User: " + isSuper + "\n"

                    let ibcb = function(isBanned) {
                        userinfostr += "Banned: " + isBanned

                        message.channel.send(userinfostr)
                    }

                    ib.isBanned({id: str}, ibcb)

                }

                is.isSuper({id: str}, iscb)

            } else {
                message.channel.send("Don't try and break me!");
                message.channel.send("https://tenor.com/view/chika-fujiwara-hit-cute-kawaii-anime-gif-13583613");
            }
                    
        } else {
            message.channel.send("You must include a user.");
        }


	},
};