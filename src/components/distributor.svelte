<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Distributor } from "$lib/types";
    let {
        distributor,
        onUpdate,
    }: { distributor: Distributor; onUpdate?: () => void } = $props();
    let showEditModal = $state<boolean>(false);
    let showDeleteModal = $state<boolean>(false);

    // Variables reactivas para el formulario de edición
    let country_id = $state<string>(distributor.country_id.trim());
    let distributor_name = $state<string>(distributor.distributor_name.trim());
    let foundation_year = $state<string>(distributor.foundation_year ?? "");

    async function handleEditSubmit(e: Event) {
        e.preventDefault();
        const new_distributor: Distributor = {
            ...distributor,
            country_id: country_id.trim(),
            distributor_name: distributor_name.trim(),
            foundation_year,
        };
        let result = await invoke<boolean>("update_distributor", {
            new_distributor,
        });
        showEditModal = false;
        if (onUpdate) onUpdate();
    }

    async function handleDelete() {
        let result = await invoke<boolean>("delete_distributor", {
            id: distributor.distributor_id,
        });
        showDeleteModal = false;
        if (onUpdate) onUpdate();
    }
</script>

<tr>
    <td class="px-4 py-2 border">{distributor.distributor_id}</td>
    <td class="px-4 py-2 border">{distributor.country_id}</td>
    <td class="px-4 py-2 border">{distributor.distributor_name}</td>
    <td class="px-4 py-2 border">{distributor.foundation_year}</td>
    <td class="px-4 py-2 border">
        <button
            class="bg-blue-500 text-white px-2 py-1 rounded mr-2"
            onclick={() => {
                country_id = distributor.country_id.trim();
                distributor_name = distributor.distributor_name.trim();
                foundation_year = distributor.foundation_year ?? "";
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
                    <h3 class="text-lg font-bold mb-4">Editar distribuidor</h3>
                    <form
                        onsubmit={handleEditSubmit}
                        class="flex flex-col gap-2"
                    >
                        <label>
                            País:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={country_id}
                            />
                        </label>
                        <label>
                            Nombre:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={distributor_name}
                            />
                        </label>
                        <label>
                            Año de fundación:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={foundation_year}
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
                    <h3 class="text-lg font-bold mb-4">
                        ¿Eliminar distribuidor?
                    </h3>
                    <p>
                        ¿Estás seguro de que deseas eliminar este distribuidor?
                    </p>
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
