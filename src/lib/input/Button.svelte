<script lang="ts">
    import type { Snippet } from 'svelte';
    import type { HTMLAttributes } from 'svelte/elements';
    let {
        text = '',
        pill = false,
        disabled = false,
        children,
        ...props
    }: {
        text?: string;
        pill?: boolean;
        disabled?: boolean;
        children?: Snippet;
    } & HTMLAttributes<HTMLButtonElement> = $props();
</script>

<button class:pill {disabled} {...props}>
    {#if text}
        {text}
    {/if}
    {#if children}
        {@render children()}
    {/if}
</button>

<style>
    button {
        border: none;
        background-color: var(--color-action);
        color: var(--color-primary);
        cursor: pointer;
        margin-bottom: 0.5rem;
        border-radius: 0.25rem;
        padding: 0.5rem 1rem;
        font-size: 1rem;
        font-weight: var(--font-weight-action);
        transition: background-color 0.2s;
    }

    button:hover {
        background-color: var(--color-accent);
    }

    button:active {
        background-color: var(--color-secondary);
        color: var(--color-background);
    }

    button:disabled {
        background-color: var(--color-disabled);
        cursor: not-allowed;
    }

    button:disabled:hover {
        background-color: var(--color-disabled);
    }

    button.pill {
        border-radius: 4rem;
    }
</style>
