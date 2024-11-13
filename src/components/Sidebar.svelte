<script lang="ts">
    import Logo from './Logo.svelte';
</script>

<aside>
    <div class="handle">
        <Logo/>
    </div>
</aside>

<style>
    aside {
        z-index: 1000;
        width: 28rem;
        height: 100vh;
        background: var(--color-accent);
        position: fixed;
        left: -28rem;

        color: var(--color-primary);

        display: flex;
        flex-direction: column;

        box-shadow: 3px 1px 5px 0px rgba(0, 0, 0, 0.75);
        transition: left 0.5s;
    }

    aside:hover {
        left: 0;
    }

    aside .handle {
        position: fixed;
        top: 0;
        left: 0;
        width: 4rem;
        height: 100vh;
    }
</style>
