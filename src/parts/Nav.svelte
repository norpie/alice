<script lang="ts">
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";

    import placeholder from "$lib/placeholder";
    let chats = placeholder.chats;
</script>

<nav class="flex flex-col h-screen bg-gray-800 text-white max-w-[260px]">
    <div class="flex justify-between items-center p-4">
        <button class="text-white">Toggle Sidebar</button>
        <button class="text-white">New Chat</button>
    </div>
    <ScrollArea class="p-4">
        {#each chats as chat}
            <div class="date-group pb-4">
                <p2 class="font-bold text-white p-2">
                    {chat.date_group}
                </p2> <br />
                <div class="flex flex-col">
                    {#each chat.conversations as conv}
                        <div
                            class="relative group flex flex-row justify-between chat-link hover:bg-gray-700 p-2 rounded-xl"
                        >
                            <a
                                class="text-gray-400 text-nowrap w-full lg:fade-out-mask group-hover:lg:fade-out-mask-earlier"
                                href="/?id={conv.id}"
                            >
                                {conv.title}
                            </a>
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

<style>
    @tailwind utilities;

    @layer utilities {
        .fade-out-mask {
            -webkit-mask-image: linear-gradient(
                to right,
                black 90%,
                transparent 100%
            );
            mask-image: linear-gradient(to right, black 90%, transparent 100%);
            -webkit-mask-repeat: no-repeat;
            mask-repeat: no-repeat;
            -webkit-mask-size: 100% 100%;
            mask-size: 100% 100%;
        }

        .fade-out-mask-earlier {
            -webkit-mask-image: linear-gradient(
                to right,
                black 80%,
                transparent 90%
            );
            mask-image: linear-gradient(to right, black 83%, transparent 93%);
            -webkit-mask-repeat: no-repeat;
            mask-repeat: no-repeat;
            -webkit-mask-size: 100% 100%;
            mask-size: 100% 100%;
        }
    }
</style>
