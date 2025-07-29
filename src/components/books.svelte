<script lang="ts">
    import type { Book as BookType } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import { Chart } from "frappe-charts";
    import Book from "./book.svelte";

    let books = $state<BookType[]>([]);
    let page = $state<number>(0);
    let showCreateModal = $state<boolean>(false);
    let formName = $state<string>("");

    let data = $derived({
        labels: books.map((book) => book.object_name),
        datasets: [
            {
                name: "Número de Páginas",
                values: books.map((book) => book.page_count),
            },
        ],
    });
    let chartRef = $state();
    let showChartModal = $state<boolean>(false);

    // Variables para el formulario de creación
    let object_id = $state<string>("");
    let object_name = $state<string>("");
    let publication_year = $state<string>("");
    let identifier_id = $state<string>("");
    let language_id = $state<string>("");
    let page_count = $state<string>("");

    $effect(() => {
        if (showChartModal && books.length !== 0 && chartRef) {
            new Chart(chartRef, {
                data,
                title: "Paginas de Libro",
                type: "bar",
                height: 500,
                animate: true,
            });
        }
    });

    function searchAll(page: number) {
        invoke<BookType[]>("get_books", { page }).then((value) => {
            books = value;
        });
    }

    function process(page: number) {
        if (formName.trim() === "") {
            searchAll(page);
        } else {
            searchBooks(page);
        }
    }

    function searchBooks(page: number) {
        invoke<BookType[]>("get_book_by_name", {
            page,
            name: formName,
        }).then((value) => {
            books = value;
        });
    }

    async function handleCreateSubmit(e: Event) {
        e.preventDefault();
        const new_book = {
            object_id,
            object_name,
            publication_year,
            identifier_id,
            language_id,
            page_count,
        };
        let result = await invoke<boolean>("create_book", {
            new_book,
        });
        if (result) {
            showCreateModal = false;
            // Limpiar campos
            object_id = "";
            object_name = "";
            publication_year = "";
            identifier_id = "";
            language_id = "";
            page_count = "";
            process(page);
        }
    }

    $effect(() => {
        process(page);
    });

    $effect(() => {
        if (formName.trim() !== "") {
            page = 0;
        }
    });
</script>

<div class="flex items-center gap-4 mb-4">
    <input
        type="text"
        class="border p-2 rounded w-full max-w-xs"
        placeholder="Buscar por nombre..."
        bind:value={formName}
        onkeydown={(e) => {
            if (e.key === "Enter") process(page);
        }}
    />
    <button
        class="px-4 py-2 bg-blue-500 text-white rounded"
        onclick={() => {
            process(page);
        }}>Buscar</button
    >
    <button
        class="px-4 py-2 bg-purple-500 text-white rounded"
        onclick={() => (showChartModal = true)}
    >
        Ver gráfico
    </button>
    {#if showChartModal}
        <div
            class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
        >
            <div class="bg-white p-6 rounded shadow-lg min-w-[1000px] relative">
                <button
                    class="absolute top-2 right-2 text-gray-500 hover:text-black"
                    onclick={() => (showChartModal = false)}>&#10005;</button
                >
                <h3 class="text-lg font-bold mb-4">Gráfico de libros</h3>
                <div bind:this={chartRef} id="chart"></div>
            </div>
        </div>
    {/if}
    <button
        class="px-4 py-2 bg-green-500 text-white rounded"
        onclick={() => (showCreateModal = true)}>Crear libro</button
    >
</div>

{#if showCreateModal}
    <div
        class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
    >
        <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
            <h3 class="text-lg font-bold mb-4">Crear libro</h3>
            <form onsubmit={handleCreateSubmit} class="flex flex-col gap-2">
                <label>
                    ID:
                    <input
                        type="text"
                        class="border p-1 w-full"
                        bind:value={object_id}
                    />
                </label>
                <label>
                    Título:
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
            <th class="px-4 py-2 border">Título</th>
            <th class="px-4 py-2 border">Año</th>
            <th class="px-4 py-2 border">Identificador</th>
            <th class="px-4 py-2 border">Idioma</th>
            <th class="px-4 py-2 border">Número de paginas</th>
            <th class="px-4 py-2 border">Actions</th>
        </tr>
    </thead>
    <tbody>
        {#each books as book}
            <Book
                {book}
                onUpdate={() => {
                    process(page);
                }}
            />
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
