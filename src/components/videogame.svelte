<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Videogame } from "../lib/types";

    let {
        videogame,
        onUpdate,
    }: { videogame: Videogame; onUpdate?: () => void } = $props();
    let showEditModal = $state<boolean>(false);
    let showDeleteModal = $state<boolean>(false);
    console.log(typeof videogame.publication_year);

    // Formulario de edición: variables individuales para reactividad
    let object_name = $state<string>(videogame.object_name.trim());
    let publication_year = $state<string>(videogame.publication_year ?? "");
    let identifier_id = $state<string>(videogame.identifier_id.trim());
    let language_id = $state<string>(videogame.language_id.trim());
    let controller_support = $state<boolean>(videogame.controller_support);
    let pegi_id = $state<string>(videogame.pegi_id.trim());

    async function handleEditSubmit(e: Event) {
        e.preventDefault();
        const new_videogame: Videogame = {
            ...videogame,
            object_name,
            publication_year,
            identifier_id,
            language_id,
            controller_support,
            pegi_id,
        };
        let result = await invoke<boolean>("update_videogame", {
            new_videogame,
        });
        console.log(
            `Update ${videogame.object_id} ${result ? "updated" : "not updated"}`,
        );
        showEditModal = false;
        if (onUpdate) onUpdate();
    }

    async function handleDelete() {
        let result = await invoke<boolean>("delete_videogame", {
            id: videogame.object_id,
        });
        console.log(
            `Delete ${videogame.object_id} ${result ? "deleted" : "not deleted"}`,
        );
        showDeleteModal = false;
        if (onUpdate) onUpdate();
    }
</script>

<tr>
    <td class="px-4 py-2 border">{videogame.object_id.trim()}</td>
    <td class="px-4 py-2 border">{videogame.object_name.trim()}</td>
    <td class="px-4 py-2 border">{videogame.publication_year}</td>
    <td class="px-4 py-2 border">{videogame.identifier_id.trim()}</td>
    <td class="px-4 py-2 border">{videogame.language_id.trim()}</td>
    <td class="px-4 py-2 border"
        >{videogame.controller_support ? "Sí" : "No"}</td
    >
    <td class="px-4 py-2 border">{videogame.pegi_id.trim()}</td>
    <td class="px-4 py-2 border">
        <button
            class="bg-blue-500 text-white px-2 py-1 rounded mr-2"
            onclick={() => {
                // force update of values, fixes bug
                object_name = videogame.object_name.trim();
                publication_year = videogame.publication_year ?? "";
                identifier_id = videogame.identifier_id.trim();
                language_id = videogame.language_id.trim();
                controller_support = videogame.controller_support;
                pegi_id = videogame.pegi_id.trim();
                showEditModal = true;
            }}
        >
            Edit
        </button>
        <button
            class="bg-red-500 text-white px-2 py-1 rounded"
            onclick={() => (showDeleteModal = true)}>Delete</button
        >

        {#if showEditModal}
            <div
                class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
            >
                <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
                    <h3 class="text-lg font-bold mb-4">Editar videojuego</h3>
                    <form
                        onsubmit={handleEditSubmit}
                        class="flex flex-col gap-2"
                    >
                        <label>
                            Nombre:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={object_name}
                            />
                        </label>
                        <label>
                            Año de publicación:
                            <input
                                type="date"
                                class="border p-1 w-full"
                                bind:value={publication_year}
                            />
                        </label>
                        <label>
                            Identificador:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={identifier_id}
                            />
                        </label>
                        <label>
                            Idioma:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={language_id}
                            />
                        </label>
                        <label>
                            Soporte de control:
                            <input
                                type="checkbox"
                                bind:checked={controller_support}
                            />
                        </label>
                        <label>
                            PEGI:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={pegi_id}
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
                        ¿Eliminar videojuego?
                    </h3>
                    <p>¿Estás seguro de que deseas eliminar esta videojuego?</p>
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
