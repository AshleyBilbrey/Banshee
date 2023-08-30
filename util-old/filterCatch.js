var MongoClient = require('mongodb').MongoClient;
var url = "mongodb://localhost:27017/";

module.exports = {
    catch(str, cb) {
        MongoClient.connect(url, { useNewUrlParser: true, useUnifiedTopology: true }, (err, db) => {
            if(err) throw err;
            var dbo = db.db("banshee");
            dbo.collection("filter").find({}, (err, result) => {
                if(err) throw err;
                result.forEach(document => {
                    if(str.includes(document.element)) {
                        cb();
                    }
                })
            })
        })
    }
}