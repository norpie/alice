<script lang="ts">
    import { setMode } from "mode-watcher";
    setMode("dark");

    import Nav from "../parts/Nav.svelte";
    import Controls from "../parts/Controls.svelte";
    import Chat from "../parts/Chat.svelte";
    import Input from "../parts/Input.svelte";

    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let model: { id: string; name: string; engine: string } | undefined =
        $state(undefined);
    let models: { id: string; engine: string; name: string }[] = $state([]);
    let showNav: boolean = $state(true);
    let connection: boolean | null = $state(null);

    async function reload() {
        if (!connection) {
            model = undefined;
            models = [];
            return;
        }
        const rawModels: { engine: string; name: string }[] =
            await invoke("list_models");
        models = modelListToMapWithIndexId(rawModels);
    }

    function modelListToMapWithIndexId(
        models: { engine: string; name: string }[],
    ): { id: string; engine: string; name: string }[] {
        let map = [];
        for (let i = 0; i < models.length; i++) {
            map[i] = {
                id: i.toString(),
                ...models[i],
            };
        }
        return map;
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

    // If the connection status is unknown for too long, refresh it
    setTimeout(async () => {
        if (connection === null) {
            await refresh();
        }
    }, 1000);
</script>

<div class="flex flex-row h-screen">
    <Nav bind:showNav bind:connection />
    <main class="flex flex-col flex-1 justify-between">
        <Controls bind:showNav bind:connection bind:model bind:models />
        <Chat />
        <Input bind:model />
    </main>
</div>
