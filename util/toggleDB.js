var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports.toggle = function(user, collection, callback) {
    MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
        if(err) throw err;
        var dbo = db.db("banshee");
        dbo.collection(collection).findOne({ userid: user.id }, (err, result) => {
            if(err) throw err;
            if(result) {
                dbo.collection(collection).deleteOne({ userid: user.id }, (err, result) => {
                    if(err) throw err;
                    callback(false);
                })
            } else {
                dbo.collection(collection).insertOne({
                    userid: user.id,
                    isThing: true
                }, (err, res) => {
                    if(err) throw err;
                    callback(true);
                })
            }
        })
    })
}