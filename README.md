# Banshee
Banshee will ban known spammers for you automatically!  
  
Banshee was developed to help deal with the problem of individuals mass joining Discord servers in the UC Davis community and spamming academic dishonesty or other commercial services. 

Add Banshee bot to your server to help limit the amount of spam your server receives. Use !b refresh to add the existing ban list to your server bans, and any new spammers will automatically be added to your server ban list.

We understand that allowing a bot to have ban permissions may be concerning, so Banshee bot has been built with a high level of transparency.
- Significant bot actions, such as adding a user to the ban list, or a new super user, are broadcasted. To receive transparency messages user !b enroll.
- Only super users can add users to the ban list. These are trusted users. You can view the list of super users with !b supers.
- Code is open source. You can view the code here. https://github.com/AshleyBilbrey/Banshee
- Banshee is a passive system, meaning it will not actively enforce the ban list. Banshee only automatically bans a user when they are added to the list, therefore you can unban a user and Banshee will not reban that user unless you use !b refresh. Using !b refresh will load the existing ban list.
- Super users have a banning cooldown to prevent abuse.
- All else fails, Banshee can just be kicked from the server to prevent it from making any other actions.

Banshee bot was made in association with the the UCD Directory Server and Cyber Security Club at UC Davis. More information about them in the links below.
- Directory: https://discord.gg/ucf44wN
- Cyber Security Club at UC Davis Server: https://discord.gg/y6K82wk6en
- Cyber Security Club at UC Davis Website: https://daviscybersec.org/

## Add

To add this bot to your server, please use this link.  
https://discord.com/api/oauth2/authorize?client_id=862520009770401794&scope=bot&permissions=10244
Please ensure the bot has the correct permissions to do its job.  
The first time you add the bot to the server, it's a good idea to use !b refresh to ban all the existing people on the list. Any new users added to the ban list will be added automatically.

## Commands

- !b enroll - Allows you to enroll/unenroll in transparency broadcasts. Banshee will DM you when someone new is banned, when there are changes to the super user list, and more.
- !b help - View information about Banshee.
- !b refresh - For server owners to add all existing users from Banshee's ban list to the server bans.
- !b supers - View the list of current super users.

Only for super users:

- !b ban :user: :reason: - Accepts @ or user ID. This will toggle a ban of a user, which will propagate to all servers Banshee is in. Reasons will be added to server ban reasons. Bans will be broadcasted to enrolled users.
- !b broadcast :message: - Have Banshee broadcast a message to all enrolled users.
- !b filter :word: - Add a word to a global filter. Typically for server join links of known spammers.
- !b filterlist - List all filtered words. Be careful where you use this command as it may be spammy.
- !b super :user: - Toggle's super user status on user. This sends a broadcast for transparency.

## Report

Please remember to report any spammers to a super user on the Banshee Bot server https://discord.gg/b8h9aKsGrT.
It would be helpful to have the spammer's userid, which you can turn on in developer settings, then right click on their name and then click 'Copy ID'. Thank you!
