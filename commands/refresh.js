var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports = {
	name: 'refresh',
	description: 'This will refresh all banned users. You should run this the first time you add the bot to your server. This will also undo all previous unbans of users on the list.',
	execute(message, args, client) {
        if(message.member.hasPermission("BAN_MEMBERS")) {
            MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
                if(err) throw err;
                var dbo = db.db("banshee");
                dbo.collection("bans").find({}, (err, result) => {
                    if(err) throw err
                    result.forEach(document => {
                        client.users.fetch(document.userid).then(user => {
                            message.guild.members.ban(user, { reason: "Banned by Banshee"})
                        })
                    })
                    message.channel.send("Refreshed bans in this server!")
                })
            })
        } else {
            message.channel.send("You need to have the server permission to ban users in order to use this command.")
        }
        
	},
};