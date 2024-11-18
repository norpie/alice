<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";

    listen<boolean>("connection_status", (event) => {
        connected = event.payload;
    });

    async function refresh() {
        connected = await invoke("connection_status");
    }

    let connected: boolean | null = $state(null);

    function whichClass() {
        if (connected === null) {
            return "unknown";
        } else if (connected) {
            return "";
        } else {
            return "disconnected";
        }
    }
</script>

<button
    aria-label="Connection status indicator"
    class={whichClass()}
    onclick={refresh}
>
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

    button.unknown {
        background-color: gray;
    }

    button.disconnected {
        background-color: red;
    }
</style>
