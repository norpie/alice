<script lang="ts">
    import * as Select from "$lib/components/ui/select/index.js";

    let selected_id = null;
    let models = [
        {
            id: 1,
            name: "Model 1",
        },
        {
            id: 2,
            name: "Model 2",
        },
        {
            id: 3,
            name: "Model 3",
        },
    ];
</script>

<Select.Root type="single" disabled={models.length === 0} bind:value={selected_id}>
    <Select.Trigger class="max-w-[260px]">
        {#if selected_id}
            {#each models as model}
                {#if model.id === selected_id}
                    {model.name}
                {/if}
            {/each}
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
