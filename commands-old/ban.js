let is = require("../util/isSuper.js")
let broadcaster = require("../util/bc.js")
let tdb = require("../util/toggleDB.js")
let bn = require("../util/bonkNum.js")
let ot = require("../util/operationTime.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'ban',
	description: 'Ban a user on all servers Banshee is on.',
	execute(message, args, client) {

        let iscb = function(isSuper) {
            if(isSuper) {
                if(args.length >= 1) {
                    let str = args[0];
                    if(str.startsWith('<@') && str.endsWith('>')) {
                        str = str.slice(2, -1);
                        if (str.startsWith('!')) {
                            str = str.slice(1);
                        }
                    }
                    if(/^\d+$/.test(str)) {
                        client.users.fetch(str).then(newBan => {
                            if(newBan) {
                                let iscb2 = function(isSuper2) {
                                    if(isSuper2) {
                                        message.channel.send("You cannot ban a superuser.")
                                    } else {
                                        let cb2 = function(time) {
                                            let d = new Date();
                                            if(d.getTime() > (time + parseInt(process.env.COOLDOWN))) {
        
                                                let cb = function(added) {
                                                    if(added) {
                                                        let banReason = ""
                                                        for(let i = 1; i < args.length; i++) {
                                                            banReason += args[i] + " ";
                                                        }
                                                        message.channel.send("Added **" + newBan.username + "#" + newBan.discriminator + "** to the ban list.\nReason: " + banReason);
                                                        broadcaster.bc("Hello, **" + newBan.username + "#" + newBan.discriminator + "** was added to the ban list.\nReason: " + banReason + "\nYou may unban this user on your server and Banshee will not reban this user unless you use " + process.env.PREFIX + " refresh.\nThe person who made this action was **" + message.author.username + "#" + message.author.discriminator + "**.", client)
                                                        bn.setNum(client);
                                                        client.guilds.cache.forEach(guild => {
                                                            try {
                                                                guild.members.ban(newBan, { days: 1, reason: "Banned by Banshee - " + banReason})
                                                            } catch {
                                                                client.users.fetch(process.env.BOT_OWNER).then((user) => {
                                                                    user.send("There was an issue banning user " + newBan.username + "#" + newBan.discriminator + " in server " + guild.name);
                                                                    if(!guild.member(client.user).hasPermission('BAN_MEMBERS')) {
                                                                        user.send("It is because I do not have permission to ban in the server.");
                                                                    }
                                                                })
                                                            }
                                                        })
                                                    } else {
                                                        message.channel.send("Removed **" + newBan.username + "#" + newBan.discriminator + "** from the ban list.");
                                                        broadcaster.bc("Hello, **" + newBan.username + "#" + newBan.discriminator + "** has been removed from the ban list.\nIf you banned this user separately then Banshee did not unban this user.\nThe person who made this action was **" + message.author.username + "#" + message.author.discriminator + "**.", client)
                                                        bn.setNum(client);
                                                        client.guilds.cache.forEach(guild => {
                                                            guild.fetchBan(newBan).then(binfo => {
                                                                if(binfo && binfo.reason.includes("Banned by Banshee")) {
                                                                    guild.members.unban(newBan, "Unbanned by Banshee");
                                                                }
                                                            })
                                                        })
                                                    }
                                                }
                                                tdb.toggle(newBan, "bans", cb)
        
                                            } else {
                                                message.channel.send("Sorry, banning is on a cooldown.");
                                            }
                                        }
        
                                        ot.getTime("bans", cb2);
        
                                    }
                                }
                                
                                is.isSuper(newBan, iscb2)


            
                            } else {
                                message.channel.send("That is not a valid user.")
                            }
                        })
                    } else {
                        message.channel.send("Don't try and break me!");
                        message.channel.send("https://tenor.com/view/chika-fujiwara-hit-cute-kawaii-anime-gif-13583613");
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