<script lang="ts">
    import * as Select from "$lib/components/ui/select/index.js";

    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    import { toast } from "svelte-sonner";

    let status = $state("unloaded");

    let {
        model = $bindable(),
        models = $bindable(),
        connection = $bindable(),
    }: {
        model:
            | {
                  name: string;
                  engine: string;
              }
            | undefined;
        models: {
            engine: string;
            name: string;
        }[];
        connection: boolean | null;
    } = $props();

    listen<string>("model_load", (event) => {
        status = event.payload;
        console.log("Model load event", status);
        toast.info(`Model load event: ${status}`);
    });

    async function loadModel(value: string) {
        if (status == "loading") return;
        const split = value.split("|");
        model = models.find(
            (m) => m.engine === split[0] && m.name === split[1],
        );
        if (!model) return;
        await invoke("load_model", { model });
    }
</script>

<Select.Root
    type="single"
    disabled={models.length === 0 || status === "loading" || !connection}
    onValueChange={(value) => loadModel(value)}
>
    <Select.Trigger class="max-w-[260px] truncate">
        {#if model}
            {model.name}
        {:else}
            Load a model.
        {/if}
    </Select.Trigger>
    <Select.Content>
        {#each models as model}
            <Select.Item value={model.engine + "|" + model.name}
                >{model.name}</Select.Item
            >
        {/each}
    </Select.Content>
</Select.Root>
