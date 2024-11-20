<script lang="ts">
    import { setMode } from "mode-watcher";
    setMode("dark");

    import Nav from "../parts/Nav.svelte";
    import Controls from "../parts/Controls.svelte";

    let showNav = $state(true);
</script>

<div class="flex flex-row h-screen">
    <Nav bind:showNav />
    <main class="flex flex-col flex-1">
        <Controls bind:showNav />
        <div id="chat"></div>
        <div id="input"></div>
    </main>
</div>
