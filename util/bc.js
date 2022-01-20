var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports = {
    bc(msg, client) {
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection("enrollees").find({}, (err, result) => {
                if(err) throw err;
                result.forEach(document => {
                    if(err) throw err;
                    client.users.fetch(document.userid).then((user) => {
                        user.send(msg).catch((err) => {
                            console.log(err)
                            console.log("Unable to send broadcast to user " + document.userid)
                        });
                    });
                })
            })
        })
    }
}