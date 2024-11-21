<script lang="ts">
    import * as Select from "$lib/components/ui/select/index.js";

    let selected_id = $state(null);

    $effect(() => {
        if (selected_id) {
            model = models.find((m) => m.id === selected_id) ?? null;
        }
    });

    let {
        model = $bindable(),
    }: {
        model: {
            id: string;
            name: string;
        } | null;
    } = $props();

    let models = [
        {
            id: "1",
            name: "Model 1",
        },
        {
            id: "2",
            name: "Model 2",
        },
        {
            id: "3",
            name: "Model 3",
        },
    ];

    function loadModel() {
        // TODO: Load a model.
    }

    function unloadModel() {
        // TODO: Unload a model.
    }
</script>

<Select.Root
    type="single"
    disabled={models.length === 0}
    bind:value={selected_id}
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
