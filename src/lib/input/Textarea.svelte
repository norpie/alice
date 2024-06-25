<script lang="ts">
    let {
        id = '',
        name = '',
        placeholder = '',
        value = '',
        rows = '3'
    } = $props<{
        id?: string;
        name: string;
        placeholder?: string;
        value?: string;
        rows?: string;
    }>();
</script>

<textarea {name} {id} {rows} {placeholder}>{value}</textarea>

<style>
    textarea {
        color: var(--color-primary);
        background-color: var(--color-accent);
        width: 600px;
        padding: 0.5rem;
        border: none;
        border-radius: 0.25rem;
        font-size: 16px;
        resize: none;
    }

    textarea::placeholder {
        color: var(--color-secondary);
    }

    textarea:focus {
        border: 2px solid var(--color-action);
        padding: calc(0.5rem - 2px);
        outline: none;
    }

    textarea:disabled {
        background-color: var(--color-disabled);
        cursor: not-allowed;
    }

    textarea:disabled::placeholder {
        color: var(--color-background);
    }
</style>
