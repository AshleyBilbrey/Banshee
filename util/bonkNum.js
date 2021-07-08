var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports = {
    setNum(client) {
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection("bans").countDocuments({}, (err, num) => {
                if(err) throw err;
                client.user.setActivity(num + " bonks!");
            })
        })
    }
}