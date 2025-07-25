<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { type book as bookType } from "../lib/types";

    let { book, onUpdate }: { book: bookType; onUpdate?: () => void } =
        $props();
    let showEditModal = $state<boolean>(false);
    let showDeleteModal = $state<boolean>(false);

    // Variables reactivas para el formulario de edición
    let object_name = $state<string>(book.object_name.trim());
    let publication_year = $state<string>(book.publication_year);
    let identifier_id = $state<string>(book.identifier_id.trim());
    let language_id = $state<string>(book.language_id.trim());
    let page_count = $state<string>(`${book.page_count}`);

    async function handleEditSubmit(e: Event) {
        e.preventDefault();
        const new_book: bookType = {
            ...book,
            object_name,
            publication_year,
            identifier_id,
            language_id,
            page_count: Number(page_count.trim()),
        };
        let result = await invoke<boolean>("update_book", {
            new_book,
        });
        console.log(
            `Update ${book.object_id} ${result ? "updated" : "not updated"}`,
        );
        showEditModal = false;
        if (onUpdate) onUpdate();
    }

    async function handleDelete() {
        let result = await invoke<boolean>("delete_book", {
            id: book.object_id,
        });
        console.log(
            `Delete ${book.object_id} ${result ? "deleted" : "not deleted"}`,
        );
        showDeleteModal = false;
        if (onUpdate) onUpdate();
    }
</script>

<tr>
    <td class="px-4 py-2 border">{book.object_id.trim()}</td>
    <td class="px-4 py-2 border">{book.object_name.trim()}</td>
    <td class="px-4 py-2 border">{book.publication_year}</td>
    <td class="px-4 py-2 border">{book.identifier_id.trim()}</td>
    <td class="px-4 py-2 border">{book.language_id.trim()}</td>
    <td class="px-4 py-2 border">{book.page_count}</td>
    <td class="px-4 py-2 border">
        <button
            class="bg-blue-500 text-white px-2 py-1 rounded mr-2"
            onclick={() => {
                object_name = book.object_name.trim();
                publication_year = book.publication_year;
                identifier_id = book.identifier_id.trim();
                language_id = book.language_id.trim();
                page_count = `${book.page_count}`;
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
                    <h3 class="text-lg font-bold mb-4">Editar libro</h3>
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
                            Páginas:
                            <input
                                type="number"
                                class="border p-1 w-full"
                                bind:value={page_count}
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
                    <h3 class="text-lg font-bold mb-4">¿Eliminar libro?</h3>
                    <p>¿Estás seguro de que deseas eliminar este libro?</p>
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
