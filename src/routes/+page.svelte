<script lang="ts">
    import { setMode } from "mode-watcher";
    setMode("dark");

    import Nav from "../parts/Nav.svelte";
    import Controls from "../parts/Controls.svelte";

    let showNav = $state(true);
    let connection: boolean | null = $state(null);
</script>

<div class="flex flex-row h-screen">
    <Nav bind:showNav bind:connection />
    <main class="flex flex-col flex-1">
        <Controls bind:showNav bind:connection />
        <div id="chat"></div>
        <div id="input"></div>
    </main>
</div>
