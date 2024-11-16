<script lang="ts">
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from "@tauri-apps/api/core";

    listen<boolean>('connection_status', (event) => {
        connected = event.payload;
        console.log("Connection status updated: " + connected);
    });

    async function refresh() {
        connected = await invoke("connection_status");
    }

    let { connected = $bindable() }: {
        connected: boolean;
        } = $props();
</script>

<button
    aria-label="Connection status indicator"
    class={connected ? "" : "disconnected"}
    onclick={refresh}>
</button>

<style>
    button {
        background-color: green;
        width: 20px;
        height: 20px;
        border-radius: 50%;
        border: none;
    }

    button:hover {
        cursor: pointer;
    }

    button.disconnected {
        background-color: red;
    }
</style>
