<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js";
    import {
        convert,
        getConversations,
        type Conversation,
    } from "$lib/conversation";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "svelte-sonner";

    let {
        model = $bindable(),
        connection = $bindable(),
        conversation = $bindable(),
        conversations = $bindable(),
    }: {
        model: { name: string; engine: string } | undefined;
        connection: boolean | null;
        conversation: Conversation | null;
        conversations: Conversation[];
    } = $props();

    let content = $state("");

    onkeydown = async (event: KeyboardEvent) => {
        if (event.key !== "Enter") {
            return;
        }
        // Check if any modifier keys are pressed
        if (event.ctrlKey || event.shiftKey || event.altKey || event.metaKey) {
            return;
        }
        await submit();
    };

    async function submit() {
        if (!content || content.trim() === "") {
            return;
        }
        if (!model || !connection) {
            toast.error("No model selected");
            return;
        }
        if (!conversation) {
            conversation = convert(await invoke("new_conversation"));
        }
        if (!conversation) {
            toast.error("Failed to create conversation");
            return;
        }
        try {
            conversation = convert(
                await invoke("new_message", {
                    id: conversation.id,
                    role: "user",
                    message: content,
                }),
            );
            conversations = await getConversations(100, 0);
        } catch (e) {
            toast.error("Failed to send message");
            console.error(e);
            return;
        }
        content = "";
    }
</script>

<div
    class="flex items-center space-x-2 mx-[28rem] m-4 rounded-3xl bg-gray-800 p-3"
>
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
        class="lucide lucide-paperclip"
        ><path
            d="m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48"
        /></svg
    >
    <Input
        class="text-lg p-4 bg-transparent focus-visible:ring-0 focus-visible:ring-transparent focus-visible:ring-offset-0"
        bind:value={content}
        placeholder="Type your message to Alice here..."
    />
    <button onclick={submit} aria-label="Send message">
        <svg
            class="w-[24px] h-[24px] text-gray-800 dark:text-white"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            fill="none"
            viewBox="0 0 24 24"
        >
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="3"
                d="M12 6v13m0-13 4 4m-4-4-4 4"
            />
        </svg>
    </button>
</div>
