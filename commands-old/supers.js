var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'supers',
	description: 'List all superusers.',
	execute(message, args, client) {
        
        let str = "List of super users:"
        
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection("superusers").find({}, (err, result) => {
                if(err) throw err;
                result.forEach(document => {
                    if(err) throw err;
                    client.users.fetch(document.userid).then((user) => {
                        str += "\n" + user.username + "#" + user.discriminator;
                    });
                })
            })
        })

        setTimeout(() => {
            str += "\nand the bot author and host, nekeki#7777."
            message.channel.send(str);
        }, 1000)
        

	},
};