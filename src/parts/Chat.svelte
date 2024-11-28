<script lang="ts">
    import { Separator } from "$lib/components/ui/separator/index.js";

    import placeholder from "../lib/placeholder";
    import { toHtml } from "$lib/markdown";
    import { CodeBlock } from "svhighlight";

    interface RawMesssage {
        role: string;
        message: string;
    }

    interface Message {
        role: string;
        chunks: Chunk[];
    }

    type Chunk = string | [string, string];

    let conversation: RawMesssage[] = $state(placeholder.conversation);
    let htmlizedConversation: Message[] = $state(htmlize(conversation));

    function htmlize(conversation: RawMesssage[]) {
        return conversation.map((message) => {
            let html = toHtml(message.message);
            let parsed = new DOMParser().parseFromString(html, "text/html");
            let chunks = [];
            for (let node of parsed.body.childNodes) {
                // if the node is a code block, we need to extract the language and the code,
                // code blocks are a <code> nested in a <pre> the language is stored as the first class
                // if the node is a not a code block, we just need to extract the html
                if (
                    node.nodeName === "PRE" &&
                    node.firstChild &&
                    node.firstChild.nodeName === "CODE"
                ) {
                    let code = node.firstChild.textContent;
                    let language = node.firstChild.classList[0];
                    chunks.push([language, code]);
                } else {
                    const html = node.outerHTML;
                    if (html) chunks.push(html);
                }
            }
            return {
                role: message.role,
                chunks,
            };
        });
    }

    // $effect(() => {
    // htmlized_conversation = htmlize(conversation);
    // });

    const roleClass = "text-blue-500 bg-blue-100 rounded-full p-1 w-12 h-12";
</script>

<div class="flex flex-col flex-1 mx-[28rem]">
    {#each htmlizedConversation as message, index}
        <div class="flex flex-col p-3">
            <div class="flex flex-row my-2">
                {#if message.role == "assistant"}
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
                        class={roleClass}
                        ><path d="M12 6V2H8" /><path
                            d="m8 18-4 4V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2Z"
                        /><path d="M2 12h2" /><path d="M9 11v2" /><path
                            d="M15 11v2"
                        /><path d="M20 12h2" /></svg
                    >
                {:else}
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
                        class={roleClass}
                        ><circle cx="12" cy="5" r="1" /><path
                            d="m9 20 3-6 3 6"
                        /><path d="m6 8 6 2 6-2" /><path d="M12 10v4" /></svg
                    >
                {/if}
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
        {#if index < htmlizedConversation.length - 1}
            <Separator />
        {/if}
    {/each}
</div>

<style>
    :global(.code-block-language) {
        font-weight: 400;
        letter-spacing: 0.08em;
    }
</style>
