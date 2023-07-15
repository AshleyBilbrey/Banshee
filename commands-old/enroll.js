let tdb = require("../util/toggleDB.js")
require('dotenv').config({ path: '../' })

module.exports = {
	name: 'enroll',
	description: 'Enroll a user into bot broadcasts.',
	execute(message, args, client) {
        let cb = function(added) {
            if(added) {
                message.channel.send("Enrolled you into broadcasts. Please remember, you must share a server with me for me to message you!");
            } else {
                message.channel.send("Removed you from broadcasts.");
            }
        }
		
        tdb.toggle(message.author, "enrollees", cb)

	},
};