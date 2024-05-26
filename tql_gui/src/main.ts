import { invoke } from "@tauri-apps/api/tauri";

const newinstInputE = document.querySelector("#newinst-input") as HTMLInputElement;

const versions: string = await invoke("fetch_versions", {});

async function createInstance() {
    // https://tauri.app/v1/guides/features/command
    if (newinstInputE?.value == "") return;

    await invoke("create_instance", {});
}

document.querySelector("#newinst-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    createInstance();
});
