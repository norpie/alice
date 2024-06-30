<script lang="ts">
    import { P, Button, TextInput } from '../lib';
    import { invoke } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';
    import Sidebar from '../components/Sidebar.svelte';
    import ConnectionIndicator from '../components/ConnectionIndicator.svelte';

    import showdown from 'showdown';
    let converter = new showdown.Converter();

    const { data }: {
        data: {
            status: 'connected' | 'disconnected' | 'reconnected' | 'lost';
        };
    } = $props()

    let status = $state(data.status);
    status = data.status;

    interface CompletionTokens {
        tokens: string
    }

    listen<CompletionTokens>('completion-tokens', async (event) => {
        if (!event.payload || !event.payload.tokens) {
            return;
        }
        // Remove latest bot message
        let latest = history.messages.pop();
        if (!latest || latest.author !== "bot") {
            return;
        }
        if (latest.content === "...") {
            latest.content = "";
        }
        history.messages.push({
            author: "bot",
            content: latest.content + event.payload.tokens,
            html: converter.makeHtml(latest.content + event.payload.tokens),
            timestamp: new Date()
        });
    });

    async function send() {
        let inputElement = document.querySelector('input[name="userMessage"]') as HTMLInputElement;
        if (!inputElement) return;
        let message = inputElement.value;
        if (!message) return;
        history.messages.push({ author: "user", content: message, html: converter.makeHtml(message), timestamp: new Date()});
        inputElement.value = '';
        let completion = "";
        try {
            let future: Promise<string> = invoke("complete_history", { history });
            history.messages.push({ author: "bot", content: "...", html: converter.makeHtml("..."),timestamp: new Date()});
            completion = await future;
        } catch (error) {
            completion = "I'm sorry, I can't do that right now.: " + error + ".";
        }
        history.messages.pop();
        history.messages.push({ author: "bot", content: completion, html: converter.makeHtml(completion), timestamp: new Date()});
    }

    interface Message {
        author: 'user' | 'bot',
        content: string,
        html: string,
        timestamp: Date
    }

    interface History {
        id: string,
        messages: Message[],
        initialIndex: number
    }

    const initialContent = "Hi, I'm Alice, your personal assistant. I can help you with a lot of things.\n## Capibilities\n- Search for information on the web.\n- Help you with your homework.\n- Write stories.\n- Help you with your workout plan.\n- Write code.\n## Examples\n- Create a workout plan.\n- Help me with my math homework.\n- Write a story about a dragon.\n- Write a program that prints \"Hello, World!\".";
    let history: History = $state({
        id: "46695d58-a0d0-4e26-a457-f210bbb1106f",
        initialIndex: 0,
        messages: [{
            author: "bot",
            content: initialContent,
            html: converter.makeHtml(initialContent),
            timestamp: new Date()
        }]});
</script>

<Sidebar />
<main>
    <div class="chat-section">
        <div class="chat-box">
            <div class="history">
                {#each history.messages as message}
                    {#if message.author === "user"}
                        <div class="user-message">
                            <P>{message.content}</P>
                        </div>
                    {:else}
                        <div class="bot-message">
                            {@html message.html}
                        </div>
                    {/if}
                {/each}
                <!-- <div class="user-message"> -->
                <!--     <P>Hi, Alice, what can you do?</P> -->
                <!-- </div> -->
                <!-- <div class="bot-message"> -->
                <!--     <P>Hi, I'm Alice, your personal assistant. I can help you with a lot of things.</P> -->
                <!--     <H size="2">Capibilities</H> -->
                <!--     <ul> -->
                <!--         <li>Search for information on the web.</li> -->
                <!--         <li>Help you with your homework.</li> -->
                <!--         <li>Write stories.</li> -->
                <!--         <li>Help you with your workout plan.</li> -->
                <!--         <li>Write code.</li> -->
                <!--     </ul> -->
                <!--     <H size="2">Examples</H> -->
                <!--     <ul> -->
                <!--         <li>Create a workout plan.</li> -->
                <!--         <li>Help me with my math homework.</li> -->
                <!--         <li>Write a story about a dragon.</li> -->
                <!--         <li>Write a program that prints "Hello, World!".</li> -->
                <!--     </ul> -->
                <!-- </div> -->
            </div>
            <div class="bottom-bar">
                <ConnectionIndicator bind:status />
                <TextInput name="userMessage" placeholder="Chat with Alice." />
                <Button onclick={send}>Send</Button>
            </div>
        </div>
    </div>
</main>

<style>
    main {
        display: flex;
        height: 100svh;
        flex-direction: row;
    }

    .chat-section {
        display: flex;
        flex-grow: 1;
        height: 100%;
        justify-content: center;
    }

    .chat-box {
        display: flex;
        flex-direction: column;
        width: max(500px, 50%);
    }

    .history {
        overflow-y: auto;
        display: flex;
        flex-direction: column;
    }

    .bot-message {
        border: 1px solid var(--color-action);
        width: 100%;
    }

    .user-message {
        background-color: var(--color-accent);
        padding: 1rem;
        margin: 1rem;
        border-radius: 1rem;
        color: var(--color-accent);
        align-self: flex-end;
    }

    .bottom-bar {
        display: flex;
        float: bottom;
        justify-content: center;
        align-items: center;
        padding-top: 1rem;
    }

    .bottom-bar :global(input) {
        width: 100%;
        min-width: max(500px, 50%);
    }

    .bottom-bar :global(button) {
        margin: 0 1rem 0 1rem;
    }
</style>
