<script lang="ts">
    import { setMode } from "mode-watcher";
    setMode("dark");

    import Nav from "../parts/Nav.svelte";
</script>

<div class="flex flex-col h-screen">
    <Nav />
    <main>
        <div id="controls"></div>
        <div id="chat"></div>
        <div id="input"></div>
    </main>
</div>
