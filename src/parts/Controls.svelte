<script lang="ts">
    import ConnectionIndicator from "../components/ConnectionIndicator.svelte";
    import ModelSelector from "../components/ModelSelector.svelte";
    let {
        showNav = $bindable(),
        connection = $bindable(),
    }: {
        showNav: boolean;
        connection: boolean | null;
    } = $props();

    let defaultClass =
        "flex flex-col max-w-[260px] p-4 transition-all duration-300";
    let showNavClass = $derived(showNav ? defaultClass + " -translate-y-[50%]" : defaultClass + " translate-y-0");
</script>

<div class={showNavClass}>
    <div class="flex flex-row justify-between pb-8">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-panel-right-close"
            onclick={() => (showNav = !showNav)}
            ><rect width="18" height="18" x="3" y="3" rx="2" /><path
                d="M15 3v18"
            /><path d="m8 9 3 3-3 3" /></svg
        >
        <ConnectionIndicator bind:connection />
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-square-pen"
            ><path
                d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
            /><path
                d="M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1 .506-.852z"
            /></svg
        >
    </div>
    <ModelSelector />
</div>
