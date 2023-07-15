import { REST } from '@discordjs/rest';
import { WebSocketManager } from '@discordjs/ws';
import { GatewayDispatchEvents, GatewayIntentBits, InteractionType, MessageFlags, Client } from '@discordjs/core';
import 'dotenv/config'

const token = process.env.BOT_TOKEN;

const rest = new REST({ version: '10' }).setToken(token);

const gateway = new WebSocketManager({
    token,
    intents: GatewayIntentBits.DirectMessages,
    rest,
});

const client = new Client({ rest, gateway });

client.on(GatewayDispatchEvents.InteractionCreate, async ({ data: interaction, api }) => {
    if (interaction.type !== InteractionType.ApplicationCommand || interaction.data.name !== 'ping') {
        return;
    }

    await api.interactions.reply(interaction.id, interaction.token, { content: 'Pong!', flags: MessageFlags.Ephemeral });
});

client.once(GatewayDispatchEvents.Ready, () => console.log('Ready!'));

gateway.connect();