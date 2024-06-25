<script lang="ts">
    let {
        name = '',
        placeholder = '',
        value = '',
        checked = false,
        disabled = false
    } = $props<{
        name: string;
        placeholder?: string;
        value?: string;
        checked?: boolean;
        disabled?: boolean;
    }>();
</script>

<input type="text" {name} {placeholder} {value} {disabled} {checked} />

<style>
    input {
        color: var(--color-primary);
        background-color: var(--color-accent);
        font-family: inherit;
        border: none;
        padding: 0.5rem;
        border-radius: 0.25rem;
        font-size: 1rem;
    }

    input::placeholder {
        color: var(--color-secondary);
    }

    input:focus {
        border: 2px solid var(--color-action);
        padding: calc(0.5rem - 2px);
        outline: none;
    }

    input:disabled {
        background-color: var(--color-disabled);
        cursor: not-allowed;
    }
</style>
