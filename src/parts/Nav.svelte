<script lang="ts">
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    import ConnectionIndicator from "../components/ConnectionIndicator.svelte";

    import placeholder from "$lib/placeholder";
    let chats = placeholder.chats;

    let {
        showNav = $bindable(),
        connection = $bindable(),
    }: {
        showNav: boolean;
        connection: boolean | null;
    } = $props();

    let default_class =
        "relative flex flex-col h-screen bg-gray-800 text-white max-w-[260px] overflow-hidden transition-width duration-300 ease-in-out";
    let active_class = $derived(
        showNav ? default_class + " w-[260px]" : default_class + " w-0",
    );
</script>

<!-- Animate the slide-in effect -->
<nav class={active_class}>
    <div class="flex justify-between items-center p-4">
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
            class="lucide lucide-panel-right-open"
            onclick={() => (showNav = !showNav)}
            ><rect width="18" height="18" x="3" y="3" rx="2" /><path
                d="M15 3v18"
            /><path d="m10 15-3-3 3-3" /></svg
        >
        <ConnectionIndicator bind:connection />
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
            class="lucide lucide-square-pen"
            ><path
                d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
            /><path
                d="M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1 .506-.852z"
            /></svg
        >
    </div>
    <ScrollArea class="p-4 text-nowrap">
        {#each chats as chat}
            <div class="date-group pb-2">
                <p class="font-bold text-white p-2">
                    {chat.date_group}
                </p>
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
