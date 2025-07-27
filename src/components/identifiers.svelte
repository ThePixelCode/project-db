<script lang="ts">
    import type { Identifier } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import IdentifierComponent from "./identifier.svelte";

    let identifiers = $state<Identifier[]>([]);
    let page = $state<number>(0);
    let showCreateModal = $state<boolean>(false);

    // Variables para el formulario de creación
    let identifier_id = $state<string>("");
    let dewey = $state<string>("");

    function process(page: number) {
        invoke<Identifier[]>("get_identifiers", { page }).then((value) => {
            identifiers = value;
        });
    }

    async function handleCreateSubmit(e: Event) {
        e.preventDefault();
        const new_identifier = {
            identifier_id,
            dewey,
        };
        let result = await invoke<boolean>("create_identifier", {
            new_identifier,
        });
        if (result) {
            showCreateModal = false;
            identifier_id = "";
            dewey = "";
            process(page);
        }
    }

    $effect(() => {
        process(page);
    });
</script>

<button
    class="mb-4 px-4 py-2 bg-green-500 text-white rounded"
    onclick={() => (showCreateModal = true)}
>
    Crear identificador
</button>

{#if showCreateModal}
    <div
        class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
    >
        <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
            <h3 class="text-lg font-bold mb-4">Crear identificador</h3>
            <form onsubmit={handleCreateSubmit} class="flex flex-col gap-2">
                <label>
                    ID:
                    <input
                        type="text"
                        class="border p-1 w-full"
                        bind:value={identifier_id}
                    />
                </label>
                <label>
                    Dewey:
                    <input
                        type="text"
                        class="border p-1 w-full"
                        bind:value={dewey}
                    />
                </label>
                <div class="flex gap-2 mt-4">
                    <button
                        type="submit"
                        class="px-4 py-2 bg-green-500 text-white rounded"
                        >Crear</button
                    >
                    <button
                        type="button"
                        class="px-4 py-2 bg-gray-300 rounded"
                        onclick={() => (showCreateModal = false)}
                        >Cancelar</button
                    >
                </div>
            </form>
        </div>
    </div>
{/if}

<table class="min-w-full border border-gray-300 mt-4">
    <thead>
        <tr class="bg-gray-100">
            <th class="px-4 py-2 border">ID</th>
            <th class="px-4 py-2 border">Dewey</th>
            <th class="px-4 py-2 border">Actions</th>
        </tr>
    </thead>
    <tbody>
        {#each identifiers as identifier}
            <IdentifierComponent {identifier} onUpdate={() => process(page)} />
        {/each}
    </tbody>
</table>

<div class="mt-4 flex gap-2">
    <button
        class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
        onclick={() => (page = Math.max(0, page - 1))}
        disabled={page === 0}>&lt; Anterior</button
    >
    <span class="self-center">Página {page + 1}</span>
    <button
        class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
        onclick={() => (page = page + 1)}>Siguiente &gt;</button
    >
</div>
