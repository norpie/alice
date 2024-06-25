<script lang="ts">
    let {
        id = '',
        name = '',
        value = '',
        checked = false,
        disabled = false
    } = $props<{ id?: string; name: string; value: string; checked?: boolean; disabled?: boolean }>();
</script>

<input type="checkbox" {id} {name} {value} {checked} {disabled} />

<style>
    [type='checkbox'] {
        width: 1rem;
        height: 1rem;
        color: var(--color-primary);
        vertical-align: middle;
        appearance: none;
        -webkit-appearance: none;
        background: none;
        border: 0;
        outline: 0;
        flex-grow: 0;
        border-radius: 0.25rem;
        background-color: var(--color-accent);
        transition: background 300ms;
        cursor: pointer;
    }

    /* Pseudo element for check styling */

    [type='checkbox']::before {
        content: '';
        color: transparent;
        display: block;
        width: inherit;
        height: inherit;
        border-radius: inherit;
        border: 0;
        background-color: transparent;
        background-size: contain;
    }

    /* Checked */

    [type='checkbox']:checked {
        background-color: var(--color-action);
    }

    [type='checkbox']:checked::before {
        box-shadow: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='32' height='32' viewBox='4 4 16 16'%3E %3Cpath d='M15.88 8.29L10 14.17l-1.88-1.88a.996.996 0 1 0-1.41 1.41l2.59 2.59c.39.39 1.02.39 1.41 0L17.3 9.7a.996.996 0 0 0 0-1.41c-.39-.39-1.03-.39-1.42 0z' fill='%23fff'/%3E %3C/svg%3E");
    }

    /* Disabled */

    [type='checkbox']:disabled {
        background-color: var(--color-disabled);
        opacity: 0.84;
        cursor: not-allowed;
    }

    /* IE */

    [type='checkbox']::-ms-check {
        content: '';
        color: transparent;
        display: block;
        width: inherit;
        height: inherit;
        border-radius: inherit;
        border: 0;
        background-color: transparent;
        background-size: contain;
        box-shadow: inset 0 0 0 1px #ccd3d8;
    }

    [type='checkbox']:checked::-ms-check {
        box-shadow: none;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24'%3E %3Cpath d='M15.88 8.29L10 14.17l-1.88-1.88a.996.996 0 1 0-1.41 1.41l2.59 2.59c.39.39 1.02.39 1.41 0L17.3 9.7a.996.996 0 0 0 0-1.41c-.39-.39-1.03-.39-1.42 0z' fill='%23fff'/%3E %3C/svg%3E");
    }
</style>
