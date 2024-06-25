<script lang="ts">
    let { children } = $props();
</script>

<span>{@render children()}</span>

<style>
    span {
        font-family: var(--font-mono);
        background-color: var(--color-accent);
        color: var(--color-secondary);
        padding: 0.2rem;
        border-radius: 0.2rem;
    }
</style>
