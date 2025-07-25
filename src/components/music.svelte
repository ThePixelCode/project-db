<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { type music as musicType } from "../lib/types";

    let { music, onUpdate }: { music: musicType; onUpdate?: () => void } =
        $props();
    let showEditModal = $state<boolean>(false);
    let showDeleteModal = $state<boolean>(false);

    // Formulario de edición: variables individuales para reactividad
    let object_name = $state<string>(music.object_name.trim());
    let publication_year = $state<string>(music.publication_year);
    let identifier_id = $state<string>(music.identifier_id.trim());
    let language_id = $state<string>(music.language_id.trim());
    let duration = $state<string>(music.duration.trim());

    async function handleEditSubmit(e: Event) {
        e.preventDefault();
        const new_music: musicType = {
            ...music,
            object_name,
            publication_year,
            identifier_id,
            language_id,
            duration,
        };
        let result = await invoke<boolean>("update_music", {
            new_music,
        });
        console.log(
            `Update ${music.object_id} ${result ? "updated" : "not updated"}`,
        );
        showEditModal = false;
        if (onUpdate) onUpdate();
    }

    async function handleDelete() {
        let result = await invoke<boolean>("delete_music", {
            id: music.object_id,
        });
        console.log(
            `Delete ${music.object_id} ${result ? "deleted" : "not deleted"}`,
        );
        showDeleteModal = false;
        if (onUpdate) onUpdate();
    }
</script>

<tr>
    <td class="px-4 py-2 border">{music.object_id.trim()}</td>
    <td class="px-4 py-2 border">{music.object_name.trim()}</td>
    <td class="px-4 py-2 border">{music.publication_year}</td>
    <td class="px-4 py-2 border">{music.identifier_id.trim()}</td>
    <td class="px-4 py-2 border">{music.language_id.trim()}</td>
    <td class="px-4 py-2 border">{music.duration.trim()}</td>
    <td class="px-4 py-2 border">
        <button
            class="bg-blue-500 text-white px-2 py-1 rounded mr-2"
            onclick={() => {
                object_name = music.object_name.trim();
                publication_year = music.publication_year.trim();
                identifier_id = music.identifier_id;
                language_id = music.language_id.trim();
                duration = music.duration.trim();
                showEditModal = true;
            }}>Edit</button
        >
        <button
            class="bg-red-500 text-white px-2 py-1 rounded"
            onclick={() => (showDeleteModal = true)}>Delete</button
        >

        {#if showEditModal}
            <div
                class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
            >
                <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
                    <h3 class="text-lg font-bold mb-4">Editar música</h3>
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
                            Duración:
                            <input
                                type="text"
                                class="border p-1 w-full"
                                bind:value={duration}
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
                    <h3 class="text-lg font-bold mb-4">¿Eliminar música?</h3>
                    <p>¿Estás seguro de que deseas eliminar esta música?</p>
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
