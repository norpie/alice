<script lang="ts">
    let { children, attributions = '' } = $props();
</script>

<div>
    {@render children()} <br />
    {#if attributions}
        <span>- {attributions}</span>
    {/if}
</div>

<style>
    div {
        background-color: var(--color-accent);
        color: var(--color-secondary);
        padding: 0.2rem;
        border-radius: 0.2rem;
        border-left: 0.2rem solid var(--color-secondary);
    }

    span {
        color: var(--color-primary);
        font-size: 0.8rem;
    }
</style>
