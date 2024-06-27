<script lang="ts">
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from "@tauri-apps/api/core";

    interface ConnectionStatus {
        status: 'connected' | 'disconnected' | 'reconnected' | 'lost';
    }

    listen<ConnectionStatus>('connection-status', (event) => {
        if (!event.payload || !event.payload.status) {
            return;
        }
        let payload = event.payload;
        switch (payload.status) {
            case 'lost':
                status = 'disconnected'
                break;
            case 'reconnected':
                status = 'connected'
                break;
        }
    });

    async function refresh() {
        status = await invoke("check_connection");
    }

    let { status = $bindable() }: {
        status: string
        } = $props();
</script>

<button
    class={status}
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
