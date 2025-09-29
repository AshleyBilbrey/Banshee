<img src="https://www.ashleybilbrey.com/public/img/projects/banshee/banshee-discord-bot.png" alt="Stylized text of banshee">

# Banshee Bot

Banshee Bot is a Discord bot to remove spammers and scammers from UC Davis community servers. It's designed with features for ease of use, set and forget for server admins, and transparency in mind. It's currently used on over 100 servers.

UC Davis has a tight network of hundreds of Discord servers for classes, clubs, and other groups. With these servers so closely networked, it's very easy for bots that promote scams or academic dishonesty services to post on many of these servers, all at once. This requires that hundreds of individual server owners each ban the same account. Banshee Bot solves this problem by hosting a central reporting service, and maintaining a list of these accounts to be automatically removed. All a server owner needs to do is add the bot to their server, and Banshee will handle the rest.

Banshee was developed in the Rust programming language with the [Poise](https://github.com/serenity-rs/poise)/[Serenity](https://serenity-rs.github.io/) framework for Discord bots. The code is open-source and [available on GitHub](https://github.com/AshleyBilbrey/Banshee).

Banshee Bot's profile photo was made by [Xenvita](https://www.deviantart.com/xen1231/art/Glitch-2-593312378) and used non-commercially (thank you for the great art!). Banshee Bot is not affiliated with UC Davis. I'm just a grad who developed and maintains it.

# Links
<p>
	<a href="https://discord.com/api/oauth2/authorize?client_id=862520009770401794&scope=bot&permissions=10244">Add to Your Server</a> -
	<a href="https://discord.gg/2nmjrEFHxp">Join Community Server</a> -
	<a href="https://github.com/AshleyBilbrey/Banshee">Visit GitHub</a>
</p>

# Features
## Set and Forget
Add Banshee to your Discord server, and it will handle the rest! When a known scam or spam account joins your server, it will automatically be removed. It will also automatically remove newly added malicious accounts to the ban list.

## Easy to Use
Banshee Bot uses Discord's slash commands and context menu commands to create a seamless experience within Discord's UI. It's designed to be color coded and use rich embeds to make it easy to understand at a glance.

<img src="https://www.ashleybilbrey.com/public/img/projects/banshee/banshee-report-rich-embed.png" alt="Three overlapping screenshots of color coded Discord rich embeds for an open report, a dismissed report, and a banned report.">

## Transparency & Security
Banshee was designed with security and transparency in mind. The code is [open source](https://github.com/AshleyBilbrey/Banshee), so anyone can audit it to understand its behavior and potential vulnerabilities. You can always see which users are being removed from your Discord server by using Discord's built-in audit log feature. We also take the responsibility of administering Banshee seriously, so the list of our "super users" is always available. If you wish to prevent a user from being removed by Banshee on your server, you can add them to an allow list. Additionally, by the [principle of least privilege](https://en.wikipedia.org/wiki/Principle_of_least_privilege), we only ask for the permissions that are necessary to operate the bot.

<img src="https://www.ashleybilbrey.com/public/img/projects/banshee/banshee-list-super-users.png" alt="The use of a Discord slash command 'supers', where Banshee Bot responds with a list of three super users.">

# Commands
## Reporting

You can easily report spammers and scammers by opening the context menu on the offending message, going to "Apps", then clicking on "Report to Banshee".

<img src="https://www.ashleybilbrey.com/public/img/projects/banshee/banshee-report-message.png" alt="The three dots next to a Discord message being selected, then apps being selected, then the text 'Report to Banshee' being selected.">

## All Users
* **/help [command]** – Show the help menu
* **/supers** – List super users
* **/user &lt;user&gt;** – Show information about a user

## Server Admins
* **/allow &lt;user&gt;** – Prevent Banshee from removing a user.
* **/allowlist** – Show list of users allowed on this server.
* **/removelegacybans** – Remove all legacy bans from Banshee.
* **/unallow &lt;user&gt;** – Removes a user from the allow list.

## Super Users
* **/register** – Register command menu
* **/super &lt;user&gt;** – Adds a new super user.
* **/unsuper &lt;user&gt;** – Removes a super user
* **/ban &lt;user&gt; [reason]** – Ban a user
* **/unban &lt;user&gt;** – Unban a user
