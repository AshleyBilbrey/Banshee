var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";
let is = require("../util/isSuper.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'filterlist',
	description: 'List of all strings in filter.',
	execute(message, args, client) {
        if(is.isSuper(message.author) || message.author.id == process.env.BOT_OWNER) {

            let str = "List of filter elements:"
        
            MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
                if(err) throw err;
                var dbo = db.db("banshee");
                dbo.collection("filter").find({}, (err, result) => {
                    if(err) throw err;
                    result.forEach(document => {
                        if(err) throw err;
                        str += "\n\n" + document.element;
                    })
                })
            })

            setTimeout(() => {
                message.channel.send(str);
            }, 1000)

        } else {
            message.channel.send("You must be a superuser to run this command.");
        }

	},
};