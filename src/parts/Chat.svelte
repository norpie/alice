<script lang="ts">
    import { Separator } from "$lib/components/ui/separator/index.js";

    import { CodeBlock } from "svhighlight";
    import { type Conversation } from "$lib/conversation";

    let {
        conversation = $bindable(),
    }: {
        conversation: Conversation | null;
    } = $props();
</script>

{#snippet icon(role: string)}
    <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="text-blue-500 bg-blue-100 rounded-full p-1 w-12 h-12"
    >
        {#if role == "assistant"}
            <path d="M12 6V2H8" /><path
                d="m8 18-4 4V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2Z"
            /><path d="M2 12h2" /><path d="M9 11v2" /><path d="M15 11v2" /><path
                d="M20 12h2"
            />
        {:else}
            <circle cx="12" cy="5" r="1" /><path d="m9 20 3-6 3 6" /><path
                d="m6 8 6 2 6-2"
            /><path d="M12 10v4" />
        {/if}
    </svg>
{/snippet}

<div class="flex flex-col flex-1 mx-[28rem]">
    {#if conversation}
        {#each conversation.messages as message, index}
            <div class="flex flex-col p-3">
                <div class="flex flex-row my-2">
                    {@render icon(message.role)}
                    <p class="text-lg font-bold capitalize ml-2 my-auto center">
                        {message.role}
                    </p>
                </div>
                <div class="flex flex-col flex-1">
                    {#each message.chunks as chunk}
                        {#if typeof chunk === "string"}
                            {@html chunk}
                        {:else}
                            <CodeBlock language={chunk[0]} code={chunk[1]} />
                        {/if}
                    {/each}
                </div>
            </div>
            {#if index < conversation.messages.length - 1}
                <Separator />
            {/if}
        {/each}
    {:else}
        <p class="text-center text-2xl rounded-xl bg-gray-800 py-4 my-auto">
            To start a new conversation, send a message.
        </p>
    {/if}
</div>

<style>
    :global(.code-block-language) {
        font-weight: 400;
        letter-spacing: 0.08em;
    }
</style>
