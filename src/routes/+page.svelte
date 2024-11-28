<script lang="ts">
    import { setMode } from "mode-watcher";
    setMode("dark");

    import Nav from "../parts/Nav.svelte";
    import Controls from "../parts/Controls.svelte";
    import Chat from "../parts/Chat.svelte";
    import Input from "../parts/Input.svelte";

    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";

    import {
        getConversations,
        find,
        type Conversation,
    } from "$lib/conversation";
    import modelUtils from "../lib/models";
    import { onMount, untrack } from "svelte";

    let conversationId: string | null = $state(null);
    let conversation: Conversation | null = $state(null);
    let conversations: Conversation[] = $state([]);
    let model: { name: string; engine: string } | undefined = $state(undefined);
    let models: { engine: string; name: string }[] = $state([]);
    let showNav: boolean = $state(true);
    let connection: boolean | null = $state(null);

    $effect(() => {
        find(conversationId).then((c) => {
            untrack(() => {
                conversation = c;
            });
        });
    });

    async function reload() {
        if (!connection) {
            return;
        }
        model = await modelUtils.getModel();
        models = await modelUtils.getModels();
    }

    async function newConnectionStatus(status: boolean | null) {
        connection = status;
        await reload();
    }

    listen<boolean>("connection_status", async (event) => {
        await newConnectionStatus(event.payload);
    });

    async function refresh() {
        connection = await invoke("connection_status");
        await newConnectionStatus(connection);
    }

    onMount(async () => {
        await refresh();
        conversations = await getConversations(20, 0);
    });
</script>

<div class="flex flex-row h-screen">
    <Nav bind:showNav bind:connection bind:conversationId bind:conversations />
    <main class="flex flex-col flex-1 justify-between">
        <Controls
            bind:showNav
            bind:connection
            bind:model
            bind:models
            bind:conversationId
        />
        <Chat bind:conversation />
        <Input
            bind:model
            bind:connection
            bind:conversation
            bind:conversations
        />
    </main>
</div>
