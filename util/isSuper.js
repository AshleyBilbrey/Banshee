var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports = {
    isSuper(user) {
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection("superusers").findOne({ userid: user.id }, (err, result) => {
                if(err) throw err;
                if(result) return true;
                return false;
            })
        })
    }
}