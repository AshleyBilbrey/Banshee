var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports = {
    getTime(collection, cb) {
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection(collection).findOne({$query:{},$orderby:{time:-1}}, (err, result) => {
                if(err) throw err;
                if(result.time) {
                    cb(result.time)
                } else {
                    cb(0);
                }
            })
        })
    }
}