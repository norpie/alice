<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { HTMLAttributes } from 'svelte/elements';

    let {
        size,
        children,
        ...props
    }: {
        size: string | number;
        children: Snippet;
    } & HTMLAttributes<HTMLHeadingElement> = $props();
</script>

{#if size === '1' || size === 1}
    <h1 {...props}>{@render children()}</h1>
{:else if size === '2' || size === 2}
    <h2 {...props}>{@render children()}</h2>
{:else if size === '3' || size === 3}
    <h3 {...props}>{@render children()}</h3>
{:else if size === '4' || size === 4}
    <h4 {...props}>{@render children()}</h4>
{:else if size === '5' || size === 5}
    <h5 {...props}>{@render children()}</h5>
{:else if size === '6' || size === 6}
    <h6 {...props}>{@render children()}</h6>
{/if}

<style>
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
        color: var(--color-primary);
        font-weight: 700;
        margin: 0;
        padding: 0;
    }
</style>
