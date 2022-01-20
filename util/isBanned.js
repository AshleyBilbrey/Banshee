var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";
require('dotenv').config({ path: '../' })

module.exports = {
    isBanned(user, cb) {
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection("bans").findOne({ userid: user.id }, (err, result) => {
                if(err) throw err;
                if(result) {
                    cb(true)
                } else {
                    cb(false)
                }
                
            })
        })
    }
}