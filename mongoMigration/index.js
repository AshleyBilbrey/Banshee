const { MongoClient } = require('mongodb');
const { Pool } = require('pg');

(async () => {
  const mongoClient = new MongoClient('mongodb://localhost:27017/', {
    useNewUrlParser: true,
    useUnifiedTopology: true
  });
  await mongoClient.connect();
  const db = mongoClient.db('banshee');
  const bansCollection = db.collection('bans');

  const pgPool = new Pool({
    connectionString: 'postgres://postgres:password@database/banshee'
  });

  try {
    const bans = bansCollection.find({});
    while (await bans.hasNext()) {
      const currentBan = await bans.next();
      const { userid, time } = currentBan;

      const snowflake = userid;
      const banned = true;
      const ban_reason = 'legacy';
      const super_user = false;
      const createdAt = new Date(time);
      const updatedAt = createdAt;

      const query = `
        INSERT INTO banshee
          (snowflake, banned, ban_reason, super_user, created_at, updated_at)
        VALUES
          ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (snowflake) DO NOTHING
      `;
      await pgPool.query(query, [
        snowflake,
        banned,
        ban_reason,
        super_user,
        createdAt,
        updatedAt
      ]);
    }

    console.log('Migration complete.');
  } catch (err) {
    console.error('Error during migration:', err);
  } finally {
    await pgPool.end();
    await mongoClient.close();
  }
})();
