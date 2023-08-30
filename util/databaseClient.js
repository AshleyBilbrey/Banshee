import { MongoClient, ServerApiVersion } from 'mongodb'

class DatabaseClient {

    constructor() {
        this.client = new MongoClient("mongodb://localhost:27017/", {
            serverApi: {
                version: ServerApiVersion.v1,
                strict: true,
                deprecationErrors: true
            }
        })

        this.clientConnect()
    }

    async clientConnect() {
        await this.client.connect()
        await this.client.db("admin").command({ ping: 1 });
    }

    async printInfo() {
        console.log("This is an instance of the DatabaseClient class.");
    }
}

export { DatabaseClient }