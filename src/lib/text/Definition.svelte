<script lang="ts">
    let { name, definition = '?' } = $props<{ name: string; definition: string }>();
</script>

<span title={definition}>{name}</span>

<style>
    span {
        font-weight: var(--font-weight-accent);
        color: var(--color-primary);
        border-bottom: 2px dashed var(--color-accent);
    }
</style>
