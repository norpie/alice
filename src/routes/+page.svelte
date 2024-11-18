<script lang="ts">
    import { P, Button, Textarea } from "../lib";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import Sidebar from "../components/Sidebar.svelte";
    import ConnectionIndicator from "../components/ConnectionIndicator.svelte";
    import { toHighlightedHtml, toHtml } from "../lib/markdown";
    import { resetMode, setMode } from "mode-watcher";

    setMode("dark");

    interface CompletionTokens {
        tokens: string;
    }

    let lastCompletionTimestamp = 0;
    listen<CompletionTokens>("completion-tokens", async (event) => {
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
        let content = latest.content + event.payload.tokens;
        let html = latest.html;
        let now = Date.now();
        let diff = now - lastCompletionTimestamp;
        if (diff > 10 || lastCompletionTimestamp === 0) {
            html = toHighlightedHtml(content);
            lastCompletionTimestamp = now;
        }
        history.messages.push({
            author: "bot",
            content,
            html,
            timestamp: new Date(),
        });
    });

    async function send() {
        let inputElement = document.querySelector(
            'textarea[name="userMessage"]',
        ) as HTMLInputElement;
        if (!inputElement) return;
        let message = inputElement.value;
        if (!message) return;
        history.messages.push({
            author: "user",
            content: message,
            html: toHtml(message),
            timestamp: new Date(),
        });
        inputElement.value = "";
        let completion = "";
        try {
            let future: Promise<string> = invoke("complete_history", {
                history,
            });
            history.messages.push({
                author: "bot",
                content: "...",
                html: toHtml("..."),
                timestamp: new Date(),
            });
            completion = await future;
        } catch (error) {
            completion =
                "I'm sorry, I can't do that right now.: " + error + ".";
        }
        history.messages.pop();
        history.messages.push({
            author: "bot",
            content: completion,
            html: toHighlightedHtml(completion),
            timestamp: new Date(),
        });
    }

    interface Message {
        author: "user" | "bot";
        content: string;
        html: string;
        timestamp: Date;
    }

    interface History {
        id: string;
        messages: Message[];
        initialIndex: number;
    }

    const initialContent =
        'Hi, I\'m Alice, your personal assistant. I can help you with a lot of things.\n## Capibilities\n- Search for information on the web.\n- Help you with your homework.\n- Write stories.\n- Help you with your workout plan.\n- Write code.\n## Examples\n- Create a workout plan.\n- Help me with my math homework.\n- Write a story about a dragon.\n- Write a program that prints "Hello, World!".';
    let history: History = $state({
        id: "46695d58-a0d0-4e26-a457-f210bbb1106f",
        initialIndex: 0,
        messages: [
            {
                author: "bot",
                content: initialContent,
                html: toHtml(initialContent),
                timestamp: new Date(),
            },
        ],
    });
</script>

<Sidebar />
<main>
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
        </div>
        <div class="bottom-bar">
            <ConnectionIndicator />
            <Textarea
                rows="1"
                name="userMessage"
                placeholder="Chat with Alice."
            />
            <Button onclick={send}>Send</Button>
        </div>
    </div>
</main>

<style>
    main {
        display: flex;
        height: 100svh;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        width: 100%;
    }

    .chat-box {
        display: flex;
        flex-grow: 1;
        height: 100%;
        max-width: 1200px;
        min-width: 700px;
        flex-direction: column;
        justify-content: space-between;
    }

    .history {
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        mask-image: linear-gradient(
            to bottom,
            transparent 0%,
            black 30px,
            black calc(100% - 30px),
            transparent 100%
        );
        padding: 4rem;
    }

    .history::-webkit-scrollbar {
        display: none;
    }

    .history :global(p) {
        text-align: justify;
        text-justify: inter-word;
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
        align-items: center;
        justify-content: space-between;
        margin: 1rem 1rem 1rem 1rem;
    }

    .bottom-bar :global(textarea) {
        width: 100%;
        margin-left: 1rem;
        margin-right: 1rem;
    }
</style>
