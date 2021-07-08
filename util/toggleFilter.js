var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports.toggle = function(str, callback) {
    MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
        if(err) throw err;
        var dbo = db.db("banshee");
        dbo.collection("filter").findOne({ element: str }, (err, result) => {
            if(err) throw err;
            if(result) {
                dbo.collection("filter").deleteOne({ element: str }, (err, result) => {
                    if(err) throw err;
                    callback(false);
                })
            } else {
                dbo.collection("filter").insertOne({
                    element: str,
                    isThing: true
                }, (err, res) => {
                    if(err) throw err;
                    callback(true);
                })
            }
        })
    })
}