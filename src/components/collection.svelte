<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Collection } from "$lib/types";
    let {
        collection,
        onUpdate,
    }: { collection: Collection; onUpdate?: () => void } = $props();
    let showEditModal = $state<boolean>(false);
    let showDeleteModal = $state<boolean>(false);

    // Variables reactivas para el formulario de edición
    let collection_name = $state<string>(
        (collection.collection_name ?? "").trim(),
    );
    let parent_collection = $state<string>(
        (collection.parent_collection ?? "").trim(),
    );

    async function handleEditSubmit(e: Event) {
        e.preventDefault();
        const new_collection: Collection = {
            ...collection,
            collection_name: collection_name.trim(),
            parent_collection: parent_collection.trim(),
        };
        let result = await invoke<boolean>("update_collection", {
            new_collection,
        });
        showEditModal = false;
        if (onUpdate) onUpdate();
    }

    async function handleDelete() {
        let result = await invoke<boolean>("delete_collection", {
            id: collection.collection_id,
        });
        showDeleteModal = false;
        if (onUpdate) onUpdate();
    }
</script>

<tr>
    <td class="px-4 py-2 border">{collection.collection_id}</td>
    <td class="px-4 py-2 border">{collection.collection_name}</td>
    <td class="px-4 py-2 border">{collection.parent_collection ?? ""}</td>
    <td class="px-4 py-2 border">
        <button
            class="bg-blue-500 text-white px-2 py-1 rounded mr-2"
            onclick={() => {
                collection_name = (collection.collection_name ?? "").trim();
                parent_collection = (collection.parent_collection ?? "").trim();
                showEditModal = true;
            }}>Editar</button
        >
        <button
            class="bg-red-500 text-white px-2 py-1 rounded"
            onclick={() => (showDeleteModal = true)}>Eliminar</button
        >

        {#if showEditModal}
            <div
                class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
            >
                <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
                    <h3 class="text-lg font-bold mb-4">Editar colección</h3>
                    <form
                        onsubmit={handleEditSubmit}
                        class="flex flex-col gap-2"
                    >
                        <label>
                            Nombre:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={collection_name}
                            />
                        </label>
                        <label>
                            Padre:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={parent_collection}
                            />
                        </label>
                        <div class="flex gap-2 mt-4">
                            <button
                                type="submit"
                                class="px-4 py-2 bg-blue-500 text-white rounded"
                                >Guardar</button
                            >
                            <button
                                type="button"
                                class="px-4 py-2 bg-gray-300 rounded"
                                onclick={() => (showEditModal = false)}
                                >Cancelar</button
                            >
                        </div>
                    </form>
                </div>
            </div>
        {/if}
        {#if showDeleteModal}
            <div
                class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
            >
                <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
                    <h3 class="text-lg font-bold mb-4">¿Eliminar colección?</h3>
                    <p>¿Estás seguro de que deseas eliminar esta colección?</p>
                    <div class="mt-4 flex gap-2">
                        <button
                            class="px-4 py-2 bg-red-500 text-white rounded"
                            onclick={handleDelete}>Eliminar</button
                        >
                        <button
                            class="px-4 py-2 bg-gray-300 rounded"
                            onclick={() => (showDeleteModal = false)}
                            >Cancelar</button
                        >
                    </div>
                </div>
            </div>
        {/if}
    </td>
</tr>
