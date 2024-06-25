<script lang="ts">
    let { children } = $props();
</script>

<div>
    {@render children()}

</div>

<style>
    div {
        font-family: var(--font-mono);
        background-color: var(--color-accent);
        color: var(--color-secondary);
        padding: 0.2rem;
        border-radius: 0.2rem;
    }
</style>
