<script lang="ts">
    import { ScrollArea } from "../lib/components/ui/scroll-area/index.js";
    import ConnectionIndicator from "../components/ConnectionIndicator.svelte";
    import NavButton from "../components/NavButton.svelte";
    import NewChat from "../components/NewChat.svelte";
    import {
        dategrouped,
        type Conversation,
        type GroupedConversations,
    } from "../lib/conversation";

    let {
        showNav = $bindable(),
        connection = $bindable(),
        conversationId = $bindable(),
        conversations = $bindable(),
    }: {
        showNav: boolean;
        connection: boolean | null;
        conversationId: string | null;
        conversations: Conversation[];
    } = $props();

    let grouped: GroupedConversations = $derived(dategrouped(conversations));

    let default_class =
        "relative flex flex-col h-screen bg-gray-800 text-white max-w-[260px] overflow-hidden transition-width duration-300 ease-in-out";
    let active_class = $derived(
        showNav ? default_class + " w-[260px]" : default_class + " w-0",
    );

    function select(event: MouseEvent) {
        conversationId = (event.target as HTMLElement).id;
    }
</script>

<!-- Animate the slide-in effect -->
<nav class={active_class}>
    <div class="flex justify-between items-center p-4">
        <NavButton bind:showNav />
        <ConnectionIndicator bind:connection />
        <NewChat bind:conversationId />
    </div>
    <ScrollArea class="p-4 text-nowrap">
        {#each grouped as group}
            <div class="date-group pb-2">
                <p class="font-bold text-white p-2">
                    {group.name}
                </p>
                <div class="flex flex-col">
                    {#each group.conversations as conv}
                        <div
                            class="relative group flex flex-row justify-between chat-link hover:bg-gray-700 p-2 rounded-xl"
                        >
                            <button
                                class="text-gray-400 text-nowrap w-full lg:fade-out-mask group-hover:lg:fade-out-mask-earlier"
                                id={conv.id}
                                onclick={select}
                            >
                                {conv.name}
                            </button>
                            <span
                                class="chat-options invisible group-hover:visible absolute right-2 align-middle"
                            >
                                ⋯
                            </span>
                        </div>
                    {/each}
                </div>
            </div>
        {/each}
    </ScrollArea>
</nav>
