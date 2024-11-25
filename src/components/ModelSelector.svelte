<script lang="ts">
    import * as Select from "$lib/components/ui/select/index.js";

    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    import { toast } from "svelte-sonner";
    import { onMount } from "svelte";

    let status = $state("unloaded");

    onMount(async () => {
        const rawModels: { engine: string; name: string }[] =
            await invoke("list_models");
        models = modelListToMapWithIndexId(rawModels);
    });

    let {
        model = $bindable(),
    }: {
        model: {
            id: string;
            name: string;
            engine: string;
        } | undefined;
    } = $props();

    let models: {
        id: string;
        engine: string;
        name: string;
    }[] = $state([]);

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

    listen<string>("model_load", (event) => {
        status = event.payload;
        console.log("Model load event", status);
        toast.info(`Model load event: ${status}`);
    });

    async function loadModel(value: string) {
        if (status == "loading") return;
        model = models.find((m) => m.id === value);
        if (!model) return;
        await invoke("load_model", { model });
    }
</script>

<Select.Root
    type="single"
    disabled={models.length === 0 || status === "loading"}
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
            <Select.Item value={model.id}>{model.name}</Select.Item>
        {/each}
    </Select.Content>
</Select.Root>
