<script lang="ts">
    import { P, H, Button, TextInput } from '../lib';
    import Sidebar from '../components/Sidebar.svelte';
    import ConnectionIndicator from '../components/ConnectionIndicator.svelte';

    const { data }: {
        data: {
            status: 'connected' | 'disconnected' | 'reconnected' | 'lost';
        };
    } = $props()

    let status = $state("disconnected");
    status = data.status;

    function send() {
        console.log('send');
    }
</script>

<Sidebar />
<main>
    <div class="chat-section">
        <div class="chat-box">
            <div class="history">
                <div class="user-message">
                    <P>Hi, Alice, what can you do?</P>
                </div>
                <div class="bot-message">
                    <P>Hi, I'm Alice, your personal assistant. I can help you with a lot of things.</P>
                    <H size="2">Capibilities</H>
                    <ul>
                        <li>Search for information on the web.</li>
                        <li>Help you with your homework.</li>
                        <li>Write stories.</li>
                        <li>Help you with your workout plan.</li>
                        <li>Write code.</li>
                    </ul>
                    <H size="2">Examples</H>
                    <ul>
                        <li>Create a workout plan.</li>
                        <li>Help me with my math homework.</li>
                        <li>Write a story about a dragon.</li>
                        <li>Write a program that prints "Hello, World!".</li>
                    </ul>
                </div>
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
