<script lang="ts">
    import { Button} from '../lib';
let open = $state(true);

    function toggleOpen() {
        open = !open;
    }
</script>

<aside class={open ? 'open' : ''}>
    <div class="handle">
        <Button onclick={toggleOpen}>
            {#if open}
                ≪
            {:else}
                ≫
            {/if}
        </Button>
    </div>

    <ul>
        <li>Home</li>
    </ul>
</aside>


<style>
    aside {
        width: 300px;
        height: 100vh;
        background: var(--color-accent);
        position: fixed;
        left: -300px;

        color: var(--color-primary);

        display: flex;
        flex-direction: column;

        box-shadow: 3px 1px 5px 0px rgba(0, 0, 0, 0.75);
        transition: left 0.5s;
    }

    aside.open {
        left: 0;
    }

    aside ul {
        padding-inline-start: 0;
    }

    aside ul li {
        list-style: none;
        font-size: 18px;
        padding: 20px 0;
        cursor: pointer;
    }

    .handle {
        position: relative;
        left: 100%;
    }
</style>
