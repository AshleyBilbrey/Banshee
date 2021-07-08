# Banshee
A Discord bot that will automatically ban known spammers from servers.

To add this bot to your server, please use this link.
https://discord.com/api/oauth2/authorize?client_id=862520009770401794&scope=bot&permissions=10244
Please ensure the bot has the correct permissions to do its job.
The first time you add the bot to the server, it's a good idea to use !b refresh to ban all the existing people on the list. Any new users added to the ban list will be added automatically.

The bot is designed to be as transparent as possible about the actions it is taking. We do this in a few ways.
- The code is open source here, so you can look at the code and see how the bot is supposed to behave.
- All the users who can add bans and edit the filter, superusers, are listed with !b supers.
- You can enroll in messages that will notify you of any changes to the ban list and superuser list, and let you know who initiated that action.
- Banshee does not constantly enforce its ban list, meaning if you would like to unban a specific user on the ban list for your server you can, and Banshee will not reban that user unless you use !b refresh.
- Similarly, if you banned a user on your server and that user was removed from the Banshee ban list, Banshee will not unban that user because it was not originally banned by Banshee.

You can use !b help for more information about the bot.

Please remember to report any spammers to the admininstrators of the directory server https://discord.gg/ucf44wN.
It would be helpful to have the spammer's userid, which you can turn on in developer settings, then right click on their name and then click 'Copy ID'. Thank you!
