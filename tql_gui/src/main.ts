import { invoke } from "@tauri-apps/api/tauri";

const greetInputEl = document.querySelector("#greet-input");
const greetMsgEl = document.querySelector("#greet-msg");

const versions: string = await invoke("fetch_versions", {});

async function greet() {
    if (greetMsgEl && greetInputEl) {
        // https://tauri.app/v1/guides/features/command
        greetMsgEl.textContent = versions;
    }
}

document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
});
