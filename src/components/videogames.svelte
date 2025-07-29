<script lang="ts">
    import type { Videogame as VideogameType } from "$lib/types";
    import { invoke } from "@tauri-apps/api/core";
    import Videogame from "./videogame.svelte";
    import { Chart } from "frappe-charts";
    let chartRef = $state();
    let showChartModal = $state<boolean>(false);

    let videogames = $state<VideogameType[]>([]);
    let studios = $state<[number, string][]>([]);
    let page = $state<number>(0);
    let showCreateModal = $state<boolean>(false);
    let formName = $state<string>("");
    let data = $derived({
        labels: studios.map((studio) => studio[1]),
        datasets: [
            {
                name: "Duración (min)",
                values: studios.map((studio) => studio[0]),
            },
        ],
    });

    // Variables para el formulario de creación
    let object_id = $state<string>("");
    let object_name = $state<string>("");
    let publication_year = $state<string>("");
    let identifier_id = $state<string>("");
    let language_id = $state<string>("");
    let controller_support = $state<boolean>(false);
    let pegi_id = $state<string>("");

    function searchAll(page: number) {
        invoke<VideogameType[]>("get_videogames", { page }).then((value) => {
            videogames = value;
        });
        invoke<[number, string][]>("get_videogames_and_studios", { page }).then(
            (value) => {
                studios = value;
            },
        );
    }

    function process(page: number) {
        if (formName.trim() === "") {
            searchAll(page);
        } else {
            searchVideogames(page);
        }
    }

    function searchVideogames(page: number) {
        invoke<VideogameType[]>("get_videogame_by_name", {
            page,
            name: formName,
        }).then((value) => {
            videogames = value;
        });
        invoke<[number, string][]>("get_videogames_and_studios_by_name", {
            page,
            name: formName,
        }).then((value) => {
            studios = value;
        });
    }

    async function handleCreateSubmit(e: Event) {
        e.preventDefault();
        const new_videogame = {
            object_id,
            object_name,
            publication_year,
            identifier_id,
            language_id,
            controller_support,
            pegi_id,
        };
        let result = await invoke<boolean>("create_videogame", {
            new_videogame,
        });
        if (result) {
            showCreateModal = false;
            // Limpiar campos
            object_id = "";
            object_name = "";
            publication_year = "";
            identifier_id = "";
            language_id = "";
            controller_support = false;
            pegi_id = "";
            process(page);
        }
    }

    $effect(() => {
        process(page);
    });

    $effect(() => {
        if (showChartModal && studios.length !== 0 && chartRef) {
            new Chart(chartRef, {
                data,
                title: "Estudios por Videojuego",
                type: "bar",
                height: 500,
                animate: true,
            });
        }
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
                <h3 class="text-lg font-bold mb-4">
                    Gráfico de Estudios por Videojuego
                </h3>
                <div bind:this={chartRef} id="chart"></div>
            </div>
        </div>
    {/if}
    <button
        class="px-4 py-2 bg-green-500 text-white rounded"
        onclick={() => (showCreateModal = true)}>Crear videojuego</button
    >
</div>

{#if showCreateModal}
    <div
        class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
    >
        <div class="bg-white p-6 rounded shadow-lg min-w-[300px]">
            <h3 class="text-lg font-bold mb-4">Crear videojuego</h3>
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
                    <input type="checkbox" bind:checked={controller_support} />
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
            <th class="px-4 py-2 border">Soporta Control</th>
            <th class="px-4 py-2 border">PEGI</th>
            <th class="px-4 py-2 border">Actions</th>
        </tr>
    </thead>
    <tbody>
        {#each videogames as videogame}
            <Videogame
                {videogame}
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
