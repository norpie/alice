<script lang="ts">
    import type { Snippet } from 'svelte';

    let { children, content }: {
        children?: Snippet;
        content?: string;
    } = $props();
</script>

{#if content}
    <p>
        {content}
    </p>
{:else if children}
    <p>
        {@render children()}
    </p>
{/if}

<style>
    p {
        color: var(--color-secondary);
        text-align: justify;
        text-justify: inter-word;
        margin: 0;
        padding: 0;
    }
</style>
