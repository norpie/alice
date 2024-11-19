<script lang="ts">
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import { resetMode, setMode } from "mode-watcher";

    setMode("dark");
    let chats = [
        {
            date_group: "Today",
            conversations: [
                {
                    title: "Lorem ipsum dolor sit amet",
                    id: "1",
                },
                {
                    title: "Consectetur adipiscing",
                    id: "2",
                },
                {
                    title: "Sed do eiusmod tempor incididunt",
                    id: "3",
                },
                {
                    title: "Ut labore et dolore magna aliqua",
                    id: "4",
                },
            ],
        },
        {
            date_group: "Yesterday",
            conversations: [
                {
                    title: "Ut enim ad minim",
                    id: "5",
                },
                {
                    title: "Quis nostrud exercitation ullamco",
                    id: "6",
                },
                {
                    title: "Laboris nisi ut aliquip ex ea commodo",
                    id: "7",
                },
                {
                    title: "Consequat duis",
                    id: "8",
                },
            ],
        },
        {
            date_group: "Last Week",
            conversations: [
                {
                    title: "In reprehenderit in voluptate",
                    id: "9",
                },
                {
                    title: "Velit esse cillum dolore eu fugiat",
                    id: "10",
                },
                {
                    title: "Nulla pariatur",
                    id: "11",
                },
                {
                    title: "Excepteur sint occaecat cupidatat",
                    id: "12",
                },
            ],
        },
        {
            date_group: "Last Month",
            conversations: [
                {
                    title: "Non proident sunt in culpa",
                    id: "13",
                },
                {
                    title: "Lorem ipsum dolor sit amet",
                    id: "14",
                },
                {
                    title: "Consectetur adipiscing",
                    id: "15",
                },
                {
                    title: "Sed do eiusmod tempor incididunt",
                    id: "16",
                },
            ],
        },
        {
            date_group: "3+ Months Ago",
            conversations: [
                {
                    title: "Ut labore et dolore magna aliqua",
                    id: "17",
                },
                {
                    title: "Ut enim ad minim",
                    id: "18",
                },
                {
                    title: "Quis nostrud exercitation ullamco",
                    id: "19",
                },
                {
                    title: "Laboris nisi ut aliquip ex ea commodo",
                    id: "20",
                },
            ],
        },
        {
            date_group: "Older",
            conversations: [
                {
                    title: "Consequat duis",
                    id: "21",
                },
                {
                    title: "In reprehenderit in voluptate",
                    id: "22",
                },
                {
                    title: "Velit esse cillum dolore eu fugiat",
                    id: "23",
                },
                {
                    title: "Nulla pariatur",
                    id: "24",
                },
            ],
        },
    ];
</script>

<div class="flex flex-col h-screen">
    <!-- Sidebar Nav: Top has buttons for toggling sidebar and new chat, bottom has list of chats -->
    <nav class="flex flex-col h-screen bg-gray-800 text-white max-w-[260px]">
        <div class="flex justify-between items-center p-4">
            <button class="text-white" on:click={resetMode}
                >Toggle Sidebar</button
            >
            <button class="text-white">New Chat</button>
        </div>
        <!-- List of chats, text should aligned to the left, scrollable -->
        <ScrollArea class="p-4">
            {#each chats as chat}
                <div class="date-group pb-4">
                    <p2 class="font-bold text-white p-2">
                        {chat.date_group}
                    </p2> <br />
                    <div class="flex flex-col">
                        {#each chat.conversations as conv}
                            <!-- fade mask on right side of text -->
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

    <!-- Main Content: Top is for some misc settings/controls, middle is chat area, bottom is for input -->
    <main>
        <div id="controls"></div>
        <div id="chat"></div>
        <div id="input"></div>
    </main>
</div>

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
