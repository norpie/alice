<script lang="ts">
    import ConnectionIndicator from "../components/ConnectionIndicator.svelte";
    import NavButton from "../components/NavButton.svelte";
    import ModelSelector from "../components/ModelSelector.svelte";
    import NewChat from "../components/NewChat.svelte";
    let {
        showNav = $bindable(),
        connection = $bindable(),
        conversationId = $bindable(),
        model = $bindable(),
        models = $bindable(),
    }: {
        showNav: boolean;
        connection: boolean | null;
        conversationId: string | null;
        model: { name: string; engine: string } | undefined;
        models: { engine: string; name: string }[];
    } = $props();

    let defaultClass =
        "flex flex-col max-w-[260px] p-4 transition-all duration-300";
    let showNavClass = $derived(
        showNav
            ? defaultClass + " -translate-y-[50%]"
            : defaultClass + " translate-y-0",
    );
</script>

<div class={showNavClass}>
    <div class="flex flex-row justify-between pb-8">
        <NavButton bind:showNav />
        <ConnectionIndicator bind:connection />
        <NewChat bind:conversationId />
    </div>
    <ModelSelector bind:model bind:models bind:connection />
</div>
